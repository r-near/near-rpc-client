/**
 * Generate Zod schemas from OpenRPC specification using openapi-zod-client
 * 
 * This produces much cleaner output than json-schema-to-zod:
 * - Uses z.union() instead of superRefine for oneOf
 * - Produces intersection types (.and()) for allOf
 * - Much smaller output (2300 lines vs 13000+)
 * 
 * Usage: bun run scripts/generate.ts
 */

import { generateZodClientFromOpenAPI } from "openapi-zod-client";
import * as fs from "fs";
import * as path from "path";

const __dirname = import.meta.dir;

// Read the OpenRPC spec - use the shared version (already collapsed by Rust)
const openrpcPath = path.join(__dirname, "../../shared/openrpc.json");
const openrpc = JSON.parse(fs.readFileSync(openrpcPath, "utf-8"));

// Preprocess schemas to convert `const` to single-value `enum`
// and add `additionalProperties: false` to avoid `.passthrough()` which creates ugly types
function preprocessSchema(schema: any): any {
  if (schema === null || typeof schema !== "object") {
    return schema;
  }

  if (Array.isArray(schema)) {
    return schema.map(preprocessSchema);
  }

  const result: any = {};
  for (const [key, value] of Object.entries(schema)) {
    if (key === "const") {
      // Convert const to single-value enum
      result["enum"] = [value];
    } else {
      result[key] = preprocessSchema(value);
    }
  }
  
  // Add additionalProperties: false to object types to avoid .passthrough()
  if (result.type === "object" && result.properties && !("additionalProperties" in result)) {
    result.additionalProperties = false;
  }
  
  return result;
}

const processedSchemas = preprocessSchema(openrpc.components.schemas);

// Convert OpenRPC to a minimal OpenAPI structure
// openapi-zod-client only needs components.schemas, which is identical
const fakeOpenApi = {
  openapi: "3.0.0",
  info: openrpc.info,
  paths: {}, // Empty - we just want schemas
  components: {
    schemas: processedSchemas,
  },
};

/**
 * Post-process the generated output to:
 * 1. Add type aliases for cleaner hover types
 * 2. Remove .passthrough() which causes ugly z.objectOutputType in hovers
 */
function postProcessOutput(content: string, schemaNames: string[]): string {
  // Remove .passthrough() - it causes ugly hover types (see https://github.com/colinhacks/zod/issues/2938)
  // For objects with additionalProperties: false, we don't need passthrough anyway
  content = content.replace(/\.passthrough\(\)/g, "");
  
  // Find the schemas export object
  const exportMatch = content.match(/export const schemas = \{[\s\S]*?\n\};/);
  if (!exportMatch) {
    console.warn("Could not find schemas export, skipping type aliases");
    return content;
  }

  // Add a Simplify utility type that forces TypeScript to expand types for cleaner hover display
  const simplifyUtil = `
// Utility type to force TypeScript to expand types for cleaner hover display
type Simplify<T> = { [K in keyof T]: T[K] } & {};
`;

  // Generate type aliases using Simplify for each schema
  const typeAliases = schemaNames
    .map((name) => `export type ${name} = Simplify<z.infer<typeof ${name}>>;`)
    .join("\n");

  // Insert after the import statement
  const importEnd = content.indexOf('import { z } from "zod";') + 'import { z } from "zod";'.length;
  const before = content.slice(0, importEnd);
  const middle = content.slice(importEnd, exportMatch.index!);
  const after = content.slice(exportMatch.index!);
  
  // Replace "export const schemas = {" with an explicit Record type annotation
  // This prevents TS7056 error about type too long to serialize
  const fixedAfter = after.replace(
    'export const schemas = {',
    'export const schemas: Record<string, z.ZodTypeAny> = {'
  );

  return before + simplifyUtil + middle + "\n// Inferred types for cleaner IDE hover\n" + typeAliases + "\n\n" + fixedAfter;
}

async function main() {
  const templatePath = path.join(
    __dirname,
    "../node_modules/openapi-zod-client/dist/templates/schemas-only.hbs"
  );
  
  // Check if template exists, try alternate path
  const altTemplatePath = path.join(
    __dirname,
    "../node_modules/openapi-zod-client/src/templates/schemas-only.hbs"
  );
  
  const finalTemplatePath = fs.existsSync(templatePath) ? templatePath : altTemplatePath;
  
  try {
    await generateZodClientFromOpenAPI({
      openApiDoc: fakeOpenApi as any,
      distPath: path.join(__dirname, "../src/schemas.ts"),
      templatePath: finalTemplatePath,
      options: {
        shouldExportAllSchemas: true,
        withDescription: false,
        withDefaultValues: false,
      },
    });

    // Post-process to add type aliases and remove passthrough
    const outputPath = path.join(__dirname, "../src/schemas.ts");
    let content = fs.readFileSync(outputPath, "utf-8");

    // Extract schema names from the export
    const schemaNames = Object.keys(processedSchemas);
    content = postProcessOutput(content, schemaNames);

    fs.writeFileSync(outputPath, content);

    console.log(`Input: ${openrpcPath}`);
    console.log(`Output: ${outputPath}`);
    console.log(`Generated ${schemaNames.length} schemas with type aliases`);
  } catch (err) {
    console.error("Error:", err);
    process.exit(1);
  }
}

main();
