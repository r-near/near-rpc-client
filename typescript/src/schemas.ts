import { z } from "zod";
// Utility type to force TypeScript to expand types for cleaner hover display
type Simplify<T> = { [K in keyof T]: T[K] } & {};


const NearToken = z.string();
const FunctionCallPermission = z.object({
  allowance: z.union([NearToken, z.null()]).optional(),
  method_names: z.array(z.string()),
  receiver_id: z.string(),
});
const GasKeyInfo = z.object({
  balance: NearToken,
  num_nonces: z.number().int().gte(0).lte(65535),
});
const AccessKeyPermission = z.union([
  z.object({ FunctionCall: FunctionCallPermission }),
  z.literal("FullAccess"),
  z.object({ GasKeyFunctionCall: z.array(z.unknown()).min(2).max(2) }),
  z.object({ GasKeyFullAccess: GasKeyInfo }),
]);
const AccessKey = z.object({
  nonce: z.number().int().gte(0),
  permission: AccessKeyPermission,
});
const NearGas = z.number();
const Fee = z.object({
  execution: NearGas,
  send_not_sir: NearGas,
  send_sir: NearGas,
});
const AccessKeyCreationConfigView = z.object({
  full_access_cost: Fee,
  function_call_cost: Fee,
  function_call_cost_per_byte: Fee,
});
const AccessKeyPermissionView = z.union([
  z.literal("FullAccess"),
  z.object({
    FunctionCall: z.object({
      allowance: z.union([NearToken, z.null()]).optional(),
      method_names: z.array(z.string()),
      receiver_id: z.string(),
    }),
  }),
  z.object({
    GasKeyFunctionCall: z.object({
      allowance: z.union([NearToken, z.null()]).optional(),
      balance: NearToken,
      method_names: z.array(z.string()),
      num_nonces: z.number().int().gte(0).lte(65535),
      receiver_id: z.string(),
    }),
  }),
  z.object({
    GasKeyFullAccess: z.object({
      balance: NearToken,
      num_nonces: z.number().int().gte(0).lte(65535),
    }),
  }),
]);
const AccessKeyView = z.object({
  nonce: z.number().int().gte(0),
  permission: AccessKeyPermissionView,
});
const PublicKey = z.string();
const AccessKeyInfoView = z.object({
  access_key: AccessKeyView,
  public_key: PublicKey,
});
const AccessKeyList = z.object({ keys: z.array(AccessKeyInfoView) });
const AccountId = z.string();
const AccountCreationConfigView = z.object({
  min_allowed_top_level_account_length: z.number().int().gte(0).lte(255),
  registrar_account_id: AccountId,
});
const Tier1ProxyView = z.object({ addr: z.string(), peer_id: PublicKey });
const AccountDataView = z.object({
  account_key: PublicKey,
  peer_id: PublicKey,
  proxies: z.array(Tier1ProxyView),
  timestamp: z.string(),
});
const AccountIdValidityRulesVersion = z.number();
const AccountInfo = z.object({
  account_id: AccountId,
  amount: NearToken,
  public_key: PublicKey,
});
const CryptoHash = z.string();
const AccountView = z.object({
  amount: NearToken,
  code_hash: CryptoHash,
  global_contract_account_id: z.union([AccountId, z.null()]).optional(),
  global_contract_hash: z.union([CryptoHash, z.null()]).optional(),
  locked: NearToken,
  storage_paid_at: z.number().int().gte(0).optional(),
  storage_usage: z.number().int().gte(0),
});
const AccountWithPublicKey = z.object({
  account_id: AccountId,
  public_key: PublicKey,
});
const ActionCreationConfigView = z.object({
  add_key_cost: AccessKeyCreationConfigView,
  create_account_cost: Fee,
  delegate_cost: Fee,
  delete_account_cost: Fee,
  delete_key_cost: Fee,
  deploy_contract_cost: Fee,
  deploy_contract_cost_per_byte: Fee,
  function_call_cost: Fee,
  function_call_cost_per_byte: Fee,
  stake_cost: Fee,
  transfer_cost: Fee,
});
const PrepareError = z.union([
  z.literal("Serialization"),
  z.literal("Deserialization"),
  z.literal("InternalMemoryDeclared"),
  z.literal("GasInstrumentation"),
  z.literal("StackHeightInstrumentation"),
  z.literal("Instantiate"),
  z.literal("Memory"),
  z.literal("TooManyFunctions"),
  z.literal("TooManyLocals"),
  z.literal("TooManyTables"),
  z.literal("TooManyTableElements"),
]);
const CompilationError = z.union([
  z.object({ CodeDoesNotExist: z.object({ account_id: AccountId }) }),
  z.object({ PrepareError: PrepareError }),
  z.object({ WasmerCompileError: z.object({ msg: z.string() }) }),
]);
const MethodResolveError = z.enum([
  "MethodEmptyName",
  "MethodNotFound",
  "MethodInvalidSignature",
]);
const WasmTrap = z.union([
  z.literal("Unreachable"),
  z.literal("IncorrectCallIndirectSignature"),
  z.literal("MemoryOutOfBounds"),
  z.literal("CallIndirectOOB"),
  z.literal("IllegalArithmetic"),
  z.literal("MisalignedAtomicAccess"),
  z.literal("IndirectCallToNull"),
  z.literal("StackOverflow"),
  z.literal("GenericTrap"),
]);
const HostError = z.union([
  z.literal("BadUTF16"),
  z.literal("BadUTF8"),
  z.literal("GasExceeded"),
  z.literal("GasLimitExceeded"),
  z.literal("BalanceExceeded"),
  z.literal("EmptyMethodName"),
  z.object({ GuestPanic: z.object({ panic_msg: z.string() }) }),
  z.literal("IntegerOverflow"),
  z.object({
    InvalidPromiseIndex: z.object({ promise_idx: z.number().int().gte(0) }),
  }),
  z.literal("CannotAppendActionToJointPromise"),
  z.literal("CannotReturnJointPromise"),
  z.object({
    InvalidPromiseResultIndex: z.object({
      result_idx: z.number().int().gte(0),
    }),
  }),
  z.object({
    InvalidRegisterId: z.object({ register_id: z.number().int().gte(0) }),
  }),
  z.object({
    IteratorWasInvalidated: z.object({
      iterator_index: z.number().int().gte(0),
    }),
  }),
  z.literal("MemoryAccessViolation"),
  z.object({
    InvalidReceiptIndex: z.object({ receipt_index: z.number().int().gte(0) }),
  }),
  z.object({
    InvalidIteratorIndex: z.object({ iterator_index: z.number().int().gte(0) }),
  }),
  z.literal("InvalidAccountId"),
  z.literal("InvalidMethodName"),
  z.literal("InvalidPublicKey"),
  z.object({ ProhibitedInView: z.object({ method_name: z.string() }) }),
  z.object({
    NumberOfLogsExceeded: z.object({ limit: z.number().int().gte(0) }),
  }),
  z.object({
    KeyLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({
    ValueLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({
    TotalLogLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({
    NumberPromisesExceeded: z.object({
      limit: z.number().int().gte(0),
      number_of_promises: z.number().int().gte(0),
    }),
  }),
  z.object({
    NumberInputDataDependenciesExceeded: z.object({
      limit: z.number().int().gte(0),
      number_of_input_data_dependencies: z.number().int().gte(0),
    }),
  }),
  z.object({
    ReturnedValueLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({
    ContractSizeExceeded: z.object({
      limit: z.number().int().gte(0),
      size: z.number().int().gte(0),
    }),
  }),
  z.object({ Deprecated: z.object({ method_name: z.string() }) }),
  z.object({ ECRecoverError: z.object({ msg: z.string() }) }),
  z.object({ AltBn128InvalidInput: z.object({ msg: z.string() }) }),
  z.object({ Ed25519VerifyInvalidInput: z.object({ msg: z.string() }) }),
]);
const FunctionCallError = z.union([
  z.enum(["WasmUnknownError", "_EVMError"]),
  z.object({ CompilationError: CompilationError }),
  z.object({ LinkError: z.object({ msg: z.string() }) }),
  z.object({ MethodResolveError: MethodResolveError }),
  z.object({ WasmTrap: WasmTrap }),
  z.object({ HostError: HostError }),
  z.object({ ExecutionError: z.string() }),
]);
const ActionsValidationError = z.union([
  z.literal("DeleteActionMustBeFinal"),
  z.object({
    TotalPrepaidGasExceeded: z.object({
      limit: NearGas.int().gte(0),
      total_prepaid_gas: NearGas.int().gte(0),
    }),
  }),
  z.object({
    TotalNumberOfActionsExceeded: z.object({
      limit: z.number().int().gte(0),
      total_number_of_actions: z.number().int().gte(0),
    }),
  }),
  z.object({
    AddKeyMethodNamesNumberOfBytesExceeded: z.object({
      limit: z.number().int().gte(0),
      total_number_of_bytes: z.number().int().gte(0),
    }),
  }),
  z.object({
    AddKeyMethodNameLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.literal("IntegerOverflow"),
  z.object({ InvalidAccountId: z.object({ account_id: z.string() }) }),
  z.object({
    ContractSizeExceeded: z.object({
      limit: z.number().int().gte(0),
      size: z.number().int().gte(0),
    }),
  }),
  z.object({
    FunctionCallMethodNameLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({
    FunctionCallArgumentsLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({ UnsuitableStakingKey: z.object({ public_key: PublicKey }) }),
  z.literal("FunctionCallZeroAttachedGas"),
  z.literal("DelegateActionMustBeOnlyOne"),
  z.object({
    UnsupportedProtocolFeature: z.object({
      protocol_feature: z.string(),
      version: z.number().int().gte(0),
    }),
  }),
  z.object({
    InvalidDeterministicStateInitReceiver: z.object({
      derived_id: AccountId,
      receiver_id: AccountId,
    }),
  }),
  z.object({
    DeterministicStateInitKeyLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({
    DeterministicStateInitValueLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({
    GasKeyInvalidNumNonces: z.object({
      limit: z.number().int().gte(0).lte(65535),
      requested_nonces: z.number().int().gte(0).lte(65535),
    }),
  }),
  z.object({ AddGasKeyWithNonZeroBalance: z.object({ balance: NearToken }) }),
  z.literal("GasKeyFunctionCallAllowanceNotAllowed"),
]);
const ReceiptValidationError = z.union([
  z.object({ InvalidPredecessorId: z.object({ account_id: z.string() }) }),
  z.object({ InvalidReceiverId: z.object({ account_id: z.string() }) }),
  z.object({ InvalidSignerId: z.object({ account_id: z.string() }) }),
  z.object({ InvalidDataReceiverId: z.object({ account_id: z.string() }) }),
  z.object({
    ReturnedValueLengthExceeded: z.object({
      length: z.number().int().gte(0),
      limit: z.number().int().gte(0),
    }),
  }),
  z.object({
    NumberInputDataDependenciesExceeded: z.object({
      limit: z.number().int().gte(0),
      number_of_input_data_dependencies: z.number().int().gte(0),
    }),
  }),
  z.object({ ActionsValidation: ActionsValidationError }),
  z.object({
    ReceiptSizeExceeded: z.object({
      limit: z.number().int().gte(0),
      size: z.number().int().gte(0),
    }),
  }),
  z.object({ InvalidRefundTo: z.object({ account_id: z.string() }) }),
]);
const InvalidAccessKeyError = z.union([
  z.object({
    AccessKeyNotFound: z.object({
      account_id: AccountId,
      public_key: PublicKey,
    }),
  }),
  z.object({
    ReceiverMismatch: z.object({
      ak_receiver: z.string(),
      tx_receiver: AccountId,
    }),
  }),
  z.object({ MethodNameMismatch: z.object({ method_name: z.string() }) }),
  z.literal("RequiresFullAccess"),
  z.object({
    NotEnoughAllowance: z.object({
      account_id: AccountId,
      allowance: NearToken,
      cost: NearToken,
      public_key: PublicKey,
    }),
  }),
  z.literal("DepositWithFunctionCall"),
]);
const GlobalContractIdentifier = z.union([
  z.object({ CodeHash: CryptoHash }),
  z.object({ AccountId: AccountId }),
]);
const ActionErrorKind = z.union([
  z.object({ AccountAlreadyExists: z.object({ account_id: AccountId }) }),
  z.object({ AccountDoesNotExist: z.object({ account_id: AccountId }) }),
  z.object({
    CreateAccountOnlyByRegistrar: z.object({
      account_id: AccountId,
      predecessor_id: AccountId,
      registrar_account_id: AccountId,
    }),
  }),
  z.object({
    CreateAccountNotAllowed: z.object({
      account_id: AccountId,
      predecessor_id: AccountId,
    }),
  }),
  z.object({
    ActorNoPermission: z.object({ account_id: AccountId, actor_id: AccountId }),
  }),
  z.object({
    DeleteKeyDoesNotExist: z.object({
      account_id: AccountId,
      public_key: PublicKey,
    }),
  }),
  z.object({
    AddKeyAlreadyExists: z.object({
      account_id: AccountId,
      public_key: PublicKey,
    }),
  }),
  z.object({ DeleteAccountStaking: z.object({ account_id: AccountId }) }),
  z.object({
    LackBalanceForState: z.object({ account_id: AccountId, amount: NearToken }),
  }),
  z.object({ TriesToUnstake: z.object({ account_id: AccountId }) }),
  z.object({
    TriesToStake: z.object({
      account_id: AccountId,
      balance: NearToken,
      locked: NearToken,
      stake: NearToken,
    }),
  }),
  z.object({
    InsufficientStake: z.object({
      account_id: AccountId,
      minimum_stake: NearToken,
      stake: NearToken,
    }),
  }),
  z.object({ FunctionCallError: FunctionCallError }),
  z.object({ NewReceiptValidationError: ReceiptValidationError }),
  z.object({
    OnlyImplicitAccountCreationAllowed: z.object({ account_id: AccountId }),
  }),
  z.object({
    DeleteAccountWithLargeState: z.object({ account_id: AccountId }),
  }),
  z.literal("DelegateActionInvalidSignature"),
  z.object({
    DelegateActionSenderDoesNotMatchTxReceiver: z.object({
      receiver_id: AccountId,
      sender_id: AccountId,
    }),
  }),
  z.literal("DelegateActionExpired"),
  z.object({ DelegateActionAccessKeyError: InvalidAccessKeyError }),
  z.object({
    DelegateActionInvalidNonce: z.object({
      ak_nonce: z.number().int().gte(0),
      delegate_nonce: z.number().int().gte(0),
    }),
  }),
  z.object({
    DelegateActionNonceTooLarge: z.object({
      delegate_nonce: z.number().int().gte(0),
      upper_bound: z.number().int().gte(0),
    }),
  }),
  z.object({
    GlobalContractDoesNotExist: z.object({
      identifier: GlobalContractIdentifier,
    }),
  }),
  z.object({
    GasKeyDoesNotExist: z.object({
      account_id: AccountId,
      public_key: PublicKey,
    }),
  }),
  z.object({
    InsufficientGasKeyBalance: z.object({
      account_id: AccountId,
      balance: NearToken,
      public_key: PublicKey,
      required: NearToken,
    }),
  }),
]);
const ActionError = z.object({
  index: z.union([z.number(), z.null()]).optional(),
  kind: ActionErrorKind,
});
const FunctionArgs = z.string();
const CreateAccountAction = z.object({}).partial();
const DeployContractAction = z.object({ code: z.string() });
const FunctionCallAction = z.object({
  args: z.string(),
  deposit: NearToken,
  gas: NearGas.int().gte(0),
  method_name: z.string(),
});
const TransferAction = z.object({ deposit: NearToken });
const StakeAction = z.object({ public_key: PublicKey, stake: NearToken });
const AddKeyAction = z.object({ access_key: AccessKey, public_key: PublicKey });
const DeleteKeyAction = z.object({ public_key: PublicKey });
const DeleteAccountAction = z.object({ beneficiary_id: AccountId });
const GlobalContractDeployMode = z.union([
  z.literal("CodeHash"),
  z.literal("AccountId"),
]);
const DeployGlobalContractAction = z.object({
  code: z.string(),
  deploy_mode: GlobalContractDeployMode,
});
const UseGlobalContractAction = z.object({
  contract_identifier: GlobalContractIdentifier,
});
const DeterministicAccountStateInitV1 = z.object({
  code: GlobalContractIdentifier,
  data: z.record(z.string()),
});
const DeterministicAccountStateInit = z.object({
  V1: DeterministicAccountStateInitV1,
});
const DeterministicStateInitAction = z.object({
  deposit: NearToken,
  state_init: DeterministicAccountStateInit,
});
const TransferToGasKeyAction = z.object({
  deposit: NearToken,
  public_key: PublicKey,
});
const WithdrawFromGasKeyAction = z.object({
  amount: NearToken,
  public_key: PublicKey,
});
const NonDelegateAction = z.union([
  z.object({ CreateAccount: CreateAccountAction }),
  z.object({ DeployContract: DeployContractAction }),
  z.object({ FunctionCall: FunctionCallAction }),
  z.object({ Transfer: TransferAction }),
  z.object({ Stake: StakeAction }),
  z.object({ AddKey: AddKeyAction }),
  z.object({ DeleteKey: DeleteKeyAction }),
  z.object({ DeleteAccount: DeleteAccountAction }),
  z.object({ DeployGlobalContract: DeployGlobalContractAction }),
  z.object({ UseGlobalContract: UseGlobalContractAction }),
  z.object({ DeterministicStateInit: DeterministicStateInitAction }),
  z.object({ TransferToGasKey: TransferToGasKeyAction }),
  z.object({ WithdrawFromGasKey: WithdrawFromGasKeyAction }),
]);
const DelegateAction = z.object({
  actions: z.array(NonDelegateAction),
  max_block_height: z.number().int().gte(0),
  nonce: z.number().int().gte(0),
  public_key: PublicKey,
  receiver_id: AccountId,
  sender_id: AccountId,
});
const Signature = z.string();
const GlobalContractIdentifierView = z.union([
  z.object({ hash: CryptoHash }),
  z.object({ account_id: AccountId }),
]);
const ActionView = z.union([
  z.literal("CreateAccount"),
  z.object({ DeployContract: z.object({ code: z.string() }) }),
  z.object({
    FunctionCall: z.object({
      args: FunctionArgs,
      deposit: NearToken,
      gas: NearGas.int().gte(0),
      method_name: z.string(),
    }),
  }),
  z.object({ Transfer: z.object({ deposit: NearToken }) }),
  z.object({ Stake: z.object({ public_key: PublicKey, stake: NearToken }) }),
  z.object({
    AddKey: z.object({ access_key: AccessKeyView, public_key: PublicKey }),
  }),
  z.object({ DeleteKey: z.object({ public_key: PublicKey }) }),
  z.object({ DeleteAccount: z.object({ beneficiary_id: AccountId }) }),
  z.object({
    Delegate: z.object({
      delegate_action: DelegateAction,
      signature: Signature,
    }),
  }),
  z.object({ DeployGlobalContract: z.object({ code: z.string() }) }),
  z.object({ DeployGlobalContractByAccountId: z.object({ code: z.string() }) }),
  z.object({ UseGlobalContract: z.object({ code_hash: CryptoHash }) }),
  z.object({
    UseGlobalContractByAccountId: z.object({ account_id: AccountId }),
  }),
  z.object({
    DeterministicStateInit: z.object({
      code: GlobalContractIdentifierView,
      data: z.record(z.string()),
      deposit: NearToken,
    }),
  }),
  z.object({
    TransferToGasKey: z.object({ deposit: NearToken, public_key: PublicKey }),
  }),
  z.object({
    WithdrawFromGasKey: z.object({ amount: NearToken, public_key: PublicKey }),
  }),
]);
const BandwidthRequestBitmap = z.object({
  data: z.array(z.number().int().gte(0).lte(255)).min(5).max(5),
});
const BandwidthRequest = z.object({
  requested_values_bitmap: BandwidthRequestBitmap,
  to_shard: z.number().int().gte(0).lte(65535),
});
const BandwidthRequestsV1 = z.object({ requests: z.array(BandwidthRequest) });
const BandwidthRequests = z.object({ V1: BandwidthRequestsV1 });
const BlockHeaderInnerLiteView = z.object({
  block_merkle_root: CryptoHash,
  epoch_id: CryptoHash,
  height: z.number().int().gte(0),
  next_bp_hash: CryptoHash,
  next_epoch_id: CryptoHash,
  outcome_root: CryptoHash,
  prev_state_root: CryptoHash,
  timestamp: z.number().int().gte(0),
  timestamp_nanosec: z.string(),
});
const SlashedValidator = z.object({
  account_id: AccountId,
  is_double_sign: z.boolean(),
});
const ValidatorStakeViewV1 = z.object({
  account_id: AccountId,
  public_key: PublicKey,
  stake: NearToken,
});
const ValidatorStakeView = ValidatorStakeViewV1;
const BlockHeaderView = z.object({
  approvals: z.array(z.union([Signature, z.null()])),
  block_body_hash: z.union([CryptoHash, z.null()]).optional(),
  block_merkle_root: CryptoHash,
  block_ordinal: z.union([z.number(), z.null()]).optional(),
  challenges_result: z.array(SlashedValidator),
  challenges_root: CryptoHash,
  chunk_endorsements: z
    .union([z.array(z.array(z.number().int().gte(0).lte(255))), z.null()])
    .optional(),
  chunk_headers_root: CryptoHash,
  chunk_mask: z.array(z.boolean()),
  chunk_receipts_root: CryptoHash,
  chunk_tx_root: CryptoHash,
  chunks_included: z.number().int().gte(0),
  epoch_id: CryptoHash,
  epoch_sync_data_hash: z.union([CryptoHash, z.null()]).optional(),
  gas_price: NearToken,
  hash: CryptoHash,
  height: z.number().int().gte(0),
  last_ds_final_block: CryptoHash,
  last_final_block: CryptoHash,
  latest_protocol_version: z.number().int().gte(0),
  next_bp_hash: CryptoHash,
  next_epoch_id: CryptoHash,
  outcome_root: CryptoHash,
  prev_hash: CryptoHash,
  prev_height: z.union([z.number(), z.null()]).optional(),
  prev_state_root: CryptoHash,
  random_value: CryptoHash,
  rent_paid: NearToken.optional(),
  shard_split: z.union([z.array(z.unknown()), z.null()]).optional(),
  signature: Signature,
  timestamp: z.number().int().gte(0),
  timestamp_nanosec: z.string(),
  total_supply: NearToken,
  validator_proposals: z.array(ValidatorStakeView),
  validator_reward: NearToken.optional(),
});
const BlockHeightRange = z.object({
  end: z.number().int().gte(0),
  start: z.number().int().gte(0),
});
const BlockHeightRanges = z.array(BlockHeightRange);
const BlockId = z.union([z.number(), CryptoHash]);
const Finality = z.enum(["optimistic", "near-final", "final"]);
const SyncCheckpoint = z.enum(["genesis", "earliest_available"]);
const BlockReference = z.union([
  z.object({ block_id: BlockId }),
  z.object({ finality: Finality }),
  z.object({ sync_checkpoint: SyncCheckpoint }),
]);
const BlockStatusView = z.object({
  hash: CryptoHash,
  height: z.number().int().gte(0),
});
const CallResult = z.object({
  logs: z.array(z.string()),
  result: z.array(z.number().int().gte(0).lte(255)),
});
const CatchupStatusView = z.object({
  blocks_to_catchup: z.array(BlockStatusView),
  shard_sync_status: z.object({}).partial(),
  sync_block_hash: CryptoHash,
  sync_block_height: z.number().int().gte(0),
});
const ChunkDistributionUris = z.object({ get: z.string(), set: z.string() });
const ChunkDistributionNetworkConfig = z.object({
  enabled: z.boolean(),
  uris: ChunkDistributionUris,
});
const CongestionInfoView = z.object({
  allowed_shard: z.number().int().gte(0).lte(65535),
  buffered_receipts_gas: z.string(),
  delayed_receipts_gas: z.string(),
  receipt_bytes: z.number().int().gte(0),
});
const ShardId = z.number();
const ChunkHeaderView = z.object({
  balance_burnt: NearToken,
  bandwidth_requests: z.union([BandwidthRequests, z.null()]).optional(),
  chunk_hash: CryptoHash,
  congestion_info: z.union([CongestionInfoView, z.null()]).optional(),
  encoded_length: z.number().int().gte(0),
  encoded_merkle_root: CryptoHash,
  gas_limit: NearGas.int().gte(0),
  gas_used: NearGas.int().gte(0),
  height_created: z.number().int().gte(0),
  height_included: z.number().int().gte(0),
  outcome_root: CryptoHash,
  outgoing_receipts_root: CryptoHash,
  prev_block_hash: CryptoHash,
  prev_state_root: CryptoHash,
  rent_paid: NearToken.optional(),
  shard_id: ShardId.int().gte(0),
  signature: Signature,
  tx_root: CryptoHash,
  validator_proposals: z.array(ValidatorStakeView),
  validator_reward: NearToken.optional(),
});
const DurationAsStdSchemaProvider = z.object({
  nanos: z.number().int(),
  secs: z.number().int(),
});
const CloudArchivalWriterConfig = z
  .object({
    archive_block_data: z.boolean(),
    polling_interval: DurationAsStdSchemaProvider,
  })
  .partial();
const CongestionControlConfigView = z.object({
  allowed_shard_outgoing_gas: NearGas,
  max_congestion_incoming_gas: NearGas,
  max_congestion_memory_consumption: z.number().int().gte(0),
  max_congestion_missed_chunks: z.number().int().gte(0),
  max_congestion_outgoing_gas: NearGas,
  max_outgoing_gas: NearGas,
  max_tx_gas: NearGas,
  min_outgoing_gas: NearGas,
  min_tx_gas: NearGas,
  outgoing_receipts_big_size_limit: z.number().int().gte(0),
  outgoing_receipts_usual_size_limit: z.number().int().gte(0),
  reject_tx_congestion_threshold: z.number(),
});
const ContractCodeView = z.object({
  code_base64: z.string(),
  hash: CryptoHash,
});
const CostGasUsed = z.object({
  cost: z.string(),
  cost_category: z.string(),
  gas_used: z.string(),
});
const CurrentEpochValidatorInfo = z.object({
  account_id: AccountId,
  is_slashed: z.boolean(),
  num_expected_blocks: z.number().int().gte(0),
  num_expected_chunks: z.number().int().gte(0).optional(),
  num_expected_chunks_per_shard: z.array(z.number().int().gte(0)).optional(),
  num_expected_endorsements: z.number().int().gte(0).optional(),
  num_expected_endorsements_per_shard: z
    .array(z.number().int().gte(0))
    .optional(),
  num_produced_blocks: z.number().int().gte(0),
  num_produced_chunks: z.number().int().gte(0).optional(),
  num_produced_chunks_per_shard: z.array(z.number().int().gte(0)).optional(),
  num_produced_endorsements: z.number().int().gte(0).optional(),
  num_produced_endorsements_per_shard: z
    .array(z.number().int().gte(0))
    .optional(),
  public_key: PublicKey,
  shards: z.array(ShardId),
  shards_endorsed: z.array(ShardId).optional(),
  stake: NearToken,
});
const DataReceiptCreationConfigView = z.object({
  base_cost: Fee,
  cost_per_byte: Fee,
});
const DataReceiverView = z.object({
  data_id: CryptoHash,
  receiver_id: AccountId,
});
const PeerInfoView = z.object({
  account_id: z.union([AccountId, z.null()]).optional(),
  addr: z.string(),
  archival: z.boolean(),
  block_hash: z.union([CryptoHash, z.null()]).optional(),
  connection_established_time_millis: z.number().int().gte(0),
  height: z.union([z.number(), z.null()]).optional(),
  is_highest_block_invalid: z.boolean(),
  is_outbound_peer: z.boolean(),
  last_time_peer_requested_millis: z.number().int().gte(0),
  last_time_received_message_millis: z.number().int().gte(0),
  nonce: z.number().int().gte(0),
  peer_id: PublicKey,
  received_bytes_per_sec: z.number().int().gte(0),
  sent_bytes_per_sec: z.number().int().gte(0),
  tracked_shards: z.array(ShardId),
});
const KnownProducerView = z.object({
  account_id: AccountId,
  next_hops: z.union([z.array(PublicKey), z.null()]).optional(),
  peer_id: PublicKey,
});
const NetworkInfoView = z.object({
  connected_peers: z.array(PeerInfoView),
  known_producers: z.array(KnownProducerView),
  num_connected_peers: z.number().int().gte(0),
  peer_max_count: z.number().int().gte(0),
  tier1_accounts_data: z.array(AccountDataView),
  tier1_accounts_keys: z.array(PublicKey),
  tier1_connections: z.array(PeerInfoView),
});
const DetailedDebugStatus = z.object({
  block_production_delay_millis: z.number().int().gte(0),
  catchup_status: z.array(CatchupStatusView),
  current_head_status: BlockStatusView,
  current_header_head_status: BlockStatusView,
  network_info: NetworkInfoView,
  sync_status: z.string(),
});
const Direction = z.enum(["Left", "Right"]);
const ExternalStorageLocation = z.union([
  z.object({ S3: z.object({ bucket: z.string(), region: z.string() }) }),
  z.object({ Filesystem: z.object({ root_dir: z.string() }) }),
  z.object({ GCS: z.object({ bucket: z.string() }) }),
]);
const DumpConfig = z.object({
  credentials_file: z.union([z.string(), z.null()]).optional(),
  iteration_delay: z.union([DurationAsStdSchemaProvider, z.null()]).optional(),
  location: ExternalStorageLocation,
  restart_dump_for_shards: z.union([z.array(ShardId), z.null()]).optional(),
});
const EpochId = CryptoHash;
const EpochSyncConfig = z.object({
  epoch_sync_horizon: z.number().int().gte(0),
  timeout_for_epoch_sync: DurationAsStdSchemaProvider,
});
const ExecutionMetadataView = z.object({
  gas_profile: z.union([z.array(CostGasUsed), z.null()]).optional(),
  version: z.number().int().gte(0),
});
const MissingTrieValueContext = z.union([
  z.literal("TrieIterator"),
  z.literal("TriePrefetchingStorage"),
  z.literal("TrieMemoryPartialStorage"),
  z.literal("TrieStorage"),
]);
const MissingTrieValue = z.object({
  context: MissingTrieValueContext,
  hash: CryptoHash,
});
const StorageError = z.union([
  z.literal("StorageInternalError"),
  z.object({ MissingTrieValue: MissingTrieValue }),
  z.literal("UnexpectedTrieValue"),
  z.object({ StorageInconsistentState: z.string() }),
  z.object({ FlatStorageBlockNotSupported: z.string() }),
  z.object({ MemTrieLoadingError: z.string() }),
]);
const InvalidTxError = z.union([
  z.object({ InvalidAccessKeyError: InvalidAccessKeyError }),
  z.object({ InvalidSignerId: z.object({ signer_id: z.string() }) }),
  z.object({ SignerDoesNotExist: z.object({ signer_id: AccountId }) }),
  z.object({
    InvalidNonce: z.object({
      ak_nonce: z.number().int().gte(0),
      tx_nonce: z.number().int().gte(0),
    }),
  }),
  z.object({
    NonceTooLarge: z.object({
      tx_nonce: z.number().int().gte(0),
      upper_bound: z.number().int().gte(0),
    }),
  }),
  z.object({ InvalidReceiverId: z.object({ receiver_id: z.string() }) }),
  z.literal("InvalidSignature"),
  z.object({
    NotEnoughBalance: z.object({
      balance: NearToken,
      cost: NearToken,
      signer_id: AccountId,
    }),
  }),
  z.object({
    LackBalanceForState: z.object({ amount: NearToken, signer_id: AccountId }),
  }),
  z.literal("CostOverflow"),
  z.literal("InvalidChain"),
  z.literal("Expired"),
  z.object({ ActionsValidation: ActionsValidationError }),
  z.object({
    TransactionSizeExceeded: z.object({
      limit: z.number().int().gte(0),
      size: z.number().int().gte(0),
    }),
  }),
  z.literal("InvalidTransactionVersion"),
  z.object({ StorageError: StorageError }),
  z.object({
    ShardCongested: z.object({
      congestion_level: z.number(),
      shard_id: z.number().int().gte(0),
    }),
  }),
  z.object({
    ShardStuck: z.object({
      missed_chunks: z.number().int().gte(0),
      shard_id: z.number().int().gte(0),
    }),
  }),
  z.object({
    InvalidNonceIndex: z.object({
      num_nonces: z.number().int().gte(0).lte(65535),
      tx_nonce_index: z.union([z.number(), z.null()]).optional(),
    }),
  }),
  z.object({
    NotEnoughGasKeyBalance: z.object({
      balance: NearToken,
      cost: NearToken,
      signer_id: AccountId,
    }),
  }),
]);
const TxExecutionError = z.union([
  z.object({ ActionError: ActionError }),
  z.object({ InvalidTxError: InvalidTxError }),
]);
const ExecutionStatusView = z.union([
  z.literal("Unknown"),
  z.object({ Failure: TxExecutionError }),
  z.object({ SuccessValue: z.string() }),
  z.object({ SuccessReceiptId: CryptoHash }),
]);
const ExecutionOutcomeView = z.object({
  executor_id: AccountId,
  gas_burnt: NearGas,
  logs: z.array(z.string()),
  metadata: ExecutionMetadataView.optional(),
  receipt_ids: z.array(CryptoHash),
  status: ExecutionStatusView,
  tokens_burnt: NearToken,
});
const MerklePathItem = z.object({ direction: Direction, hash: CryptoHash });
const ExecutionOutcomeWithIdView = z.object({
  block_hash: CryptoHash,
  id: CryptoHash,
  outcome: ExecutionOutcomeView,
  proof: z.array(MerklePathItem),
});
const ExtCostsConfigView = z.object({
  alt_bn128_g1_multiexp_base: NearGas,
  alt_bn128_g1_multiexp_element: NearGas,
  alt_bn128_g1_sum_base: NearGas,
  alt_bn128_g1_sum_element: NearGas,
  alt_bn128_pairing_check_base: NearGas,
  alt_bn128_pairing_check_element: NearGas,
  base: NearGas,
  bls12381_g1_multiexp_base: NearGas.int().gte(0),
  bls12381_g1_multiexp_element: NearGas.int().gte(0),
  bls12381_g2_multiexp_base: NearGas.int().gte(0),
  bls12381_g2_multiexp_element: NearGas.int().gte(0),
  bls12381_map_fp2_to_g2_base: NearGas.int().gte(0),
  bls12381_map_fp2_to_g2_element: NearGas.int().gte(0),
  bls12381_map_fp_to_g1_base: NearGas.int().gte(0),
  bls12381_map_fp_to_g1_element: NearGas.int().gte(0),
  bls12381_p1_decompress_base: NearGas.int().gte(0),
  bls12381_p1_decompress_element: NearGas.int().gte(0),
  bls12381_p1_sum_base: NearGas.int().gte(0),
  bls12381_p1_sum_element: NearGas.int().gte(0),
  bls12381_p2_decompress_base: NearGas.int().gte(0),
  bls12381_p2_decompress_element: NearGas.int().gte(0),
  bls12381_p2_sum_base: NearGas.int().gte(0),
  bls12381_p2_sum_element: NearGas.int().gte(0),
  bls12381_pairing_base: NearGas.int().gte(0),
  bls12381_pairing_element: NearGas.int().gte(0),
  contract_compile_base: NearGas.int().gte(0),
  contract_compile_bytes: NearGas.int().gte(0),
  contract_loading_base: NearGas,
  contract_loading_bytes: NearGas,
  ecrecover_base: NearGas,
  ed25519_verify_base: NearGas,
  ed25519_verify_byte: NearGas,
  keccak256_base: NearGas,
  keccak256_byte: NearGas,
  keccak512_base: NearGas,
  keccak512_byte: NearGas,
  log_base: NearGas,
  log_byte: NearGas,
  promise_and_base: NearGas,
  promise_and_per_promise: NearGas,
  promise_return: NearGas,
  read_cached_trie_node: NearGas,
  read_memory_base: NearGas,
  read_memory_byte: NearGas,
  read_register_base: NearGas,
  read_register_byte: NearGas,
  ripemd160_base: NearGas,
  ripemd160_block: NearGas,
  sha256_base: NearGas,
  sha256_byte: NearGas,
  storage_has_key_base: NearGas,
  storage_has_key_byte: NearGas,
  storage_iter_create_from_byte: NearGas,
  storage_iter_create_prefix_base: NearGas,
  storage_iter_create_prefix_byte: NearGas,
  storage_iter_create_range_base: NearGas,
  storage_iter_create_to_byte: NearGas,
  storage_iter_next_base: NearGas,
  storage_iter_next_key_byte: NearGas,
  storage_iter_next_value_byte: NearGas,
  storage_large_read_overhead_base: NearGas,
  storage_large_read_overhead_byte: NearGas,
  storage_read_base: NearGas,
  storage_read_key_byte: NearGas,
  storage_read_value_byte: NearGas,
  storage_remove_base: NearGas,
  storage_remove_key_byte: NearGas,
  storage_remove_ret_value_byte: NearGas,
  storage_write_base: NearGas,
  storage_write_evicted_byte: NearGas,
  storage_write_key_byte: NearGas,
  storage_write_value_byte: NearGas,
  touching_trie_node: NearGas,
  utf16_decoding_base: NearGas,
  utf16_decoding_byte: NearGas,
  utf8_decoding_base: NearGas,
  utf8_decoding_byte: NearGas,
  validator_stake_base: NearGas,
  validator_total_stake_base: NearGas,
  write_memory_base: NearGas,
  write_memory_byte: NearGas,
  write_register_base: NearGas,
  write_register_byte: NearGas,
  yield_create_base: NearGas,
  yield_create_byte: NearGas,
  yield_resume_base: NearGas,
  yield_resume_byte: NearGas,
});
const ExternalStorageConfig = z.object({
  external_storage_fallback_threshold: z.number().int().gte(0).optional(),
  location: ExternalStorageLocation,
  num_concurrent_requests: z.number().int().gte(0).lte(255).optional(),
  num_concurrent_requests_during_catchup: z
    .number()
    .int()
    .gte(0)
    .lte(255)
    .optional(),
});
const FinalExecutionStatus = z.union([
  z.literal("NotStarted"),
  z.literal("Started"),
  z.object({ Failure: TxExecutionError }),
  z.object({ SuccessValue: z.string() }),
]);
const SignedTransactionView = z.object({
  actions: z.array(ActionView),
  hash: CryptoHash,
  nonce: z.number().int().gte(0),
  nonce_index: z.union([z.number(), z.null()]).optional(),
  priority_fee: z.number().int().gte(0).optional(),
  public_key: PublicKey,
  receiver_id: AccountId,
  signature: Signature,
  signer_id: AccountId,
});
const FinalExecutionOutcomeView = z.object({
  receipts_outcome: z.array(ExecutionOutcomeWithIdView),
  status: FinalExecutionStatus,
  transaction: SignedTransactionView,
  transaction_outcome: ExecutionOutcomeWithIdView,
});
const ReceiptEnumView = z.union([
  z.object({
    Action: z.object({
      actions: z.array(ActionView),
      gas_price: NearToken,
      input_data_ids: z.array(CryptoHash),
      is_promise_yield: z.boolean().optional(),
      output_data_receivers: z.array(DataReceiverView),
      refund_to: z.union([AccountId, z.null()]).optional(),
      signer_id: AccountId,
      signer_public_key: PublicKey,
    }),
  }),
  z.object({
    Data: z.object({
      data: z.union([z.string(), z.null()]).optional(),
      data_id: CryptoHash,
      is_promise_resume: z.boolean().optional(),
    }),
  }),
  z.object({
    GlobalContractDistribution: z.object({
      already_delivered_shards: z.array(ShardId),
      code: z.string(),
      id: GlobalContractIdentifier,
      target_shard: ShardId.int().gte(0),
    }),
  }),
]);
const ReceiptView = z.object({
  predecessor_id: AccountId,
  priority: z.number().int().gte(0).optional(),
  receipt: ReceiptEnumView,
  receipt_id: CryptoHash,
  receiver_id: AccountId,
});
const FinalExecutionOutcomeWithReceiptView = z.object({
  receipts: z.array(ReceiptView),
  receipts_outcome: z.array(ExecutionOutcomeWithIdView),
  status: FinalExecutionStatus,
  transaction: SignedTransactionView,
  transaction_outcome: ExecutionOutcomeWithIdView,
});
const GCConfig = z
  .object({
    gc_blocks_limit: z.number().int().gte(0),
    gc_fork_clean_step: z.number().int().gte(0),
    gc_num_epochs_to_keep: z.number().int().gte(0),
    gc_step_period: DurationAsStdSchemaProvider,
  })
  .partial();
const ShardLayoutV0 = z.object({
  num_shards: z.number().int().gte(0),
  version: z.number().int().gte(0),
});
const ShardLayoutV1 = z.object({
  boundary_accounts: z.array(AccountId),
  shards_split_map: z.union([z.array(z.array(ShardId)), z.null()]).optional(),
  to_parent_shard_map: z.union([z.array(ShardId), z.null()]).optional(),
  version: z.number().int().gte(0),
});
const ShardLayoutV2 = z.object({
  boundary_accounts: z.array(AccountId),
  id_to_index_map: z.record(z.number().int().gte(0)),
  index_to_id_map: z.record(ShardId),
  shard_ids: z.array(ShardId),
  shards_parent_map: z.union([z.record(ShardId), z.null()]).optional(),
  shards_split_map: z.union([z.record(z.array(ShardId)), z.null()]).optional(),
  version: z.number().int().gte(0),
});
const ShardLayoutV3 = z.object({
  boundary_accounts: z.array(AccountId),
  id_to_index_map: z.record(z.number().int().gte(0)),
  last_split: ShardId.int().gte(0),
  shard_ids: z.array(ShardId),
  shards_split_map: z.record(z.array(ShardId)),
});
const ShardLayout = z.union([
  z.object({ V0: ShardLayoutV0 }),
  z.object({ V1: ShardLayoutV1 }),
  z.object({ V2: ShardLayoutV2 }),
  z.object({ V3: ShardLayoutV3 }),
]);
const GenesisConfig = z.object({
  avg_hidden_validator_seats_per_shard: z.array(z.number().int().gte(0)),
  block_producer_kickout_threshold: z.number().int().gte(0).lte(255),
  chain_id: z.string(),
  chunk_producer_assignment_changes_limit: z.number().int().gte(0).optional(),
  chunk_producer_kickout_threshold: z.number().int().gte(0).lte(255),
  chunk_validator_only_kickout_threshold: z
    .number()
    .int()
    .gte(0)
    .lte(255)
    .optional(),
  dynamic_resharding: z.boolean(),
  epoch_length: z.number().int().gte(0),
  fishermen_threshold: NearToken,
  gas_limit: NearGas,
  gas_price_adjustment_rate: z.array(z.number().int()).min(2).max(2),
  genesis_height: z.number().int().gte(0),
  genesis_time: z.string().datetime({ offset: true }),
  max_gas_price: NearToken,
  max_inflation_rate: z.array(z.number().int()).min(2).max(2),
  max_kickout_stake_perc: z.number().int().gte(0).lte(255).optional(),
  min_gas_price: NearToken,
  minimum_stake_divisor: z.number().int().gte(0).optional(),
  minimum_stake_ratio: z.array(z.number().int()).min(2).max(2).optional(),
  minimum_validators_per_shard: z.number().int().gte(0).optional(),
  num_block_producer_seats: z.number().int().gte(0),
  num_block_producer_seats_per_shard: z.array(z.number().int().gte(0)),
  num_blocks_per_year: z.number().int().gte(0),
  num_chunk_only_producer_seats: z.number().int().gte(0).optional(),
  num_chunk_producer_seats: z.number().int().gte(0).optional(),
  num_chunk_validator_seats: z.number().int().gte(0).optional(),
  online_max_threshold: z.array(z.number().int()).min(2).max(2).optional(),
  online_min_threshold: z.array(z.number().int()).min(2).max(2).optional(),
  protocol_reward_rate: z.array(z.number().int()).min(2).max(2),
  protocol_treasury_account: AccountId,
  protocol_upgrade_stake_threshold: z
    .array(z.number().int())
    .min(2)
    .max(2)
    .optional(),
  protocol_version: z.number().int().gte(0),
  shard_layout: ShardLayout.optional(),
  shuffle_shard_assignment_for_chunk_producers: z.boolean().optional(),
  target_validator_mandates_per_shard: z.number().int().gte(0).optional(),
  total_supply: NearToken,
  transaction_validity_period: z.number().int().gte(0),
  use_production_config: z.boolean().optional(),
  validators: z.array(AccountInfo),
});
const GenesisConfigRequest = z.object({}).partial();
const LightClientBlockLiteView = z.object({
  inner_lite: BlockHeaderInnerLiteView,
  inner_rest_hash: CryptoHash,
  prev_block_hash: CryptoHash,
});
const LimitConfig = z.object({
  account_id_validity_rules_version: AccountIdValidityRulesVersion.optional(),
  initial_memory_pages: z.number().int().gte(0),
  max_actions_per_receipt: z.number().int().gte(0),
  max_arguments_length: z.number().int().gte(0),
  max_contract_size: z.number().int().gte(0),
  max_elements_per_contract_table: z.union([z.number(), z.null()]).optional(),
  max_functions_number_per_contract: z.union([z.number(), z.null()]).optional(),
  max_gas_burnt: NearGas,
  max_length_method_name: z.number().int().gte(0),
  max_length_returned_data: z.number().int().gte(0),
  max_length_storage_key: z.number().int().gte(0),
  max_length_storage_value: z.number().int().gte(0),
  max_locals_per_contract: z.union([z.number(), z.null()]).optional(),
  max_memory_pages: z.number().int().gte(0),
  max_number_bytes_method_names: z.number().int().gte(0),
  max_number_input_data_dependencies: z.number().int().gte(0),
  max_number_logs: z.number().int().gte(0),
  max_number_registers: z.number().int().gte(0),
  max_promises_per_function_call_action: z.number().int().gte(0),
  max_receipt_size: z.number().int().gte(0),
  max_register_size: z.number().int().gte(0),
  max_stack_height: z.number().int().gte(0),
  max_tables_per_contract: z.union([z.number(), z.null()]).optional(),
  max_total_log_length: z.number().int().gte(0),
  max_total_prepaid_gas: NearGas,
  max_transaction_size: z.number().int().gte(0),
  max_yield_payload_size: z.number().int().gte(0),
  per_receipt_storage_proof_size_limit: z.number().int().gte(0),
  registers_memory_limit: z.number().int().gte(0),
  yield_timeout_length_in_blocks: z.number().int().gte(0),
});
const LogSummaryStyle = z.enum(["plain", "colored"]);
const MutableConfigValue = z.string();
const NextEpochValidatorInfo = z.object({
  account_id: AccountId,
  public_key: PublicKey,
  shards: z.array(ShardId),
  stake: NearToken,
});
const PeerId = PublicKey;
const ProtocolVersionCheckConfig = z.enum(["Next", "NextNext"]);
const StoreKey = z.string();
const QueryRequest = z.union([
  z.object({ account_id: AccountId, request_type: z.literal("view_account") }),
  z.object({ account_id: AccountId, request_type: z.literal("view_code") }),
  z.object({
    account_id: AccountId,
    include_proof: z.boolean().optional(),
    prefix_base64: StoreKey,
    request_type: z.literal("view_state"),
  }),
  z.object({
    account_id: AccountId,
    public_key: PublicKey,
    request_type: z.literal("view_access_key"),
  }),
  z.object({
    account_id: AccountId,
    request_type: z.literal("view_access_key_list"),
  }),
  z.object({
    account_id: AccountId,
    public_key: PublicKey,
    request_type: z.literal("view_gas_key_nonces"),
  }),
  z.object({
    account_id: AccountId,
    args_base64: FunctionArgs,
    method_name: z.string(),
    request_type: z.literal("call_function"),
  }),
  z.object({
    code_hash: CryptoHash,
    request_type: z.literal("view_global_contract_code"),
  }),
  z.object({
    account_id: AccountId,
    request_type: z.literal("view_global_contract_code_by_account_id"),
  }),
]);
const RpcBlockRequest = z.union([
  z.object({ block_id: BlockId }),
  z.object({ finality: Finality }),
  z.object({ sync_checkpoint: SyncCheckpoint }),
]);
const RpcBlockResponse = z.object({
  author: AccountId,
  chunks: z.array(ChunkHeaderView),
  header: BlockHeaderView,
});
const RpcChunkRequest = z.union([
  z.object({ block_id: BlockId, shard_id: ShardId.int().gte(0) }),
  z.object({ chunk_id: CryptoHash }),
]);
const RpcChunkResponse = z.object({
  author: AccountId,
  header: ChunkHeaderView,
  receipts: z.array(ReceiptView),
  transactions: z.array(SignedTransactionView),
});
const RpcClientConfigRequest = z.object({}).partial();
const SyncConcurrency = z.object({
  apply: z.number().int().gte(0).lte(255),
  apply_during_catchup: z.number().int().gte(0).lte(255),
  peer_downloads: z.number().int().gte(0).lte(255),
  per_shard: z.number().int().gte(0).lte(255),
});
const SyncConfig = z.union([
  z.literal("Peers"),
  z.object({ ExternalStorage: ExternalStorageConfig }),
]);
const StateSyncConfig = z
  .object({
    concurrency: SyncConcurrency,
    dump: z.union([DumpConfig, z.null()]),
    parts_compression_lvl: z.number().int(),
    sync: SyncConfig,
  })
  .partial();
const ShardUId = z.object({
  shard_id: z.number().int().gte(0),
  version: z.number().int().gte(0),
});
const TrackedShardsConfig = z.union([
  z.literal("NoShards"),
  z.object({ Shards: z.array(ShardUId) }),
  z.literal("AllShards"),
  z.object({ ShadowValidator: AccountId }),
  z.object({ Schedule: z.array(z.array(ShardId)) }),
  z.object({ Accounts: z.array(AccountId) }),
]);
const Version = z.object({
  build: z.string(),
  commit: z.string(),
  rustc_version: z.string().optional(),
  version: z.string(),
});
const RpcClientConfigResponse = z.object({
  archive: z.boolean(),
  block_fetch_horizon: z.number().int().gte(0),
  block_header_fetch_horizon: z.number().int().gte(0),
  block_production_tracking_delay: z
    .array(z.number().int().gte(0))
    .min(2)
    .max(2),
  catchup_step_period: z.array(z.number().int().gte(0)).min(2).max(2),
  chain_id: z.string(),
  chunk_distribution_network: z
    .union([ChunkDistributionNetworkConfig, z.null()])
    .optional(),
  chunk_request_retry_period: z.array(z.number().int().gte(0)).min(2).max(2),
  chunk_validation_threads: z.number().int().gte(0),
  chunk_wait_mult: z.array(z.number().int()).min(2).max(2),
  chunks_cache_height_horizon: z.number().int().gte(0),
  client_background_migration_threads: z.number().int().gte(0),
  cloud_archival_writer: z
    .union([CloudArchivalWriterConfig, z.null()])
    .optional(),
  disable_tx_routing: z.boolean(),
  doomslug_step_period: z.array(z.number().int().gte(0)).min(2).max(2),
  enable_early_prepare_transactions: z.boolean(),
  enable_multiline_logging: z.boolean(),
  enable_statistics_export: z.boolean(),
  epoch_length: z.number().int().gte(0),
  epoch_sync: EpochSyncConfig,
  expected_shutdown: MutableConfigValue,
  gc: GCConfig,
  header_sync_expected_height_per_second: z.number().int().gte(0),
  header_sync_initial_timeout: z.array(z.number().int().gte(0)).min(2).max(2),
  header_sync_progress_timeout: z.array(z.number().int().gte(0)).min(2).max(2),
  header_sync_stall_ban_timeout: z.array(z.number().int().gte(0)).min(2).max(2),
  log_summary_period: z.array(z.number().int().gte(0)).min(2).max(2),
  log_summary_style: LogSummaryStyle,
  max_block_production_delay: z.array(z.number().int().gte(0)).min(2).max(2),
  max_block_wait_delay: z.array(z.number().int().gte(0)).min(2).max(2),
  max_gas_burnt_view: z.union([NearGas, z.null()]).optional(),
  min_block_production_delay: z.array(z.number().int().gte(0)).min(2).max(2),
  min_num_peers: z.number().int().gte(0),
  num_block_producer_seats: z.number().int().gte(0),
  orphan_state_witness_max_size: z.number().int().gte(0),
  orphan_state_witness_pool_size: z.number().int().gte(0),
  produce_chunk_add_transactions_time_limit: z.string(),
  produce_empty_blocks: z.boolean(),
  protocol_version_check: ProtocolVersionCheckConfig,
  resharding_config: MutableConfigValue,
  rpc_addr: z.union([z.string(), z.null()]).optional(),
  save_invalid_witnesses: z.boolean(),
  save_latest_witnesses: z.boolean(),
  save_state_changes: z.boolean(),
  save_trie_changes: z.boolean(),
  save_tx_outcomes: z.boolean(),
  save_untracked_partial_chunks_parts: z.boolean(),
  skip_sync_wait: z.boolean(),
  state_request_server_threads: z.number().int().gte(0),
  state_request_throttle_period: z.array(z.number().int().gte(0)).min(2).max(2),
  state_requests_per_throttle_period: z.number().int().gte(0),
  state_sync: StateSyncConfig,
  state_sync_enabled: z.boolean(),
  state_sync_external_backoff: z.array(z.number().int().gte(0)).min(2).max(2),
  state_sync_external_timeout: z.array(z.number().int().gte(0)).min(2).max(2),
  state_sync_p2p_timeout: z.array(z.number().int().gte(0)).min(2).max(2),
  state_sync_retry_backoff: z.array(z.number().int().gte(0)).min(2).max(2),
  sync_check_period: z.array(z.number().int().gte(0)).min(2).max(2),
  sync_height_threshold: z.number().int().gte(0),
  sync_max_block_requests: z.number().int().gte(0),
  sync_step_period: z.array(z.number().int().gte(0)).min(2).max(2),
  tracked_shards_config: TrackedShardsConfig,
  transaction_pool_size_limit: z.union([z.number(), z.null()]).optional(),
  transaction_request_handler_threads: z.number().int().gte(0),
  trie_viewer_state_size_limit: z.union([z.number(), z.null()]).optional(),
  ttl_account_id_router: z.array(z.number().int().gte(0)).min(2).max(2),
  tx_routing_height_horizon: z.number().int().gte(0),
  version: Version,
  view_client_threads: z.number().int().gte(0),
});
const RpcCongestionLevelRequest = z.union([
  z.object({ block_id: BlockId, shard_id: ShardId.int().gte(0) }),
  z.object({ chunk_id: CryptoHash }),
]);
const RpcCongestionLevelResponse = z.object({ congestion_level: z.number() });
const RpcGasPriceRequest = z
  .object({ block_id: z.union([BlockId, z.null()]) })
  .partial();
const RpcGasPriceResponse = z.object({ gas_price: NearToken });
const RpcHealthRequest = z.object({}).partial();
const RpcHealthResponse = z.null();
const RpcKnownProducer = z.object({
  account_id: AccountId,
  addr: z.union([z.string(), z.null()]).optional(),
  peer_id: PeerId,
});
const RpcLightClientBlockProofRequest = z.object({
  block_hash: CryptoHash,
  light_client_head: CryptoHash,
});
const RpcLightClientBlockProofResponse = z.object({
  block_header_lite: LightClientBlockLiteView,
  block_proof: z.array(MerklePathItem),
});
const RpcLightClientExecutionProofRequest = z.union([
  z.object({
    light_client_head: CryptoHash,
    sender_id: AccountId,
    transaction_hash: CryptoHash,
    type: z.literal("transaction"),
  }),
  z.object({
    light_client_head: CryptoHash,
    receipt_id: CryptoHash,
    receiver_id: AccountId,
    type: z.literal("receipt"),
  }),
]);
const RpcLightClientExecutionProofResponse = z.object({
  block_header_lite: LightClientBlockLiteView,
  block_proof: z.array(MerklePathItem),
  outcome_proof: ExecutionOutcomeWithIdView,
  outcome_root_proof: z.array(MerklePathItem),
});
const RpcLightClientNextBlockRequest = z.object({
  last_block_hash: CryptoHash,
});
const RpcLightClientNextBlockResponse = z
  .object({
    approvals_after_next: z.array(z.union([Signature, z.null()])),
    inner_lite: BlockHeaderInnerLiteView,
    inner_rest_hash: CryptoHash,
    next_block_inner_hash: CryptoHash,
    next_bps: z.union([z.array(ValidatorStakeView), z.null()]),
    prev_block_hash: CryptoHash,
  })
  .partial();
const RpcMaintenanceWindowsRequest = z.object({ account_id: AccountId });
const RpcNetworkInfoRequest = z.object({}).partial();
const RpcPeerInfo = z.object({
  account_id: z.union([AccountId, z.null()]).optional(),
  addr: z.union([z.string(), z.null()]).optional(),
  id: PeerId,
});
const RpcNetworkInfoResponse = z.object({
  active_peers: z.array(RpcPeerInfo),
  known_producers: z.array(RpcKnownProducer),
  num_active_peers: z.number().int().gte(0),
  peer_max_count: z.number().int().gte(0),
  received_bytes_per_sec: z.number().int().gte(0),
  sent_bytes_per_sec: z.number().int().gte(0),
});
const RpcProtocolConfigRequest = z.union([
  z.object({ block_id: BlockId }),
  z.object({ finality: Finality }),
  z.object({ sync_checkpoint: SyncCheckpoint }),
]);
const StorageUsageConfigView = z.object({
  num_bytes_account: z.number().int().gte(0),
  num_extra_bytes_record: z.number().int().gte(0),
});
const RuntimeFeesConfigView = z.object({
  action_creation_config: ActionCreationConfigView,
  action_receipt_creation_config: Fee,
  burnt_gas_reward: z.array(z.number().int()).min(2).max(2),
  data_receipt_creation_config: DataReceiptCreationConfigView,
  pessimistic_gas_price_inflation_ratio: z
    .array(z.number().int())
    .min(2)
    .max(2),
  storage_usage_config: StorageUsageConfigView,
});
const StorageGetMode = z.enum(["FlatStorage", "Trie"]);
const VMKind = z.union([
  z.literal("Wasmer0"),
  z.literal("Wasmtime"),
  z.literal("Wasmer2"),
  z.literal("NearVm"),
]);
const VMConfigView = z.object({
  deterministic_account_ids: z.boolean(),
  discard_custom_sections: z.boolean(),
  eth_implicit_accounts: z.boolean(),
  ext_costs: ExtCostsConfigView,
  fix_contract_loading_cost: z.boolean(),
  global_contract_host_fns: z.boolean(),
  grow_mem_cost: z.number().int().gte(0),
  implicit_account_creation: z.boolean(),
  limit_config: LimitConfig,
  linear_op_base_cost: z.number().int().gte(0),
  linear_op_unit_cost: z.number().int().gte(0),
  reftypes_bulk_memory: z.boolean(),
  regular_op_cost: z.number().int().gte(0),
  storage_get_mode: StorageGetMode,
  vm_kind: VMKind,
});
const WitnessConfigView = z.object({
  combined_transactions_size_limit: z.number().int().gte(0),
  main_storage_proof_size_soft_limit: z.number().int().gte(0),
  new_transactions_validation_state_size_soft_limit: z.number().int().gte(0),
});
const RuntimeConfigView = z.object({
  account_creation_config: AccountCreationConfigView,
  congestion_control_config: CongestionControlConfigView,
  storage_amount_per_byte: NearToken,
  transaction_costs: RuntimeFeesConfigView,
  wasm_config: VMConfigView,
  witness_config: WitnessConfigView,
});
const RpcProtocolConfigResponse = z.object({
  avg_hidden_validator_seats_per_shard: z.array(z.number().int().gte(0)),
  block_producer_kickout_threshold: z.number().int().gte(0).lte(255),
  chain_id: z.string(),
  chunk_producer_kickout_threshold: z.number().int().gte(0).lte(255),
  chunk_validator_only_kickout_threshold: z.number().int().gte(0).lte(255),
  dynamic_resharding: z.boolean(),
  epoch_length: z.number().int().gte(0),
  fishermen_threshold: NearToken,
  gas_limit: NearGas,
  gas_price_adjustment_rate: z.array(z.number().int()).min(2).max(2),
  genesis_height: z.number().int().gte(0),
  genesis_time: z.string().datetime({ offset: true }),
  max_gas_price: NearToken,
  max_inflation_rate: z.array(z.number().int()).min(2).max(2),
  max_kickout_stake_perc: z.number().int().gte(0).lte(255),
  min_gas_price: NearToken,
  minimum_stake_divisor: z.number().int().gte(0),
  minimum_stake_ratio: z.array(z.number().int()).min(2).max(2),
  minimum_validators_per_shard: z.number().int().gte(0),
  num_block_producer_seats: z.number().int().gte(0),
  num_block_producer_seats_per_shard: z.array(z.number().int().gte(0)),
  num_blocks_per_year: z.number().int().gte(0),
  online_max_threshold: z.array(z.number().int()).min(2).max(2),
  online_min_threshold: z.array(z.number().int()).min(2).max(2),
  protocol_reward_rate: z.array(z.number().int()).min(2).max(2),
  protocol_treasury_account: AccountId,
  protocol_upgrade_stake_threshold: z.array(z.number().int()).min(2).max(2),
  protocol_version: z.number().int().gte(0),
  runtime_config: RuntimeConfigView,
  shard_layout: ShardLayout,
  shuffle_shard_assignment_for_chunk_producers: z.boolean(),
  target_validator_mandates_per_shard: z.number().int().gte(0),
  transaction_validity_period: z.number().int().gte(0),
});
const RpcQueryRequest = BlockReference.and(QueryRequest);
const StoreValue = z.string();
const StateItem = z.object({ key: StoreKey, value: StoreValue });
const ViewStateResult = z.object({
  proof: z.array(z.string()).optional(),
  values: z.array(StateItem),
});
const RpcQueryResponse = z.union([
  z
    .object({ block_hash: CryptoHash, block_height: z.number().int().gte(0) })
    .and(AccountView),
  z
    .object({ block_hash: CryptoHash, block_height: z.number().int().gte(0) })
    .and(ContractCodeView),
  z
    .object({ block_hash: CryptoHash, block_height: z.number().int().gte(0) })
    .and(ViewStateResult),
  z
    .object({ block_hash: CryptoHash, block_height: z.number().int().gte(0) })
    .and(CallResult),
  z
    .object({ block_hash: CryptoHash, block_height: z.number().int().gte(0) })
    .and(AccessKeyView),
  z
    .object({ block_hash: CryptoHash, block_height: z.number().int().gte(0) })
    .and(AccessKeyList),
  z
    .object({ block_hash: CryptoHash, block_height: z.number().int().gte(0) })
    .and(z.array(z.number().int().gte(0))),
]);
const RpcReceiptRequest = z.object({ receipt_id: CryptoHash });
const RpcReceiptResponse = z.object({
  predecessor_id: AccountId,
  priority: z.number().int().gte(0).optional(),
  receipt: ReceiptEnumView,
  receipt_id: CryptoHash,
  receiver_id: AccountId,
});
const SignedTransaction = z.string();
const TxExecutionStatus = z.union([
  z.literal("NONE"),
  z.literal("INCLUDED"),
  z.literal("EXECUTED_OPTIMISTIC"),
  z.literal("INCLUDED_FINAL"),
  z.literal("EXECUTED"),
  z.literal("FINAL"),
]);
const RpcSendTransactionRequest = z.object({
  signed_tx_base64: SignedTransaction,
  wait_until: TxExecutionStatus.optional(),
});
const RpcSplitStorageInfoRequest = z.object({}).partial();
const RpcSplitStorageInfoResponse = z
  .object({
    cold_head_height: z.union([z.number(), z.null()]),
    final_head_height: z.union([z.number(), z.null()]),
    head_height: z.union([z.number(), z.null()]),
    hot_db_kind: z.union([z.string(), z.null()]),
  })
  .partial();
const StateChangesRequestView = z.union([
  z.object({
    account_ids: z.array(AccountId),
    changes_type: z.literal("account_changes"),
  }),
  z.object({
    changes_type: z.literal("single_access_key_changes"),
    keys: z.array(AccountWithPublicKey),
  }),
  z.object({
    account_ids: z.array(AccountId),
    changes_type: z.literal("all_access_key_changes"),
  }),
  z.object({
    account_ids: z.array(AccountId),
    changes_type: z.literal("contract_code_changes"),
  }),
  z.object({
    account_ids: z.array(AccountId),
    changes_type: z.literal("data_changes"),
    key_prefix_base64: StoreKey,
  }),
]);
const RpcStateChangesInBlockByTypeRequest = BlockReference.and(
  StateChangesRequestView
);
const StateChangeKindView = z.union([
  z.object({ account_id: AccountId, type: z.literal("account_touched") }),
  z.object({ account_id: AccountId, type: z.literal("access_key_touched") }),
  z.object({ account_id: AccountId, type: z.literal("data_touched") }),
  z.object({ account_id: AccountId, type: z.literal("contract_code_touched") }),
]);
const RpcStateChangesInBlockByTypeResponse = z.object({
  block_hash: CryptoHash,
  changes: z.array(StateChangeKindView),
});
const RpcStateChangesInBlockRequest = z.union([
  z.object({ block_id: BlockId }),
  z.object({ finality: Finality }),
  z.object({ sync_checkpoint: SyncCheckpoint }),
]);
const StateChangeCauseView = z.union([
  z.object({ type: z.literal("not_writable_to_disk") }),
  z.object({ type: z.literal("initial_state") }),
  z.object({ tx_hash: CryptoHash, type: z.literal("transaction_processing") }),
  z.object({
    receipt_hash: CryptoHash,
    type: z.literal("action_receipt_processing_started"),
  }),
  z.object({
    receipt_hash: CryptoHash,
    type: z.literal("action_receipt_gas_reward"),
  }),
  z.object({ receipt_hash: CryptoHash, type: z.literal("receipt_processing") }),
  z.object({ receipt_hash: CryptoHash, type: z.literal("postponed_receipt") }),
  z.object({ type: z.literal("updated_delayed_receipts") }),
  z.object({ type: z.literal("validator_accounts_update") }),
  z.object({ type: z.literal("migration") }),
  z.object({ type: z.literal("bandwidth_scheduler_state_update") }),
]);
const StateChangeWithCauseView = z.union([
  z.object({
    cause: StateChangeCauseView,
    change: z.object({
      account_id: AccountId,
      amount: NearToken,
      code_hash: CryptoHash,
      global_contract_account_id: z.union([AccountId, z.null()]).optional(),
      global_contract_hash: z.union([CryptoHash, z.null()]).optional(),
      locked: NearToken,
      storage_paid_at: z.number().int().gte(0).optional(),
      storage_usage: z.number().int().gte(0),
    }),
    type: z.literal("account_update"),
  }),
  z.object({
    cause: StateChangeCauseView,
    change: z.object({ account_id: AccountId }),
    type: z.literal("account_deletion"),
  }),
  z.object({
    cause: StateChangeCauseView,
    change: z.object({
      access_key: AccessKeyView,
      account_id: AccountId,
      public_key: PublicKey,
    }),
    type: z.literal("access_key_update"),
  }),
  z.object({
    cause: StateChangeCauseView,
    change: z.object({ account_id: AccountId, public_key: PublicKey }),
    type: z.literal("access_key_deletion"),
  }),
  z.object({
    cause: StateChangeCauseView,
    change: z.object({
      account_id: AccountId,
      index: z.number().int().gte(0).lte(65535),
      nonce: z.number().int().gte(0),
      public_key: PublicKey,
    }),
    type: z.literal("gas_key_nonce_update"),
  }),
  z.object({
    cause: StateChangeCauseView,
    change: z.object({
      account_id: AccountId,
      key_base64: StoreKey,
      value_base64: StoreValue,
    }),
    type: z.literal("data_update"),
  }),
  z.object({
    cause: StateChangeCauseView,
    change: z.object({ account_id: AccountId, key_base64: StoreKey }),
    type: z.literal("data_deletion"),
  }),
  z.object({
    cause: StateChangeCauseView,
    change: z.object({ account_id: AccountId, code_base64: z.string() }),
    type: z.literal("contract_code_update"),
  }),
  z.object({
    cause: StateChangeCauseView,
    change: z.object({ account_id: AccountId }),
    type: z.literal("contract_code_deletion"),
  }),
]);
const RpcStateChangesInBlockResponse = z.object({
  block_hash: CryptoHash,
  changes: z.array(StateChangeWithCauseView),
});
const RpcStatusRequest = z.object({}).partial();
const StatusSyncInfo = z.object({
  earliest_block_hash: z.union([CryptoHash, z.null()]).optional(),
  earliest_block_height: z.union([z.number(), z.null()]).optional(),
  earliest_block_time: z.union([z.string(), z.null()]).optional(),
  epoch_id: z.union([EpochId, z.null()]).optional(),
  epoch_start_height: z.union([z.number(), z.null()]).optional(),
  latest_block_hash: CryptoHash,
  latest_block_height: z.number().int().gte(0),
  latest_block_time: z.string(),
  latest_state_root: CryptoHash,
  syncing: z.boolean(),
});
const ValidatorInfo = z.object({ account_id: AccountId });
const RpcStatusResponse = z.object({
  chain_id: z.string(),
  detailed_debug_status: z.union([DetailedDebugStatus, z.null()]).optional(),
  genesis_hash: CryptoHash,
  latest_protocol_version: z.number().int().gte(0),
  node_key: z.union([PublicKey, z.null()]).optional(),
  node_public_key: PublicKey,
  protocol_version: z.number().int().gte(0),
  rpc_addr: z.union([z.string(), z.null()]).optional(),
  sync_info: StatusSyncInfo,
  uptime_sec: z.number().int(),
  validator_account_id: z.union([AccountId, z.null()]).optional(),
  validator_public_key: z.union([PublicKey, z.null()]).optional(),
  validators: z.array(ValidatorInfo),
  version: Version,
});
const RpcTransactionResponse = z.union([
  z
    .object({ final_execution_status: TxExecutionStatus })
    .and(FinalExecutionOutcomeWithReceiptView),
  z
    .object({ final_execution_status: TxExecutionStatus })
    .and(FinalExecutionOutcomeView),
  z.object({ final_execution_status: TxExecutionStatus }),
]);
const RpcTransactionStatusRequest = z.union([
  z
    .object({ wait_until: TxExecutionStatus })
    .partial()
    .and(z.object({ signed_tx_base64: SignedTransaction })),
  z
    .object({ wait_until: TxExecutionStatus })
    .partial()
    .and(z.object({ sender_account_id: AccountId, tx_hash: CryptoHash })),
]);
const RpcValidatorRequest = z.union([
  z.object({ epoch_id: EpochId }),
  z.object({ block_id: BlockId }),
  z.object({ latest: z.null() }),
]);
const ValidatorKickoutReason = z.union([
  z.literal("_UnusedSlashed"),
  z.object({
    NotEnoughBlocks: z.object({
      expected: z.number().int().gte(0),
      produced: z.number().int().gte(0),
    }),
  }),
  z.object({
    NotEnoughChunks: z.object({
      expected: z.number().int().gte(0),
      produced: z.number().int().gte(0),
    }),
  }),
  z.literal("Unstaked"),
  z.object({
    NotEnoughStake: z.object({
      stake_u128: NearToken,
      threshold_u128: NearToken,
    }),
  }),
  z.literal("DidNotGetASeat"),
  z.object({
    NotEnoughChunkEndorsements: z.object({
      expected: z.number().int().gte(0),
      produced: z.number().int().gte(0),
    }),
  }),
  z.object({
    ProtocolVersionTooOld: z.object({
      network_version: z.number().int().gte(0),
      version: z.number().int().gte(0),
    }),
  }),
]);
const ValidatorKickoutView = z.object({
  account_id: AccountId,
  reason: ValidatorKickoutReason,
});
const RpcValidatorResponse = z.object({
  current_fishermen: z.array(ValidatorStakeView),
  current_proposals: z.array(ValidatorStakeView),
  current_validators: z.array(CurrentEpochValidatorInfo),
  epoch_height: z.number().int().gte(0),
  epoch_start_height: z.number().int().gte(0),
  next_fishermen: z.array(ValidatorStakeView),
  next_validators: z.array(NextEpochValidatorInfo),
  prev_epoch_kickout: z.array(ValidatorKickoutView),
});
const RpcValidatorsOrderedRequest = z
  .object({ block_id: z.union([BlockId, z.null()]) })
  .partial();
const SignedDelegateAction = z.object({
  delegate_action: DelegateAction,
  signature: Signature,
});
const ValidatorStakeViews = z.array(ValidatorStakeView);


// Inferred types for cleaner IDE hover
export type AccessKey = Simplify<z.infer<typeof AccessKey>>;
export type AccessKeyCreationConfigView = Simplify<z.infer<typeof AccessKeyCreationConfigView>>;
export type AccessKeyInfoView = Simplify<z.infer<typeof AccessKeyInfoView>>;
export type AccessKeyList = Simplify<z.infer<typeof AccessKeyList>>;
export type AccessKeyPermission = Simplify<z.infer<typeof AccessKeyPermission>>;
export type AccessKeyPermissionView = Simplify<z.infer<typeof AccessKeyPermissionView>>;
export type AccessKeyView = Simplify<z.infer<typeof AccessKeyView>>;
export type AccountCreationConfigView = Simplify<z.infer<typeof AccountCreationConfigView>>;
export type AccountDataView = Simplify<z.infer<typeof AccountDataView>>;
export type AccountId = Simplify<z.infer<typeof AccountId>>;
export type AccountIdValidityRulesVersion = Simplify<z.infer<typeof AccountIdValidityRulesVersion>>;
export type AccountInfo = Simplify<z.infer<typeof AccountInfo>>;
export type AccountView = Simplify<z.infer<typeof AccountView>>;
export type AccountWithPublicKey = Simplify<z.infer<typeof AccountWithPublicKey>>;
export type ActionCreationConfigView = Simplify<z.infer<typeof ActionCreationConfigView>>;
export type ActionError = Simplify<z.infer<typeof ActionError>>;
export type ActionErrorKind = Simplify<z.infer<typeof ActionErrorKind>>;
export type ActionView = Simplify<z.infer<typeof ActionView>>;
export type ActionsValidationError = Simplify<z.infer<typeof ActionsValidationError>>;
export type AddKeyAction = Simplify<z.infer<typeof AddKeyAction>>;
export type BandwidthRequest = Simplify<z.infer<typeof BandwidthRequest>>;
export type BandwidthRequestBitmap = Simplify<z.infer<typeof BandwidthRequestBitmap>>;
export type BandwidthRequests = Simplify<z.infer<typeof BandwidthRequests>>;
export type BandwidthRequestsV1 = Simplify<z.infer<typeof BandwidthRequestsV1>>;
export type BlockHeaderInnerLiteView = Simplify<z.infer<typeof BlockHeaderInnerLiteView>>;
export type BlockHeaderView = Simplify<z.infer<typeof BlockHeaderView>>;
export type BlockHeightRange = Simplify<z.infer<typeof BlockHeightRange>>;
export type BlockHeightRanges = Simplify<z.infer<typeof BlockHeightRanges>>;
export type BlockId = Simplify<z.infer<typeof BlockId>>;
export type BlockReference = Simplify<z.infer<typeof BlockReference>>;
export type BlockStatusView = Simplify<z.infer<typeof BlockStatusView>>;
export type CallResult = Simplify<z.infer<typeof CallResult>>;
export type CatchupStatusView = Simplify<z.infer<typeof CatchupStatusView>>;
export type ChunkDistributionNetworkConfig = Simplify<z.infer<typeof ChunkDistributionNetworkConfig>>;
export type ChunkDistributionUris = Simplify<z.infer<typeof ChunkDistributionUris>>;
export type ChunkHeaderView = Simplify<z.infer<typeof ChunkHeaderView>>;
export type CloudArchivalWriterConfig = Simplify<z.infer<typeof CloudArchivalWriterConfig>>;
export type CompilationError = Simplify<z.infer<typeof CompilationError>>;
export type CongestionControlConfigView = Simplify<z.infer<typeof CongestionControlConfigView>>;
export type CongestionInfoView = Simplify<z.infer<typeof CongestionInfoView>>;
export type ContractCodeView = Simplify<z.infer<typeof ContractCodeView>>;
export type CostGasUsed = Simplify<z.infer<typeof CostGasUsed>>;
export type CreateAccountAction = Simplify<z.infer<typeof CreateAccountAction>>;
export type CryptoHash = Simplify<z.infer<typeof CryptoHash>>;
export type CurrentEpochValidatorInfo = Simplify<z.infer<typeof CurrentEpochValidatorInfo>>;
export type DataReceiptCreationConfigView = Simplify<z.infer<typeof DataReceiptCreationConfigView>>;
export type DataReceiverView = Simplify<z.infer<typeof DataReceiverView>>;
export type DelegateAction = Simplify<z.infer<typeof DelegateAction>>;
export type DeleteAccountAction = Simplify<z.infer<typeof DeleteAccountAction>>;
export type DeleteKeyAction = Simplify<z.infer<typeof DeleteKeyAction>>;
export type DeployContractAction = Simplify<z.infer<typeof DeployContractAction>>;
export type DeployGlobalContractAction = Simplify<z.infer<typeof DeployGlobalContractAction>>;
export type DetailedDebugStatus = Simplify<z.infer<typeof DetailedDebugStatus>>;
export type DeterministicAccountStateInit = Simplify<z.infer<typeof DeterministicAccountStateInit>>;
export type DeterministicAccountStateInitV1 = Simplify<z.infer<typeof DeterministicAccountStateInitV1>>;
export type DeterministicStateInitAction = Simplify<z.infer<typeof DeterministicStateInitAction>>;
export type Direction = Simplify<z.infer<typeof Direction>>;
export type DumpConfig = Simplify<z.infer<typeof DumpConfig>>;
export type DurationAsStdSchemaProvider = Simplify<z.infer<typeof DurationAsStdSchemaProvider>>;
export type EpochId = Simplify<z.infer<typeof EpochId>>;
export type EpochSyncConfig = Simplify<z.infer<typeof EpochSyncConfig>>;
export type ExecutionMetadataView = Simplify<z.infer<typeof ExecutionMetadataView>>;
export type ExecutionOutcomeView = Simplify<z.infer<typeof ExecutionOutcomeView>>;
export type ExecutionOutcomeWithIdView = Simplify<z.infer<typeof ExecutionOutcomeWithIdView>>;
export type ExecutionStatusView = Simplify<z.infer<typeof ExecutionStatusView>>;
export type ExtCostsConfigView = Simplify<z.infer<typeof ExtCostsConfigView>>;
export type ExternalStorageConfig = Simplify<z.infer<typeof ExternalStorageConfig>>;
export type ExternalStorageLocation = Simplify<z.infer<typeof ExternalStorageLocation>>;
export type Fee = Simplify<z.infer<typeof Fee>>;
export type FinalExecutionOutcomeView = Simplify<z.infer<typeof FinalExecutionOutcomeView>>;
export type FinalExecutionOutcomeWithReceiptView = Simplify<z.infer<typeof FinalExecutionOutcomeWithReceiptView>>;
export type FinalExecutionStatus = Simplify<z.infer<typeof FinalExecutionStatus>>;
export type Finality = Simplify<z.infer<typeof Finality>>;
export type FunctionArgs = Simplify<z.infer<typeof FunctionArgs>>;
export type FunctionCallAction = Simplify<z.infer<typeof FunctionCallAction>>;
export type FunctionCallError = Simplify<z.infer<typeof FunctionCallError>>;
export type FunctionCallPermission = Simplify<z.infer<typeof FunctionCallPermission>>;
export type GCConfig = Simplify<z.infer<typeof GCConfig>>;
export type GasKeyInfo = Simplify<z.infer<typeof GasKeyInfo>>;
export type GenesisConfig = Simplify<z.infer<typeof GenesisConfig>>;
export type GenesisConfigRequest = Simplify<z.infer<typeof GenesisConfigRequest>>;
export type GlobalContractDeployMode = Simplify<z.infer<typeof GlobalContractDeployMode>>;
export type GlobalContractIdentifier = Simplify<z.infer<typeof GlobalContractIdentifier>>;
export type GlobalContractIdentifierView = Simplify<z.infer<typeof GlobalContractIdentifierView>>;
export type HostError = Simplify<z.infer<typeof HostError>>;
export type InvalidAccessKeyError = Simplify<z.infer<typeof InvalidAccessKeyError>>;
export type InvalidTxError = Simplify<z.infer<typeof InvalidTxError>>;
export type KnownProducerView = Simplify<z.infer<typeof KnownProducerView>>;
export type LightClientBlockLiteView = Simplify<z.infer<typeof LightClientBlockLiteView>>;
export type LimitConfig = Simplify<z.infer<typeof LimitConfig>>;
export type LogSummaryStyle = Simplify<z.infer<typeof LogSummaryStyle>>;
export type MerklePathItem = Simplify<z.infer<typeof MerklePathItem>>;
export type MethodResolveError = Simplify<z.infer<typeof MethodResolveError>>;
export type MissingTrieValue = Simplify<z.infer<typeof MissingTrieValue>>;
export type MissingTrieValueContext = Simplify<z.infer<typeof MissingTrieValueContext>>;
export type MutableConfigValue = Simplify<z.infer<typeof MutableConfigValue>>;
export type NearGas = Simplify<z.infer<typeof NearGas>>;
export type NearToken = Simplify<z.infer<typeof NearToken>>;
export type NetworkInfoView = Simplify<z.infer<typeof NetworkInfoView>>;
export type NextEpochValidatorInfo = Simplify<z.infer<typeof NextEpochValidatorInfo>>;
export type NonDelegateAction = Simplify<z.infer<typeof NonDelegateAction>>;
export type PeerId = Simplify<z.infer<typeof PeerId>>;
export type PeerInfoView = Simplify<z.infer<typeof PeerInfoView>>;
export type PrepareError = Simplify<z.infer<typeof PrepareError>>;
export type ProtocolVersionCheckConfig = Simplify<z.infer<typeof ProtocolVersionCheckConfig>>;
export type PublicKey = Simplify<z.infer<typeof PublicKey>>;
export type QueryRequest = Simplify<z.infer<typeof QueryRequest>>;
export type ReceiptEnumView = Simplify<z.infer<typeof ReceiptEnumView>>;
export type ReceiptValidationError = Simplify<z.infer<typeof ReceiptValidationError>>;
export type ReceiptView = Simplify<z.infer<typeof ReceiptView>>;
export type RpcBlockRequest = Simplify<z.infer<typeof RpcBlockRequest>>;
export type RpcBlockResponse = Simplify<z.infer<typeof RpcBlockResponse>>;
export type RpcChunkRequest = Simplify<z.infer<typeof RpcChunkRequest>>;
export type RpcChunkResponse = Simplify<z.infer<typeof RpcChunkResponse>>;
export type RpcClientConfigRequest = Simplify<z.infer<typeof RpcClientConfigRequest>>;
export type RpcClientConfigResponse = Simplify<z.infer<typeof RpcClientConfigResponse>>;
export type RpcCongestionLevelRequest = Simplify<z.infer<typeof RpcCongestionLevelRequest>>;
export type RpcCongestionLevelResponse = Simplify<z.infer<typeof RpcCongestionLevelResponse>>;
export type RpcGasPriceRequest = Simplify<z.infer<typeof RpcGasPriceRequest>>;
export type RpcGasPriceResponse = Simplify<z.infer<typeof RpcGasPriceResponse>>;
export type RpcHealthRequest = Simplify<z.infer<typeof RpcHealthRequest>>;
export type RpcHealthResponse = Simplify<z.infer<typeof RpcHealthResponse>>;
export type RpcKnownProducer = Simplify<z.infer<typeof RpcKnownProducer>>;
export type RpcLightClientBlockProofRequest = Simplify<z.infer<typeof RpcLightClientBlockProofRequest>>;
export type RpcLightClientBlockProofResponse = Simplify<z.infer<typeof RpcLightClientBlockProofResponse>>;
export type RpcLightClientExecutionProofRequest = Simplify<z.infer<typeof RpcLightClientExecutionProofRequest>>;
export type RpcLightClientExecutionProofResponse = Simplify<z.infer<typeof RpcLightClientExecutionProofResponse>>;
export type RpcLightClientNextBlockRequest = Simplify<z.infer<typeof RpcLightClientNextBlockRequest>>;
export type RpcLightClientNextBlockResponse = Simplify<z.infer<typeof RpcLightClientNextBlockResponse>>;
export type RpcMaintenanceWindowsRequest = Simplify<z.infer<typeof RpcMaintenanceWindowsRequest>>;
export type RpcNetworkInfoRequest = Simplify<z.infer<typeof RpcNetworkInfoRequest>>;
export type RpcNetworkInfoResponse = Simplify<z.infer<typeof RpcNetworkInfoResponse>>;
export type RpcPeerInfo = Simplify<z.infer<typeof RpcPeerInfo>>;
export type RpcProtocolConfigRequest = Simplify<z.infer<typeof RpcProtocolConfigRequest>>;
export type RpcProtocolConfigResponse = Simplify<z.infer<typeof RpcProtocolConfigResponse>>;
export type RpcQueryRequest = Simplify<z.infer<typeof RpcQueryRequest>>;
export type RpcQueryResponse = Simplify<z.infer<typeof RpcQueryResponse>>;
export type RpcReceiptRequest = Simplify<z.infer<typeof RpcReceiptRequest>>;
export type RpcReceiptResponse = Simplify<z.infer<typeof RpcReceiptResponse>>;
export type RpcSendTransactionRequest = Simplify<z.infer<typeof RpcSendTransactionRequest>>;
export type RpcSplitStorageInfoRequest = Simplify<z.infer<typeof RpcSplitStorageInfoRequest>>;
export type RpcSplitStorageInfoResponse = Simplify<z.infer<typeof RpcSplitStorageInfoResponse>>;
export type RpcStateChangesInBlockByTypeRequest = Simplify<z.infer<typeof RpcStateChangesInBlockByTypeRequest>>;
export type RpcStateChangesInBlockByTypeResponse = Simplify<z.infer<typeof RpcStateChangesInBlockByTypeResponse>>;
export type RpcStateChangesInBlockRequest = Simplify<z.infer<typeof RpcStateChangesInBlockRequest>>;
export type RpcStateChangesInBlockResponse = Simplify<z.infer<typeof RpcStateChangesInBlockResponse>>;
export type RpcStatusRequest = Simplify<z.infer<typeof RpcStatusRequest>>;
export type RpcStatusResponse = Simplify<z.infer<typeof RpcStatusResponse>>;
export type RpcTransactionResponse = Simplify<z.infer<typeof RpcTransactionResponse>>;
export type RpcTransactionStatusRequest = Simplify<z.infer<typeof RpcTransactionStatusRequest>>;
export type RpcValidatorRequest = Simplify<z.infer<typeof RpcValidatorRequest>>;
export type RpcValidatorResponse = Simplify<z.infer<typeof RpcValidatorResponse>>;
export type RpcValidatorsOrderedRequest = Simplify<z.infer<typeof RpcValidatorsOrderedRequest>>;
export type RuntimeConfigView = Simplify<z.infer<typeof RuntimeConfigView>>;
export type RuntimeFeesConfigView = Simplify<z.infer<typeof RuntimeFeesConfigView>>;
export type ShardId = Simplify<z.infer<typeof ShardId>>;
export type ShardLayout = Simplify<z.infer<typeof ShardLayout>>;
export type ShardLayoutV0 = Simplify<z.infer<typeof ShardLayoutV0>>;
export type ShardLayoutV1 = Simplify<z.infer<typeof ShardLayoutV1>>;
export type ShardLayoutV2 = Simplify<z.infer<typeof ShardLayoutV2>>;
export type ShardLayoutV3 = Simplify<z.infer<typeof ShardLayoutV3>>;
export type ShardUId = Simplify<z.infer<typeof ShardUId>>;
export type Signature = Simplify<z.infer<typeof Signature>>;
export type SignedDelegateAction = Simplify<z.infer<typeof SignedDelegateAction>>;
export type SignedTransaction = Simplify<z.infer<typeof SignedTransaction>>;
export type SignedTransactionView = Simplify<z.infer<typeof SignedTransactionView>>;
export type SlashedValidator = Simplify<z.infer<typeof SlashedValidator>>;
export type StakeAction = Simplify<z.infer<typeof StakeAction>>;
export type StateChangeCauseView = Simplify<z.infer<typeof StateChangeCauseView>>;
export type StateChangeKindView = Simplify<z.infer<typeof StateChangeKindView>>;
export type StateChangeWithCauseView = Simplify<z.infer<typeof StateChangeWithCauseView>>;
export type StateChangesRequestView = Simplify<z.infer<typeof StateChangesRequestView>>;
export type StateItem = Simplify<z.infer<typeof StateItem>>;
export type StateSyncConfig = Simplify<z.infer<typeof StateSyncConfig>>;
export type StatusSyncInfo = Simplify<z.infer<typeof StatusSyncInfo>>;
export type StorageError = Simplify<z.infer<typeof StorageError>>;
export type StorageGetMode = Simplify<z.infer<typeof StorageGetMode>>;
export type StorageUsageConfigView = Simplify<z.infer<typeof StorageUsageConfigView>>;
export type StoreKey = Simplify<z.infer<typeof StoreKey>>;
export type StoreValue = Simplify<z.infer<typeof StoreValue>>;
export type SyncCheckpoint = Simplify<z.infer<typeof SyncCheckpoint>>;
export type SyncConcurrency = Simplify<z.infer<typeof SyncConcurrency>>;
export type SyncConfig = Simplify<z.infer<typeof SyncConfig>>;
export type Tier1ProxyView = Simplify<z.infer<typeof Tier1ProxyView>>;
export type TrackedShardsConfig = Simplify<z.infer<typeof TrackedShardsConfig>>;
export type TransferAction = Simplify<z.infer<typeof TransferAction>>;
export type TransferToGasKeyAction = Simplify<z.infer<typeof TransferToGasKeyAction>>;
export type TxExecutionError = Simplify<z.infer<typeof TxExecutionError>>;
export type TxExecutionStatus = Simplify<z.infer<typeof TxExecutionStatus>>;
export type UseGlobalContractAction = Simplify<z.infer<typeof UseGlobalContractAction>>;
export type VMConfigView = Simplify<z.infer<typeof VMConfigView>>;
export type VMKind = Simplify<z.infer<typeof VMKind>>;
export type ValidatorInfo = Simplify<z.infer<typeof ValidatorInfo>>;
export type ValidatorKickoutReason = Simplify<z.infer<typeof ValidatorKickoutReason>>;
export type ValidatorKickoutView = Simplify<z.infer<typeof ValidatorKickoutView>>;
export type ValidatorStakeView = Simplify<z.infer<typeof ValidatorStakeView>>;
export type ValidatorStakeViewV1 = Simplify<z.infer<typeof ValidatorStakeViewV1>>;
export type ValidatorStakeViews = Simplify<z.infer<typeof ValidatorStakeViews>>;
export type Version = Simplify<z.infer<typeof Version>>;
export type ViewStateResult = Simplify<z.infer<typeof ViewStateResult>>;
export type WasmTrap = Simplify<z.infer<typeof WasmTrap>>;
export type WithdrawFromGasKeyAction = Simplify<z.infer<typeof WithdrawFromGasKeyAction>>;
export type WitnessConfigView = Simplify<z.infer<typeof WitnessConfigView>>;

export const schemas: Record<string, z.ZodTypeAny> = {
  NearToken,
  FunctionCallPermission,
  GasKeyInfo,
  AccessKeyPermission,
  AccessKey,
  NearGas,
  Fee,
  AccessKeyCreationConfigView,
  AccessKeyPermissionView,
  AccessKeyView,
  PublicKey,
  AccessKeyInfoView,
  AccessKeyList,
  AccountId,
  AccountCreationConfigView,
  Tier1ProxyView,
  AccountDataView,
  AccountIdValidityRulesVersion,
  AccountInfo,
  CryptoHash,
  AccountView,
  AccountWithPublicKey,
  ActionCreationConfigView,
  PrepareError,
  CompilationError,
  MethodResolveError,
  WasmTrap,
  HostError,
  FunctionCallError,
  ActionsValidationError,
  ReceiptValidationError,
  InvalidAccessKeyError,
  GlobalContractIdentifier,
  ActionErrorKind,
  ActionError,
  FunctionArgs,
  CreateAccountAction,
  DeployContractAction,
  FunctionCallAction,
  TransferAction,
  StakeAction,
  AddKeyAction,
  DeleteKeyAction,
  DeleteAccountAction,
  GlobalContractDeployMode,
  DeployGlobalContractAction,
  UseGlobalContractAction,
  DeterministicAccountStateInitV1,
  DeterministicAccountStateInit,
  DeterministicStateInitAction,
  TransferToGasKeyAction,
  WithdrawFromGasKeyAction,
  NonDelegateAction,
  DelegateAction,
  Signature,
  GlobalContractIdentifierView,
  ActionView,
  BandwidthRequestBitmap,
  BandwidthRequest,
  BandwidthRequestsV1,
  BandwidthRequests,
  BlockHeaderInnerLiteView,
  SlashedValidator,
  ValidatorStakeViewV1,
  ValidatorStakeView,
  BlockHeaderView,
  BlockHeightRange,
  BlockHeightRanges,
  BlockId,
  Finality,
  SyncCheckpoint,
  BlockReference,
  BlockStatusView,
  CallResult,
  CatchupStatusView,
  ChunkDistributionUris,
  ChunkDistributionNetworkConfig,
  CongestionInfoView,
  ShardId,
  ChunkHeaderView,
  DurationAsStdSchemaProvider,
  CloudArchivalWriterConfig,
  CongestionControlConfigView,
  ContractCodeView,
  CostGasUsed,
  CurrentEpochValidatorInfo,
  DataReceiptCreationConfigView,
  DataReceiverView,
  PeerInfoView,
  KnownProducerView,
  NetworkInfoView,
  DetailedDebugStatus,
  Direction,
  ExternalStorageLocation,
  DumpConfig,
  EpochId,
  EpochSyncConfig,
  ExecutionMetadataView,
  MissingTrieValueContext,
  MissingTrieValue,
  StorageError,
  InvalidTxError,
  TxExecutionError,
  ExecutionStatusView,
  ExecutionOutcomeView,
  MerklePathItem,
  ExecutionOutcomeWithIdView,
  ExtCostsConfigView,
  ExternalStorageConfig,
  FinalExecutionStatus,
  SignedTransactionView,
  FinalExecutionOutcomeView,
  ReceiptEnumView,
  ReceiptView,
  FinalExecutionOutcomeWithReceiptView,
  GCConfig,
  ShardLayoutV0,
  ShardLayoutV1,
  ShardLayoutV2,
  ShardLayoutV3,
  ShardLayout,
  GenesisConfig,
  GenesisConfigRequest,
  LightClientBlockLiteView,
  LimitConfig,
  LogSummaryStyle,
  MutableConfigValue,
  NextEpochValidatorInfo,
  PeerId,
  ProtocolVersionCheckConfig,
  StoreKey,
  QueryRequest,
  RpcBlockRequest,
  RpcBlockResponse,
  RpcChunkRequest,
  RpcChunkResponse,
  RpcClientConfigRequest,
  SyncConcurrency,
  SyncConfig,
  StateSyncConfig,
  ShardUId,
  TrackedShardsConfig,
  Version,
  RpcClientConfigResponse,
  RpcCongestionLevelRequest,
  RpcCongestionLevelResponse,
  RpcGasPriceRequest,
  RpcGasPriceResponse,
  RpcHealthRequest,
  RpcHealthResponse,
  RpcKnownProducer,
  RpcLightClientBlockProofRequest,
  RpcLightClientBlockProofResponse,
  RpcLightClientExecutionProofRequest,
  RpcLightClientExecutionProofResponse,
  RpcLightClientNextBlockRequest,
  RpcLightClientNextBlockResponse,
  RpcMaintenanceWindowsRequest,
  RpcNetworkInfoRequest,
  RpcPeerInfo,
  RpcNetworkInfoResponse,
  RpcProtocolConfigRequest,
  StorageUsageConfigView,
  RuntimeFeesConfigView,
  StorageGetMode,
  VMKind,
  VMConfigView,
  WitnessConfigView,
  RuntimeConfigView,
  RpcProtocolConfigResponse,
  RpcQueryRequest,
  StoreValue,
  StateItem,
  ViewStateResult,
  RpcQueryResponse,
  RpcReceiptRequest,
  RpcReceiptResponse,
  SignedTransaction,
  TxExecutionStatus,
  RpcSendTransactionRequest,
  RpcSplitStorageInfoRequest,
  RpcSplitStorageInfoResponse,
  StateChangesRequestView,
  RpcStateChangesInBlockByTypeRequest,
  StateChangeKindView,
  RpcStateChangesInBlockByTypeResponse,
  RpcStateChangesInBlockRequest,
  StateChangeCauseView,
  StateChangeWithCauseView,
  RpcStateChangesInBlockResponse,
  RpcStatusRequest,
  StatusSyncInfo,
  ValidatorInfo,
  RpcStatusResponse,
  RpcTransactionResponse,
  RpcTransactionStatusRequest,
  RpcValidatorRequest,
  ValidatorKickoutReason,
  ValidatorKickoutView,
  RpcValidatorResponse,
  RpcValidatorsOrderedRequest,
  SignedDelegateAction,
  ValidatorStakeViews,
};
