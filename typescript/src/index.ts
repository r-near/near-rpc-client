// NEAR RPC Client for TypeScript
// Type-safe access to NEAR Protocol JSON-RPC with Zod validation

export { NearRpcClient, NearRpcError } from "./client.js";
export type { NearRpcClientOptions } from "./client.js";

// Re-export all schemas and types
export * from "./schemas.js";
