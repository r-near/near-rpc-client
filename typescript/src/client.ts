/**
 * Type-safe NEAR RPC Client
 * 
 * Generated from OpenRPC specification with runtime Zod validation.
 */

import { z } from "zod";
import { schemas } from "./schemas.js";
import { RpcMethodMap, rpcMethodSchemas, RpcMethodName } from "./methods.js";

// Import type aliases for clean method signatures
import type {
  RpcBlockRequest,
  RpcBlockResponse,
  RpcChunkRequest,
  RpcChunkResponse,
  RpcQueryRequest,
  RpcQueryResponse,
  RpcStatusResponse,
  RpcSendTransactionRequest,
  RpcTransactionResponse,
  RpcTransactionStatusRequest,
  RpcValidatorRequest,
  RpcValidatorResponse,
  RpcNetworkInfoResponse,
  RpcHealthResponse,
  RpcGasPriceRequest,
  RpcGasPriceResponse,
  RpcProtocolConfigRequest,
  RpcProtocolConfigResponse,
  RpcStateChangesInBlockByTypeRequest,
  RpcStateChangesInBlockResponse,
  RpcStateChangesInBlockRequest,
  RpcStateChangesInBlockByTypeResponse,
  RpcReceiptRequest,
  RpcReceiptResponse,
  CryptoHash,
} from "./schemas.js";

/** JSON-RPC response structure */
interface JsonRpcResponse {
  jsonrpc: "2.0";
  id: number;
  result?: unknown;
  error?: {
    code: number;
    message: string;
    data?: unknown;
  };
}

/** Options for creating a NEAR RPC client */
export interface NearRpcClientOptions {
  /** RPC endpoint URL */
  url: string;
  /** Optional headers to include in requests */
  headers?: Record<string, string>;
  /** Optional fetch implementation (defaults to global fetch) */
  fetch?: typeof fetch;
  /** Request timeout in ms (default: 30000) */
  timeout?: number;
  /** Whether to validate responses with Zod (default: true) */
  validateResponses?: boolean;
}

/** RPC Error returned by NEAR nodes */
export class NearRpcError extends Error {
  constructor(
    public code: number,
    message: string,
    public data?: unknown
  ) {
    super(message);
    this.name = "NearRpcError";
  }
}

/** Validation error when response doesn't match schema */
export class ValidationError extends Error {
  constructor(
    public method: string,
    public issues: z.ZodIssue[]
  ) {
    super(`Response validation failed for ${method}: ${JSON.stringify(issues)}`);
    this.name = "ValidationError";
  }
}

/**
 * NEAR Protocol JSON-RPC Client
 * 
 * Provides type-safe access to NEAR RPC methods with runtime validation.
 */
export class NearRpcClient {
  private url: string;
  private headers: Record<string, string>;
  private fetchFn: typeof fetch;
  private timeout: number;
  private validateResponses: boolean;
  private requestId = 0;

  constructor(options: NearRpcClientOptions) {
    this.url = options.url;
    this.headers = {
      "Content-Type": "application/json",
      ...options.headers,
    };
    this.fetchFn = options.fetch ?? globalThis.fetch;
    this.timeout = options.timeout ?? 30000;
    this.validateResponses = options.validateResponses ?? true;
  }

  /** Create a client for NEAR mainnet */
  static mainnet(options?: Partial<NearRpcClientOptions>): NearRpcClient {
    return new NearRpcClient({
      url: "https://rpc.mainnet.near.org",
      ...options,
    });
  }

  /** Create a client for NEAR testnet */
  static testnet(options?: Partial<NearRpcClientOptions>): NearRpcClient {
    return new NearRpcClient({
      url: "https://rpc.testnet.near.org",
      ...options,
    });
  }

  /**
   * Generic RPC call with full type safety
   */
  async call<M extends RpcMethodName>(
    method: M,
    params: RpcMethodMap[M]["request"]
  ): Promise<RpcMethodMap[M]["response"]> {
    const id = ++this.requestId;
    
    const body = JSON.stringify({
      jsonrpc: "2.0",
      id,
      method,
      params,
    });

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);

