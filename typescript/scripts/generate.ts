/**
 * Generate Zod schemas from OpenRPC specification
 * 
 * Usage: bun run scripts/generate.ts
 */

import { jsonSchemaToZod } from "json-schema-to-zod";
import * as fs from "fs";
import * as path from "path";

const openrpcPath = path.join(import.meta.dir, "../../shared/openrpc.json");
const openrpc = JSON.parse(fs.readFileSync(openrpcPath, "utf-8"));

const schemas = openrpc.components?.schemas || {};

// Build definitions map for $ref resolution
const definitions: Record<string, unknown> = {};
for (const [name, schema] of Object.entries(schemas)) {
  definitions[name] = schema;
}

/**
 * Recursively resolve $ref pointers in JSON Schema.
 * Uses cycle detection to prevent infinite recursion.
 */
function resolveRefs(obj: unknown, seen = new Set<string>(), depth = 0): unknown {
  if (depth > 20) return {};
  if (obj === null || typeof obj !== "object") return obj;
  if (Array.isArray(obj)) return obj.map((item) => resolveRefs(item, seen, depth));

  const record = obj as Record<string, unknown>;

  // Handle $ref
  if (record.$ref && typeof record.$ref === "string") {
    const refPath = record.$ref.replace("#/components/schemas/", "");

    // Cycle detection
    if (seen.has(refPath)) {
      return { $lazyRef: refPath };
    }

    const referenced = definitions[refPath];
    if (referenced) {
      const newSeen = new Set(seen);
      newSeen.add(refPath);
      return resolveRefs(referenced, newSeen, depth + 1);
    }
    return {};
  }

  const result: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(record)) {
    result[key] = resolveRefs(value, seen, depth);
  }
  return result;
}

const schemaNames = Object.keys(definitions);

let output = `// Auto-generated from OpenRPC schema - DO NOT EDIT
import { z } from "zod";

`;

// Generate each schema
for (const name of schemaNames) {
  const schema = definitions[name];
  const resolved = resolveRefs(schema, new Set([name]));

  // Replace lazy refs with empty object (recursive types become z.any())
  const cleanedStr = JSON.stringify(resolved).replace(
    /\{"\$lazyRef":"[^"]+"\}/g,
    "{}"
  );
  const cleaned = JSON.parse(cleanedStr);

  try {
    const zodCode = jsonSchemaToZod(cleaned, {
      name: name,
      module: "esm",
      type: true,
      noImport: true,
    });

    output += zodCode + "\n\n";
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    console.error(`Failed to generate ${name}: ${message}`);
    output += `export const ${name} = z.unknown();\n`;
    output += `export type ${name} = z.infer<typeof ${name}>;\n\n`;
  }
}

const outPath = path.join(import.meta.dir, "../src/schemas.ts");
fs.writeFileSync(outPath, output);
console.log(`Generated ${schemaNames.length} schemas to ${outPath}`);
