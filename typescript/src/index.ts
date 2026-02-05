// NEAR RPC Client for TypeScript
// Type-safe access to NEAR Protocol JSON-RPC with Zod validation

export { NearRpcClient, NearRpcError, ValidationError } from "./client.js";
export type { NearRpcClientOptions } from "./client.js";

// Re-export all schemas and types
export * from "./schemas.js";

// Re-export method types
export type { RpcMethodMap, RpcMethodName, RpcRequest, RpcResponse, TypedRpcClient } from "./methods.js";
export { rpcMethodSchemas } from "./methods.js";
