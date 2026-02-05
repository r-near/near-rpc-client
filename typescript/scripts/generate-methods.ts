/**
 * Generate RPC method types from OpenRPC spec.
 * 
 * This creates a type-safe interface for all NEAR RPC methods,
 * mapping each method to its request and response schemas.
 */

import * as fs from "fs";
import * as path from "path";

const __dirname = import.meta.dir;

interface OpenRpcParam {
  name: string;
  schema: { $ref?: string; type?: string } | any;
  required?: boolean;
}

interface OpenRpcMethod {
  name: string;
  params?: OpenRpcParam[];
  result?: {
    name?: string;
    schema: { $ref?: string } | any;
  };
}

function extractSchemaName(schema: any): string | null {
  if (!schema) return null;
  if (schema.$ref) {
    // "#/components/schemas/RpcBlockRequest" -> "RpcBlockRequest"
    return schema.$ref.split("/").pop() || null;
  }
  return null;
}

function toMethodName(rpcMethodName: string): string {
  // Convert snake_case to camelCase
  return rpcMethodName.replace(/_([a-z])/g, (_, c) => c.toUpperCase());
}

function generateMethodTypes(openrpcPath: string): string {
  const openrpc = JSON.parse(fs.readFileSync(openrpcPath, "utf-8"));
  const methods: OpenRpcMethod[] = openrpc.methods || [];

  const lines: string[] = [
    "// Auto-generated RPC method types - DO NOT EDIT",
    "",
    "// Import type aliases (not schemas) for clean hover types",
    'import type {',
  ];

  // Collect all schema names used
  const usedSchemas = new Set<string>();
  for (const method of methods) {
    const params = method.params || [];
    if (params.length > 0) {
      const paramSchema = extractSchemaName(params[0].schema);
      if (paramSchema) usedSchemas.add(paramSchema);
    }
    const resultSchema = extractSchemaName(method.result?.schema);
    if (resultSchema) usedSchemas.add(resultSchema);
  }

  // Import the type aliases
  lines.push(`  ${Array.from(usedSchemas).sort().join(",\n  ")}`);
  lines.push('} from "./schemas.js";');
  lines.push("");
  
  // Also import schemas for runtime validation
  lines.push('import { schemas } from "./schemas.js";');
  lines.push('import { z } from "zod";');
  lines.push("");

  // Generate method map type using the type aliases directly
  lines.push("/** Map of RPC method names to their request/response types */");
  lines.push("export interface RpcMethodMap {");

  for (const method of methods) {
    const params = method.params || [];
    const result = method.result?.schema;

    // Get request type (use the type alias directly)
    let requestType = "void";
    if (params.length > 0) {
      const paramSchema = extractSchemaName(params[0].schema);
      if (paramSchema) {
        requestType = paramSchema; // Use type alias directly!
      }
    }

    // Get response type (use the type alias directly)
    let responseType = "unknown";
    const resultSchema = extractSchemaName(result);
    if (resultSchema) {
      responseType = resultSchema; // Use type alias directly!
    }

    lines.push(`  "${method.name}": {`);
    lines.push(`    request: ${requestType};`);
    lines.push(`    response: ${responseType};`);
    lines.push(`  };`);
  }

  lines.push("}");
  lines.push("");

  // Generate method schemas map (for runtime validation)
  // Use explicit type to prevent TS7056 "type too long to serialize" error
  lines.push("/** Runtime schemas for request/response validation */");
  lines.push("export const rpcMethodSchemas: Record<string, { request: z.ZodTypeAny; response: z.ZodTypeAny }> = {");

  for (const method of methods) {
    const params = method.params || [];
    const result = method.result?.schema;

    const paramSchema = params.length > 0 ? extractSchemaName(params[0].schema) : null;
    const resultSchema = extractSchemaName(result);

    lines.push(`  "${method.name}": {`);
    lines.push(`    request: ${paramSchema ? `schemas.${paramSchema}` : "z.void()"},`);
    lines.push(`    response: ${resultSchema ? `schemas.${resultSchema}` : "z.unknown()"},`);
    lines.push(`  },`);
  }

  lines.push("};");
  lines.push("");

  // Generate typed RPC client interface
  lines.push("/** Type-safe RPC client interface */");
  lines.push("export interface TypedRpcClient {");

  for (const method of methods) {
    const camelName = toMethodName(method.name);
    lines.push(`  ${camelName}(request: RpcMethodMap["${method.name}"]["request"]): Promise<RpcMethodMap["${method.name}"]["response"]>;`);
  }

  lines.push("}");
  lines.push("");

  // Generate method name union type
  lines.push("/** Union of all RPC method names */");
  lines.push("export type RpcMethodName = keyof RpcMethodMap;");
  lines.push("");

  // Generate a helper to get request/response types
  lines.push("/** Helper types for generic RPC handling */");
  lines.push("export type RpcRequest<M extends RpcMethodName> = RpcMethodMap[M][\"request\"];");
  lines.push("export type RpcResponse<M extends RpcMethodName> = RpcMethodMap[M][\"response\"];");

  return lines.join("\n");
}

// Main
const inputPath = path.join(__dirname, "../../shared/openrpc.json");
const outputPath = path.join(__dirname, "../src/methods.ts");

console.log(`Input: ${inputPath}`);
console.log(`Output: ${outputPath}`);

const content = generateMethodTypes(inputPath);
fs.writeFileSync(outputPath, content);

console.log(`Generated ${outputPath}`);
