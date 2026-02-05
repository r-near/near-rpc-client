// Auto-generated RPC method types - DO NOT EDIT

// Import type aliases (not schemas) for clean hover types
import type {
  BlockHeightRanges,
  CryptoHash,
  GenesisConfig,
  GenesisConfigRequest,
  RpcBlockRequest,
  RpcBlockResponse,
  RpcChunkRequest,
  RpcChunkResponse,
  RpcClientConfigRequest,
  RpcClientConfigResponse,
  RpcCongestionLevelRequest,
  RpcCongestionLevelResponse,
  RpcGasPriceRequest,
  RpcGasPriceResponse,
  RpcHealthRequest,
  RpcHealthResponse,
  RpcLightClientBlockProofRequest,
  RpcLightClientBlockProofResponse,
  RpcLightClientExecutionProofRequest,
  RpcLightClientExecutionProofResponse,
  RpcLightClientNextBlockRequest,
  RpcLightClientNextBlockResponse,
  RpcMaintenanceWindowsRequest,
  RpcNetworkInfoRequest,
  RpcNetworkInfoResponse,
  RpcProtocolConfigRequest,
  RpcProtocolConfigResponse,
  RpcQueryRequest,
  RpcQueryResponse,
  RpcReceiptRequest,
  RpcReceiptResponse,
  RpcSendTransactionRequest,
  RpcSplitStorageInfoRequest,
  RpcSplitStorageInfoResponse,
  RpcStateChangesInBlockByTypeRequest,
  RpcStateChangesInBlockByTypeResponse,
  RpcStateChangesInBlockRequest,
  RpcStateChangesInBlockResponse,
  RpcStatusRequest,
  RpcStatusResponse,
  RpcTransactionResponse,
  RpcTransactionStatusRequest,
  RpcValidatorRequest,
  RpcValidatorResponse,
  RpcValidatorsOrderedRequest,
  ValidatorStakeViews
} from "./schemas.js";

import { schemas } from "./schemas.js";
import { z } from "zod";