    try {
      const response = await this.fetchFn(this.url, {
        method: "POST",
        headers: this.headers,
        body,
        signal: controller.signal,
      });

      if (!response.ok) {
        throw new Error(`HTTP error: ${response.status} ${response.statusText}`);
      }

      const json = await response.json() as JsonRpcResponse;

      if (json.error) {
        throw new NearRpcError(
          json.error.code,
          json.error.message,
          json.error.data
        );
      }

      const result = json.result;

      // Validate response if enabled
      if (this.validateResponses) {
        const methodSchema = rpcMethodSchemas[method];
        const parseResult = methodSchema.response.safeParse(result);
        if (!parseResult.success) {
          throw new ValidationError(method, parseResult.error.issues);
        }
        return parseResult.data as RpcMethodMap[M]["response"];
      }

      return result as RpcMethodMap[M]["response"];
    } finally {
      clearTimeout(timeoutId);
    }
  }

  // ========== Status Methods ==========

  /** Get node status including sync info and validators */
  async status(): Promise<RpcStatusResponse> {
    return this.call("status", {});
  }

  /** Health check - returns empty object if healthy, throws if not */
  async health(): Promise<RpcHealthResponse> {
    return this.call("health", {});
  }

  /** Get network info including connected peers */
  async networkInfo(): Promise<RpcNetworkInfoResponse> {
    return this.call("network_info", {});
  }

  // ========== Block Methods ==========

  /** Get a block by height, hash, or finality */
  async block(request: RpcBlockRequest): Promise<RpcBlockResponse> {
    return this.call("block", request);
  }

  /** Get a chunk by chunk hash or block + shard ID */
  async chunk(request: RpcChunkRequest): Promise<RpcChunkResponse> {
    return this.call("chunk", request);
  }

  // ========== Query Methods ==========

  /** Query the state (view account, view code, call function, etc.) */
  async query(request: RpcQueryRequest): Promise<RpcQueryResponse> {
    return this.call("query", request);
  }

  // ========== Gas Methods ==========

  /** Get current gas price */
  async gasPrice(request: RpcGasPriceRequest): Promise<RpcGasPriceResponse> {
    return this.call("gas_price", request);
  }

  // ========== Transaction Methods ==========

  /** Send a transaction asynchronously (returns immediately with tx hash) */
  async broadcastTxAsync(request: RpcSendTransactionRequest): Promise<CryptoHash> {
    return this.call("broadcast_tx_async", request);
  }

  /** Send a transaction and wait for it to complete */
  async broadcastTxCommit(request: RpcSendTransactionRequest): Promise<RpcTransactionResponse> {
    return this.call("broadcast_tx_commit", request);
  }

  /** Get transaction status by hash */
  async tx(request: RpcTransactionStatusRequest): Promise<RpcTransactionResponse> {
    return this.call("tx", request);
  }

  /** Get a receipt by ID */
  async receipt(request: RpcReceiptRequest): Promise<RpcReceiptResponse> {
    return this.call("EXPERIMENTAL_receipt", request);
  }

  // ========== Validator Methods ==========

  /** Get validators for an epoch */
  async validators(request: RpcValidatorRequest): Promise<RpcValidatorResponse> {
    return this.call("validators", request);
  }

  // ========== State Change Methods ==========

  /** Get changes in a block */
  async changesInBlock(request: RpcStateChangesInBlockRequest): Promise<RpcStateChangesInBlockByTypeResponse> {
    return this.call("EXPERIMENTAL_changes_in_block", request);
  }

  /** Get state changes */
  async changes(request: RpcStateChangesInBlockByTypeRequest): Promise<RpcStateChangesInBlockResponse> {
    return this.call("EXPERIMENTAL_changes", request);
  }

  // ========== Protocol Config Methods ==========

  /** Get protocol configuration */
  async protocolConfig(request: RpcProtocolConfigRequest): Promise<RpcProtocolConfigResponse> {
    return this.call("EXPERIMENTAL_protocol_config", request);
  }
}

// Re-export types for convenience
export { schemas };
export type { RpcMethodMap, RpcMethodName };
export { rpcMethodSchemas };

// Re-export commonly used types
export type {
  RpcBlockRequest,
  RpcBlockResponse,
  RpcQueryRequest,
  RpcQueryResponse,
  RpcTransactionResponse,
  RpcStatusResponse,
  BlockReference,
  QueryRequest,
  AccountView,
  BlockHeaderView,
  ChunkHeaderView,
} from "./schemas.js";
