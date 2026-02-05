// NEAR RPC Client for TypeScript
// Auto-generated types with runtime validation via Zod

import {
  RpcStatusResponse,
  RpcBlockRequest,
  RpcBlockResponse,
  RpcQueryRequest,
  RpcQueryResponse,
  RpcGasPriceRequest,
  RpcGasPriceResponse,
  RpcValidatorRequest,
  RpcValidatorResponse,
  RpcNetworkInfoResponse,
  RpcTransactionStatusRequest,
  RpcTransactionResponse,
  RpcReceiptRequest,
  RpcReceiptResponse,
  RpcStateChangesInBlockRequest,
  RpcStateChangesInBlockResponse,
  RpcStateChangesInBlockByTypeRequest,
  RpcStateChangesInBlockByTypeResponse,
  RpcProtocolConfigRequest,
  RpcProtocolConfigResponse,
  RpcLightClientExecutionProofRequest,
  RpcLightClientExecutionProofResponse,
  RpcLightClientNextBlockRequest,
  RpcLightClientNextBlockResponse,
  RpcChunkRequest,
  RpcChunkResponse,
  RpcSendTransactionRequest,
  RpcCongestionLevelRequest,
  RpcCongestionLevelResponse,
} from "./schemas.js";

// Re-export types for convenience
export * from "./schemas.js";

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

/** Options for creating a NEAR RPC client */
export interface NearRpcClientOptions {
  /** RPC endpoint URL */
  url: string;
  /** Optional headers to include in requests */
  headers?: Record<string, string>;
  /** Optional fetch implementation (defaults to global fetch) */
  fetch?: typeof fetch;
}

/** JSON-RPC response structure */
interface JsonRpcResponse<T = unknown> {
  jsonrpc: "2.0";
  id: number;
  result?: T;
  error?: {
    code: number;
    message: string;
    data?: unknown;
  };
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
  private requestId = 0;

  constructor(options: NearRpcClientOptions) {
    this.url = options.url;
    this.headers = {
      "Content-Type": "application/json",
      ...options.headers,
    };
    this.fetchFn = options.fetch ?? globalThis.fetch;
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

  /** Make a raw JSON-RPC call */
  private async call<T>(method: string, params: unknown): Promise<T> {
    const id = ++this.requestId;
    const body = JSON.stringify({
      jsonrpc: "2.0",
      id,
      method,
      params,
    });

    const response = await this.fetchFn(this.url, {
      method: "POST",
      headers: this.headers,
      body,
    });

    if (!response.ok) {
      throw new Error(`HTTP error: ${response.status} ${response.statusText}`);
    }

    const json = (await response.json()) as JsonRpcResponse;

    if (json.error) {
      throw new NearRpcError(
        json.error.code,
        json.error.message,
        json.error.data
      );
    }

    return json.result as T;
  }

  // ========== Status Methods ==========

  /** Get node status including sync info and validators */
  async status(): Promise<RpcStatusResponse> {
    const result = await this.call<unknown>("status", []);
    return RpcStatusResponse.parse(result);
  }

  /** Health check - returns empty object if healthy, throws if not */
  async health(): Promise<Record<string, never>> {
    return this.call("health", []);
  }

  /** Get network info including connected peers */
  async networkInfo(): Promise<RpcNetworkInfoResponse> {
    const result = await this.call<unknown>("network_info", []);
    return RpcNetworkInfoResponse.parse(result);
  }

  // ========== Block Methods ==========

  /** Get a block by height, hash, or finality */
  async block(request: RpcBlockRequest): Promise<RpcBlockResponse> {
    const result = await this.call<unknown>("block", request);
    return RpcBlockResponse.parse(result);
  }

  /** Get a chunk by chunk hash or block + shard ID */
  async chunk(request: RpcChunkRequest): Promise<RpcChunkResponse> {
    const result = await this.call<unknown>("chunk", request);
    return RpcChunkResponse.parse(result);
  }

  // ========== Query Methods ==========

  /** Query the state (view account, view code, call function, etc.) */
  async query(request: RpcQueryRequest): Promise<RpcQueryResponse> {
    const result = await this.call<unknown>("query", request);
    return RpcQueryResponse.parse(result);
  }

  // ========== Gas Methods ==========

  /** Get current gas price */
  async gasPrice(request: RpcGasPriceRequest): Promise<RpcGasPriceResponse> {
    const result = await this.call<unknown>("gas_price", request);
    return RpcGasPriceResponse.parse(result);
  }

  /** Get congestion level for an account */
  async congestionLevel(request: RpcCongestionLevelRequest): Promise<RpcCongestionLevelResponse> {
    const result = await this.call<unknown>("congestion_level", request);
    return RpcCongestionLevelResponse.parse(result);
  }

  // ========== Transaction Methods ==========

  /** Send a transaction asynchronously (returns immediately with tx hash) */
  async broadcastTxAsync(request: RpcSendTransactionRequest): Promise<string> {
    return this.call("broadcast_tx_async", request);
  }

  /** Send a transaction and wait for it to complete */
  async broadcastTxCommit(request: RpcSendTransactionRequest): Promise<RpcTransactionResponse> {
    const result = await this.call<unknown>("broadcast_tx_commit", request);
    return RpcTransactionResponse.parse(result);
  }

  /** Get transaction status by hash */
  async tx(request: RpcTransactionStatusRequest): Promise<RpcTransactionResponse> {
    const result = await this.call<unknown>("tx", request);
    return RpcTransactionResponse.parse(result);
  }

  /** Get a receipt by ID */
  async receipt(request: RpcReceiptRequest): Promise<RpcReceiptResponse> {
    const result = await this.call<unknown>("EXPERIMENTAL_receipt", request);
    return RpcReceiptResponse.parse(result);
  }

  // ========== Validator Methods ==========

  /** Get validators for an epoch */
  async validators(request: RpcValidatorRequest): Promise<RpcValidatorResponse> {
    const result = await this.call<unknown>("validators", request);
    return RpcValidatorResponse.parse(result);
  }

  // ========== State Change Methods ==========

  /** Get changes in a block */
  async changesInBlock(request: RpcStateChangesInBlockRequest): Promise<RpcStateChangesInBlockResponse> {
    const result = await this.call<unknown>("EXPERIMENTAL_changes_in_block", request);
    return RpcStateChangesInBlockResponse.parse(result);
  }

  /** Get state changes */
  async changes(request: RpcStateChangesInBlockByTypeRequest): Promise<RpcStateChangesInBlockByTypeResponse> {
    const result = await this.call<unknown>("EXPERIMENTAL_changes", request);
    return RpcStateChangesInBlockByTypeResponse.parse(result);
  }

  // ========== Protocol Config Methods ==========

  /** Get protocol configuration */
  async protocolConfig(request: RpcProtocolConfigRequest): Promise<RpcProtocolConfigResponse> {
    const result = await this.call<unknown>("EXPERIMENTAL_protocol_config", request);
    return RpcProtocolConfigResponse.parse(result);
  }

  // ========== Light Client Methods ==========

  /** Get a light client proof */
  async lightClientProof(request: RpcLightClientExecutionProofRequest): Promise<RpcLightClientExecutionProofResponse> {
    const result = await this.call<unknown>("EXPERIMENTAL_light_client_proof", request);
    return RpcLightClientExecutionProofResponse.parse(result);
  }

  /** Get the next light client block */
  async nextLightClientBlock(request: RpcLightClientNextBlockRequest): Promise<RpcLightClientNextBlockResponse> {
    const result = await this.call<unknown>("next_light_client_block", request);
    return RpcLightClientNextBlockResponse.parse(result);
  }
}