/** Map of RPC method names to their request/response types */
export interface RpcMethodMap {
  "block": {
    request: RpcBlockRequest;
    response: RpcBlockResponse;
  };
  "chunk": {
    request: RpcChunkRequest;
    response: RpcChunkResponse;
  };
  "gas_price": {
    request: RpcGasPriceRequest;
    response: RpcGasPriceResponse;
  };
  "query": {
    request: RpcQueryRequest;
    response: RpcQueryResponse;
  };
  "send_tx": {
    request: RpcSendTransactionRequest;
    response: RpcTransactionResponse;
  };
  "tx": {
    request: RpcTransactionStatusRequest;
    response: RpcTransactionResponse;
  };
  "status": {
    request: RpcStatusRequest;
    response: RpcStatusResponse;
  };
  "validators": {
    request: RpcValidatorRequest;
    response: RpcValidatorResponse;
  };
  "network_info": {
    request: RpcNetworkInfoRequest;
    response: RpcNetworkInfoResponse;
  };
  "health": {
    request: RpcHealthRequest;
    response: RpcHealthResponse;
  };
  "light_client_proof": {
    request: RpcLightClientExecutionProofRequest;
    response: RpcLightClientExecutionProofResponse;
  };
  "next_light_client_block": {
    request: RpcLightClientNextBlockRequest;
    response: RpcLightClientNextBlockResponse;
  };
  "light_client_block_proof": {
    request: RpcLightClientBlockProofRequest;
    response: RpcLightClientBlockProofResponse;
  };
  "EXPERIMENTAL_changes_in_block": {
    request: RpcStateChangesInBlockRequest;
    response: RpcStateChangesInBlockByTypeResponse;
  };
  "EXPERIMENTAL_changes": {
    request: RpcStateChangesInBlockByTypeRequest;
    response: RpcStateChangesInBlockResponse;
  };
  "EXPERIMENTAL_protocol_config": {
    request: RpcProtocolConfigRequest;
    response: RpcProtocolConfigResponse;
  };
  "EXPERIMENTAL_genesis_config": {
    request: GenesisConfigRequest;
    response: GenesisConfig;
  };
  "EXPERIMENTAL_receipt": {
    request: RpcReceiptRequest;
    response: RpcReceiptResponse;
  };
  "EXPERIMENTAL_maintenance_windows": {
    request: RpcMaintenanceWindowsRequest;
    response: BlockHeightRanges;
  };
  "EXPERIMENTAL_split_storage_info": {
    request: RpcSplitStorageInfoRequest;
    response: RpcSplitStorageInfoResponse;
  };
  "EXPERIMENTAL_congestion_level": {
    request: RpcCongestionLevelRequest;
    response: RpcCongestionLevelResponse;
  };
  "EXPERIMENTAL_validators_ordered": {
    request: RpcValidatorsOrderedRequest;
    response: ValidatorStakeViews;
  };
  "EXPERIMENTAL_client_config": {
    request: RpcClientConfigRequest;
    response: RpcClientConfigResponse;
  };
  "EXPERIMENTAL_tx_status": {
    request: RpcTransactionStatusRequest;
    response: RpcTransactionResponse;
  };
  "EXPERIMENTAL_light_client_proof": {
    request: RpcLightClientExecutionProofRequest;
    response: RpcLightClientExecutionProofResponse;
  };
  "EXPERIMENTAL_light_client_block_proof": {
    request: RpcLightClientBlockProofRequest;
    response: RpcLightClientBlockProofResponse;
  };
  "block_effects": {
    request: RpcStateChangesInBlockRequest;
    response: RpcStateChangesInBlockByTypeResponse;
  };
  "changes": {
    request: RpcStateChangesInBlockByTypeRequest;
    response: RpcStateChangesInBlockResponse;
  };
  "genesis_config": {
    request: GenesisConfigRequest;
    response: GenesisConfig;
  };
  "client_config": {
    request: RpcClientConfigRequest;
    response: RpcClientConfigResponse;
  };
  "maintenance_windows": {
    request: RpcMaintenanceWindowsRequest;
    response: BlockHeightRanges;
  };
  "broadcast_tx_async": {
    request: RpcSendTransactionRequest;
    response: CryptoHash;
  };
  "broadcast_tx_commit": {
    request: RpcSendTransactionRequest;
    response: RpcTransactionResponse;
  };
}

/** Runtime schemas for request/response validation */
export const rpcMethodSchemas: Record<string, { request: z.ZodTypeAny; response: z.ZodTypeAny }> = {
  "block": {
    request: schemas.RpcBlockRequest,
    response: schemas.RpcBlockResponse,
  },
  "chunk": {
    request: schemas.RpcChunkRequest,
    response: schemas.RpcChunkResponse,
  },
  "gas_price": {
    request: schemas.RpcGasPriceRequest,
    response: schemas.RpcGasPriceResponse,
  },
  "query": {
    request: schemas.RpcQueryRequest,
    response: schemas.RpcQueryResponse,
  },
  "send_tx": {
    request: schemas.RpcSendTransactionRequest,
    response: schemas.RpcTransactionResponse,
  },
  "tx": {
    request: schemas.RpcTransactionStatusRequest,
    response: schemas.RpcTransactionResponse,
  },
  "status": {
    request: schemas.RpcStatusRequest,
    response: schemas.RpcStatusResponse,
  },
  "validators": {
    request: schemas.RpcValidatorRequest,
    response: schemas.RpcValidatorResponse,
  },
  "network_info": {
    request: schemas.RpcNetworkInfoRequest,
    response: schemas.RpcNetworkInfoResponse,
  },
  "health": {
    request: schemas.RpcHealthRequest,
    response: schemas.RpcHealthResponse,
  },
  "light_client_proof": {
    request: schemas.RpcLightClientExecutionProofRequest,
    response: schemas.RpcLightClientExecutionProofResponse,
  },
  "next_light_client_block": {
    request: schemas.RpcLightClientNextBlockRequest,
    response: schemas.RpcLightClientNextBlockResponse,
  },
  "light_client_block_proof": {
    request: schemas.RpcLightClientBlockProofRequest,
    response: schemas.RpcLightClientBlockProofResponse,
  },
  "EXPERIMENTAL_changes_in_block": {
    request: schemas.RpcStateChangesInBlockRequest,
    response: schemas.RpcStateChangesInBlockByTypeResponse,
  },
  "EXPERIMENTAL_changes": {
    request: schemas.RpcStateChangesInBlockByTypeRequest,
    response: schemas.RpcStateChangesInBlockResponse,
  },
  "EXPERIMENTAL_protocol_config": {
    request: schemas.RpcProtocolConfigRequest,
    response: schemas.RpcProtocolConfigResponse,
  },
  "EXPERIMENTAL_genesis_config": {
    request: schemas.GenesisConfigRequest,
    response: schemas.GenesisConfig,
  },
  "EXPERIMENTAL_receipt": {
    request: schemas.RpcReceiptRequest,
    response: schemas.RpcReceiptResponse,
  },
  "EXPERIMENTAL_maintenance_windows": {
    request: schemas.RpcMaintenanceWindowsRequest,
    response: schemas.BlockHeightRanges,
  },
  "EXPERIMENTAL_split_storage_info": {
    request: schemas.RpcSplitStorageInfoRequest,
    response: schemas.RpcSplitStorageInfoResponse,
  },
  "EXPERIMENTAL_congestion_level": {
    request: schemas.RpcCongestionLevelRequest,
    response: schemas.RpcCongestionLevelResponse,
  },
  "EXPERIMENTAL_validators_ordered": {
    request: schemas.RpcValidatorsOrderedRequest,
    response: schemas.ValidatorStakeViews,
  },
  "EXPERIMENTAL_client_config": {
    request: schemas.RpcClientConfigRequest,
    response: schemas.RpcClientConfigResponse,
  },
  "EXPERIMENTAL_tx_status": {
    request: schemas.RpcTransactionStatusRequest,
    response: schemas.RpcTransactionResponse,
  },
  "EXPERIMENTAL_light_client_proof": {
    request: schemas.RpcLightClientExecutionProofRequest,
    response: schemas.RpcLightClientExecutionProofResponse,
  },
  "EXPERIMENTAL_light_client_block_proof": {
    request: schemas.RpcLightClientBlockProofRequest,
    response: schemas.RpcLightClientBlockProofResponse,
  },
  "block_effects": {
    request: schemas.RpcStateChangesInBlockRequest,
    response: schemas.RpcStateChangesInBlockByTypeResponse,
  },
  "changes": {
    request: schemas.RpcStateChangesInBlockByTypeRequest,
    response: schemas.RpcStateChangesInBlockResponse,
  },
  "genesis_config": {
    request: schemas.GenesisConfigRequest,
    response: schemas.GenesisConfig,
  },
  "client_config": {
    request: schemas.RpcClientConfigRequest,
    response: schemas.RpcClientConfigResponse,
  },
  "maintenance_windows": {
    request: schemas.RpcMaintenanceWindowsRequest,
    response: schemas.BlockHeightRanges,
  },
  "broadcast_tx_async": {
    request: schemas.RpcSendTransactionRequest,
    response: schemas.CryptoHash,
  },
  "broadcast_tx_commit": {
    request: schemas.RpcSendTransactionRequest,
    response: schemas.RpcTransactionResponse,
  },
};

/** Type-safe RPC client interface */
export interface TypedRpcClient {
  block(request: RpcMethodMap["block"]["request"]): Promise<RpcMethodMap["block"]["response"]>;
  chunk(request: RpcMethodMap["chunk"]["request"]): Promise<RpcMethodMap["chunk"]["response"]>;
  gasPrice(request: RpcMethodMap["gas_price"]["request"]): Promise<RpcMethodMap["gas_price"]["response"]>;
  query(request: RpcMethodMap["query"]["request"]): Promise<RpcMethodMap["query"]["response"]>;
  sendTx(request: RpcMethodMap["send_tx"]["request"]): Promise<RpcMethodMap["send_tx"]["response"]>;
  tx(request: RpcMethodMap["tx"]["request"]): Promise<RpcMethodMap["tx"]["response"]>;
  status(request: RpcMethodMap["status"]["request"]): Promise<RpcMethodMap["status"]["response"]>;
  validators(request: RpcMethodMap["validators"]["request"]): Promise<RpcMethodMap["validators"]["response"]>;
  networkInfo(request: RpcMethodMap["network_info"]["request"]): Promise<RpcMethodMap["network_info"]["response"]>;
  health(request: RpcMethodMap["health"]["request"]): Promise<RpcMethodMap["health"]["response"]>;
  lightClientProof(request: RpcMethodMap["light_client_proof"]["request"]): Promise<RpcMethodMap["light_client_proof"]["response"]>;
  nextLightClientBlock(request: RpcMethodMap["next_light_client_block"]["request"]): Promise<RpcMethodMap["next_light_client_block"]["response"]>;
  lightClientBlockProof(request: RpcMethodMap["light_client_block_proof"]["request"]): Promise<RpcMethodMap["light_client_block_proof"]["response"]>;
  EXPERIMENTALChangesInBlock(request: RpcMethodMap["EXPERIMENTAL_changes_in_block"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_changes_in_block"]["response"]>;
  EXPERIMENTALChanges(request: RpcMethodMap["EXPERIMENTAL_changes"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_changes"]["response"]>;
  EXPERIMENTALProtocolConfig(request: RpcMethodMap["EXPERIMENTAL_protocol_config"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_protocol_config"]["response"]>;
  EXPERIMENTALGenesisConfig(request: RpcMethodMap["EXPERIMENTAL_genesis_config"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_genesis_config"]["response"]>;
  EXPERIMENTALReceipt(request: RpcMethodMap["EXPERIMENTAL_receipt"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_receipt"]["response"]>;
  EXPERIMENTALMaintenanceWindows(request: RpcMethodMap["EXPERIMENTAL_maintenance_windows"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_maintenance_windows"]["response"]>;
  EXPERIMENTALSplitStorageInfo(request: RpcMethodMap["EXPERIMENTAL_split_storage_info"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_split_storage_info"]["response"]>;
  EXPERIMENTALCongestionLevel(request: RpcMethodMap["EXPERIMENTAL_congestion_level"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_congestion_level"]["response"]>;
  EXPERIMENTALValidatorsOrdered(request: RpcMethodMap["EXPERIMENTAL_validators_ordered"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_validators_ordered"]["response"]>;
  EXPERIMENTALClientConfig(request: RpcMethodMap["EXPERIMENTAL_client_config"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_client_config"]["response"]>;
  EXPERIMENTALTxStatus(request: RpcMethodMap["EXPERIMENTAL_tx_status"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_tx_status"]["response"]>;
  EXPERIMENTALLightClientProof(request: RpcMethodMap["EXPERIMENTAL_light_client_proof"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_light_client_proof"]["response"]>;
  EXPERIMENTALLightClientBlockProof(request: RpcMethodMap["EXPERIMENTAL_light_client_block_proof"]["request"]): Promise<RpcMethodMap["EXPERIMENTAL_light_client_block_proof"]["response"]>;
  blockEffects(request: RpcMethodMap["block_effects"]["request"]): Promise<RpcMethodMap["block_effects"]["response"]>;
  changes(request: RpcMethodMap["changes"]["request"]): Promise<RpcMethodMap["changes"]["response"]>;
  genesisConfig(request: RpcMethodMap["genesis_config"]["request"]): Promise<RpcMethodMap["genesis_config"]["response"]>;
  clientConfig(request: RpcMethodMap["client_config"]["request"]): Promise<RpcMethodMap["client_config"]["response"]>;
  maintenanceWindows(request: RpcMethodMap["maintenance_windows"]["request"]): Promise<RpcMethodMap["maintenance_windows"]["response"]>;
  broadcastTxAsync(request: RpcMethodMap["broadcast_tx_async"]["request"]): Promise<RpcMethodMap["broadcast_tx_async"]["response"]>;
  broadcastTxCommit(request: RpcMethodMap["broadcast_tx_commit"]["request"]): Promise<RpcMethodMap["broadcast_tx_commit"]["response"]>;
}

/** Union of all RPC method names */
export type RpcMethodName = keyof RpcMethodMap;

/** Helper types for generic RPC handling */
export type RpcRequest<M extends RpcMethodName> = RpcMethodMap[M]["request"];
export type RpcResponse<M extends RpcMethodName> = RpcMethodMap[M]["response"];