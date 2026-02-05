/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
/**Access key provides limited access to an account. Each access key belongs to some account and
is identified by a unique (within the account) public key. One account may have large number of
access keys. Access keys allow to act on behalf of the account by restricting transactions
that can be issued.
`account_id,public_key` is a key in the state*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccessKey",
///  "description": "Access key provides limited access to an account. Each access key belongs to some account and\nis identified by a unique (within the account) public key. One account may have large number of\naccess keys. Access keys allow to act on behalf of the account by restricting transactions\nthat can be issued.\n`account_id,public_key` is a key in the state",
///  "type": "object",
///  "required": [
///    "nonce",
///    "permission"
///  ],
///  "properties": {
///    "nonce": {
///      "description": "Nonce for this access key, used for tx nonce generation. When access key is created, nonce\nis set to `(block_height - 1) * 1e6` to avoid tx hash collision on access key re-creation.\nSee <https://github.com/near/nearcore/issues/3779> for more details.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "permission": {
///      "description": "Defines permissions for this access key.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccessKeyPermission"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKey {
    /**Nonce for this access key, used for tx nonce generation. When access key is created, nonce
is set to `(block_height - 1) * 1e6` to avoid tx hash collision on access key re-creation.
See <https://github.com/near/nearcore/issues/3779> for more details.*/
    pub nonce: u64,
    ///Defines permissions for this access key.
    pub permission: AccessKeyPermission,
}
///Describes the cost of creating an access key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccessKeyCreationConfigView",
///  "description": "Describes the cost of creating an access key.",
///  "type": "object",
///  "required": [
///    "full_access_cost",
///    "function_call_cost",
///    "function_call_cost_per_byte"
///  ],
///  "properties": {
///    "full_access_cost": {
///      "description": "Base cost of creating a full access access-key.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "function_call_cost": {
///      "description": "Base cost of creating an access-key restricted to specific functions.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "function_call_cost_per_byte": {
///      "description": "Cost per byte of method_names of creating a restricted access-key.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKeyCreationConfigView {
    ///Base cost of creating a full access access-key.
    pub full_access_cost: Fee,
    ///Base cost of creating an access-key restricted to specific functions.
    pub function_call_cost: Fee,
    ///Cost per byte of method_names of creating a restricted access-key.
    pub function_call_cost_per_byte: Fee,
}
///Describes information about an access key including the public key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccessKeyInfoView",
///  "description": "Describes information about an access key including the public key.",
///  "type": "object",
///  "required": [
///    "access_key",
///    "public_key"
///  ],
///  "properties": {
///    "access_key": {
///      "$ref": "#/definitions/AccessKeyView"
///    },
///    "public_key": {
///      "$ref": "#/definitions/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKeyInfoView {
    pub access_key: AccessKeyView,
    pub public_key: PublicKey,
}
///Lists access keys
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccessKeyList",
///  "description": "Lists access keys",
///  "type": "object",
///  "required": [
///    "keys"
///  ],
///  "properties": {
///    "keys": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AccessKeyInfoView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKeyList {
    pub keys: ::std::vec::Vec<AccessKeyInfoView>,
}
///Defines permissions for AccessKey
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccessKeyPermission",
///  "description": "Defines permissions for AccessKey",
///  "oneOf": [
///    {
///      "title": "FunctionCall",
///      "type": "object",
///      "required": [
///        "FunctionCall"
///      ],
///      "properties": {
///        "FunctionCall": {
///          "$ref": "#/definitions/FunctionCallPermission"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Grants full access to the account.\nNOTE: It's used to replace account-level public keys.",
///      "type": "string",
///      "const": "FullAccess"
///    },
///    {
///      "title": "GasKeyFunctionCall",
///      "description": "Gas key with limited permission to make transactions with FunctionCallActions\nGas keys are a kind of access keys with a prepaid balance to pay for gas.",
///      "type": "object",
///      "required": [
///        "GasKeyFunctionCall"
///      ],
///      "properties": {
///        "GasKeyFunctionCall": {
///          "type": "array",
///          "items": [
///            {
///              "$ref": "#/definitions/GasKeyInfo"
///            },
///            {
///              "$ref": "#/definitions/FunctionCallPermission"
///            }
///          ],
///          "maxItems": 2,
///          "minItems": 2
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "GasKeyFullAccess",
///      "description": "Gas key with full access to the account.\nGas keys are a kind of access keys with a prepaid balance to pay for gas.",
///      "type": "object",
///      "required": [
///        "GasKeyFullAccess"
///      ],
///      "properties": {
///        "GasKeyFullAccess": {
///          "$ref": "#/definitions/GasKeyInfo"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum AccessKeyPermission {
    FunctionCall(FunctionCallPermission),
    /**Grants full access to the account.
NOTE: It's used to replace account-level public keys.*/
    FullAccess,
    /**Gas key with limited permission to make transactions with FunctionCallActions
Gas keys are a kind of access keys with a prepaid balance to pay for gas.*/
    GasKeyFunctionCall(GasKeyInfo, FunctionCallPermission),
    /**Gas key with full access to the account.
Gas keys are a kind of access keys with a prepaid balance to pay for gas.*/
    GasKeyFullAccess(GasKeyInfo),
}
impl ::std::convert::From<FunctionCallPermission> for AccessKeyPermission {
    fn from(value: FunctionCallPermission) -> Self {
        Self::FunctionCall(value)
    }
}
impl ::std::convert::From<(GasKeyInfo, FunctionCallPermission)> for AccessKeyPermission {
    fn from(value: (GasKeyInfo, FunctionCallPermission)) -> Self {
        Self::GasKeyFunctionCall(value.0, value.1)
    }
}
impl ::std::convert::From<GasKeyInfo> for AccessKeyPermission {
    fn from(value: GasKeyInfo) -> Self {
        Self::GasKeyFullAccess(value)
    }
}
///Describes the permission scope for an access key. Whether it is a function call or a full access key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccessKeyPermissionView",
///  "description": "Describes the permission scope for an access key. Whether it is a function call or a full access key.",
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "FullAccess"
///      ]
///    },
///    {
///      "title": "FunctionCall",
///      "type": "object",
///      "required": [
///        "FunctionCall"
///      ],
///      "properties": {
///        "FunctionCall": {
///          "type": "object",
///          "required": [
///            "method_names",
///            "receiver_id"
///          ],
///          "properties": {
///            "allowance": {
///              "oneOf": [
///                {
///                  "$ref": "#/definitions/NearToken"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "method_names": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            },
///            "receiver_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "GasKeyFunctionCall",
///      "type": "object",
///      "required": [
///        "GasKeyFunctionCall"
///      ],
///      "properties": {
///        "GasKeyFunctionCall": {
///          "type": "object",
///          "required": [
///            "balance",
///            "method_names",
///            "num_nonces",
///            "receiver_id"
///          ],
///          "properties": {
///            "allowance": {
///              "oneOf": [
///                {
///                  "$ref": "#/definitions/NearToken"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "balance": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "method_names": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            },
///            "num_nonces": {
///              "type": "integer",
///              "format": "uint16",
///              "maximum": 65535.0,
///              "minimum": 0.0
///            },
///            "receiver_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "GasKeyFullAccess",
///      "type": "object",
///      "required": [
///        "GasKeyFullAccess"
///      ],
///      "properties": {
///        "GasKeyFullAccess": {
///          "type": "object",
///          "required": [
///            "balance",
///            "num_nonces"
///          ],
///          "properties": {
///            "balance": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "num_nonces": {
///              "type": "integer",
///              "format": "uint16",
///              "maximum": 65535.0,
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum AccessKeyPermissionView {
    FullAccess,
    FunctionCall {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        allowance: ::std::option::Option<NearToken>,
        method_names: ::std::vec::Vec<::std::string::String>,
        receiver_id: ::std::string::String,
    },
    GasKeyFunctionCall {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        allowance: ::std::option::Option<NearToken>,
        balance: NearToken,
        method_names: ::std::vec::Vec<::std::string::String>,
        num_nonces: u16,
        receiver_id: ::std::string::String,
    },
    GasKeyFullAccess { balance: NearToken, num_nonces: u16 },
}
///Describes access key permission scope and nonce.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccessKeyView",
///  "description": "Describes access key permission scope and nonce.",
///  "type": "object",
///  "required": [
///    "nonce",
///    "permission"
///  ],
///  "properties": {
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "permission": {
///      "$ref": "#/definitions/AccessKeyPermissionView"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKeyView {
    pub nonce: u64,
    pub permission: AccessKeyPermissionView,
}
///The structure describes configuration for creation of new accounts.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccountCreationConfigView",
///  "description": "The structure describes configuration for creation of new accounts.",
///  "type": "object",
///  "required": [
///    "min_allowed_top_level_account_length",
///    "registrar_account_id"
///  ],
///  "properties": {
///    "min_allowed_top_level_account_length": {
///      "description": "The minimum length of the top-level account ID that is allowed to be created by any account.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "registrar_account_id": {
///      "description": "The account ID of the account registrar. This account ID allowed to create top-level\naccounts of any valid length.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountCreationConfigView {
    ///The minimum length of the top-level account ID that is allowed to be created by any account.
    pub min_allowed_top_level_account_length: u8,
    /**The account ID of the account registrar. This account ID allowed to create top-level
accounts of any valid length.*/
    pub registrar_account_id: AccountId,
}
/**AccountData is a piece of global state that a validator
signs and broadcasts to the network.

It is essentially the data that a validator wants to share with the network.
All the nodes in the network are collecting the account data
broadcasted by the validators.
Since the number of the validators is bounded and their
identity is known (and the maximal size of allowed AccountData is bounded)
the global state that is distributed in the form of AccountData is bounded
as well.
Find more information in the docs [here](https://github.com/near/nearcore/blob/560f7fc8f4b3106e0d5d46050688610b1f104ac6/chain/client/src/client.rs#L2232)*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccountDataView",
///  "description": "AccountData is a piece of global state that a validator\nsigns and broadcasts to the network.\n\nIt is essentially the data that a validator wants to share with the network.\nAll the nodes in the network are collecting the account data\nbroadcasted by the validators.\nSince the number of the validators is bounded and their\nidentity is known (and the maximal size of allowed AccountData is bounded)\nthe global state that is distributed in the form of AccountData is bounded\nas well.\nFind more information in the docs [here](https://github.com/near/nearcore/blob/560f7fc8f4b3106e0d5d46050688610b1f104ac6/chain/client/src/client.rs#L2232)",
///  "type": "object",
///  "required": [
///    "account_key",
///    "peer_id",
///    "proxies",
///    "timestamp"
///  ],
///  "properties": {
///    "account_key": {
///      "description": "Account key of the validator signing this AccountData.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    },
///    "peer_id": {
///      "description": "ID of the node that handles the account key (aka validator key).",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    },
///    "proxies": {
///      "description": "Proxy nodes that are directly connected to the validator node\n(this list may include the validator node itself).\nTIER1 nodes should connect to one of the proxies to sent TIER1\nmessages to the validator.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Tier1ProxyView"
///      }
///    },
///    "timestamp": {
///      "description": "UTC timestamp of when the AccountData has been signed.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountDataView {
    ///Account key of the validator signing this AccountData.
    pub account_key: PublicKey,
    ///ID of the node that handles the account key (aka validator key).
    pub peer_id: PublicKey,
    /**Proxy nodes that are directly connected to the validator node
(this list may include the validator node itself).
TIER1 nodes should connect to one of the proxies to sent TIER1
messages to the validator.*/
    pub proxies: ::std::vec::Vec<Tier1ProxyView>,
    ///UTC timestamp of when the AccountData has been signed.
    pub timestamp: ::std::string::String,
}
/**NEAR Account Identifier.

This is a unique, syntactically valid, human-readable account identifier on the NEAR network.

[See the crate-level docs for information about validation.](index.html#account-id-rules)

Also see [Error kind precedence](AccountId#error-kind-precedence).

## Examples

```
use near_account_id::AccountId;

let alice: AccountId = "alice.near".parse().unwrap();

assert!("ƒelicia.near".parse::<AccountId>().is_err()); // (ƒ is not f)
```*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccountId",
///  "description": "NEAR Account Identifier.\n\nThis is a unique, syntactically valid, human-readable account identifier on the NEAR network.\n\n[See the crate-level docs for information about validation.](index.html#account-id-rules)\n\nAlso see [Error kind precedence](AccountId#error-kind-precedence).\n\n## Examples\n\n```\nuse near_account_id::AccountId;\n\nlet alice: AccountId = \"alice.near\".parse().unwrap();\n\nassert!(\"ƒelicia.near\".parse::<AccountId>().is_err()); // (ƒ is not f)\n```",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct AccountId(pub ::std::string::String);
impl ::std::ops::Deref for AccountId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AccountId> for ::std::string::String {
    fn from(value: AccountId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for AccountId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for AccountId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`AccountIdValidityRulesVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccountIdValidityRulesVersion",
///  "type": "integer",
///  "format": "uint8",
///  "maximum": 255.0,
///  "minimum": 0.0
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct AccountIdValidityRulesVersion(pub u8);
impl ::std::ops::Deref for AccountIdValidityRulesVersion {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<AccountIdValidityRulesVersion> for u8 {
    fn from(value: AccountIdValidityRulesVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<u8> for AccountIdValidityRulesVersion {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for AccountIdValidityRulesVersion {
    type Err = <u8 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for AccountIdValidityRulesVersion {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for AccountIdValidityRulesVersion {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for AccountIdValidityRulesVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///Account info for validators
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccountInfo",
///  "description": "Account info for validators",
///  "type": "object",
///  "required": [
///    "account_id",
///    "amount",
///    "public_key"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "amount": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "public_key": {
///      "$ref": "#/definitions/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountInfo {
    pub account_id: AccountId,
    pub amount: NearToken,
    pub public_key: PublicKey,
}
///A view of the account
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccountView",
///  "description": "A view of the account",
///  "type": "object",
///  "required": [
///    "amount",
///    "code_hash",
///    "locked",
///    "storage_usage"
///  ],
///  "properties": {
///    "amount": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "code_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "global_contract_account_id": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "global_contract_hash": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "locked": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "storage_paid_at": {
///      "description": "TODO(2271): deprecated.",
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "storage_usage": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountView {
    pub amount: NearToken,
    pub code_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_account_id: ::std::option::Option<AccountId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_hash: ::std::option::Option<CryptoHash>,
    pub locked: NearToken,
    ///TODO(2271): deprecated.
    #[serde(default)]
    pub storage_paid_at: u64,
    pub storage_usage: u64,
}
///Account ID with its public key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AccountWithPublicKey",
///  "description": "Account ID with its public key.",
///  "type": "object",
///  "required": [
///    "account_id",
///    "public_key"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "public_key": {
///      "$ref": "#/definitions/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountWithPublicKey {
    pub account_id: AccountId,
    pub public_key: PublicKey,
}
///Describes the cost of creating a specific action, `Action`. Includes all variants.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ActionCreationConfigView",
///  "description": "Describes the cost of creating a specific action, `Action`. Includes all variants.",
///  "type": "object",
///  "required": [
///    "add_key_cost",
///    "create_account_cost",
///    "delegate_cost",
///    "delete_account_cost",
///    "delete_key_cost",
///    "deploy_contract_cost",
///    "deploy_contract_cost_per_byte",
///    "function_call_cost",
///    "function_call_cost_per_byte",
///    "stake_cost",
///    "transfer_cost"
///  ],
///  "properties": {
///    "add_key_cost": {
///      "description": "Base cost of adding a key.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccessKeyCreationConfigView"
///        }
///      ]
///    },
///    "create_account_cost": {
///      "description": "Base cost of creating an account.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "delegate_cost": {
///      "description": "Base cost for processing a delegate action.\n\nThis is on top of the costs for the actions inside the delegate action.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "delete_account_cost": {
///      "description": "Base cost of deleting an account.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "delete_key_cost": {
///      "description": "Base cost of deleting a key.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "deploy_contract_cost": {
///      "description": "Base cost of deploying a contract.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "deploy_contract_cost_per_byte": {
///      "description": "Cost per byte of deploying a contract.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "function_call_cost": {
///      "description": "Base cost of calling a function.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "function_call_cost_per_byte": {
///      "description": "Cost per byte of method name and arguments of calling a function.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "stake_cost": {
///      "description": "Base cost of staking.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "transfer_cost": {
///      "description": "Base cost of making a transfer.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ActionCreationConfigView {
    ///Base cost of adding a key.
    pub add_key_cost: AccessKeyCreationConfigView,
    ///Base cost of creating an account.
    pub create_account_cost: Fee,
    /**Base cost for processing a delegate action.

This is on top of the costs for the actions inside the delegate action.*/
    pub delegate_cost: Fee,
    ///Base cost of deleting an account.
    pub delete_account_cost: Fee,
    ///Base cost of deleting a key.
    pub delete_key_cost: Fee,
    ///Base cost of deploying a contract.
    pub deploy_contract_cost: Fee,
    ///Cost per byte of deploying a contract.
    pub deploy_contract_cost_per_byte: Fee,
    ///Base cost of calling a function.
    pub function_call_cost: Fee,
    ///Cost per byte of method name and arguments of calling a function.
    pub function_call_cost_per_byte: Fee,
    ///Base cost of staking.
    pub stake_cost: Fee,
    ///Base cost of making a transfer.
    pub transfer_cost: Fee,
}
///An error happened during Action execution
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ActionError",
///  "description": "An error happened during Action execution",
///  "type": "object",
///  "required": [
///    "kind"
///  ],
///  "properties": {
///    "index": {
///      "description": "Index of the failed action in the transaction.\nAction index is not defined if ActionError.kind is `ActionErrorKind::LackBalanceForState`",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "kind": {
///      "description": "The kind of ActionError happened",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ActionErrorKind"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ActionError {
    /**Index of the failed action in the transaction.
Action index is not defined if ActionError.kind is `ActionErrorKind::LackBalanceForState`*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub index: ::std::option::Option<u64>,
    ///The kind of ActionError happened
    pub kind: ActionErrorKind,
}
///`ActionErrorKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ActionErrorKind",
///  "oneOf": [
///    {
///      "title": "AccountAlreadyExists",
///      "description": "Happens when CreateAccount action tries to create an account with account_id which is already exists in the storage",
///      "type": "object",
///      "required": [
///        "AccountAlreadyExists"
///      ],
///      "properties": {
///        "AccountAlreadyExists": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AccountDoesNotExist",
///      "description": "Happens when TX receiver_id doesn't exist (but action is not Action::CreateAccount)",
///      "type": "object",
///      "required": [
///        "AccountDoesNotExist"
///      ],
///      "properties": {
///        "AccountDoesNotExist": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "CreateAccountOnlyByRegistrar",
///      "description": "A top-level account ID can only be created by registrar.",
///      "type": "object",
///      "required": [
///        "CreateAccountOnlyByRegistrar"
///      ],
///      "properties": {
///        "CreateAccountOnlyByRegistrar": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "predecessor_id",
///            "registrar_account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "predecessor_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "registrar_account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "CreateAccountNotAllowed",
///      "description": "A newly created account must be under a namespace of the creator account",
///      "type": "object",
///      "required": [
///        "CreateAccountNotAllowed"
///      ],
///      "properties": {
///        "CreateAccountNotAllowed": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "predecessor_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "predecessor_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ActorNoPermission",
///      "description": "Administrative actions like `DeployContract`, `Stake`, `AddKey`, `DeleteKey`. can be proceed only if sender=receiver\nor the first TX action is a `CreateAccount` action",
///      "type": "object",
///      "required": [
///        "ActorNoPermission"
///      ],
///      "properties": {
///        "ActorNoPermission": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "actor_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "actor_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeleteKeyDoesNotExist",
///      "description": "Account tries to remove an access key that doesn't exist",
///      "type": "object",
///      "required": [
///        "DeleteKeyDoesNotExist"
///      ],
///      "properties": {
///        "DeleteKeyDoesNotExist": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AddKeyAlreadyExists",
///      "description": "The public key is already used for an existing access key",
///      "type": "object",
///      "required": [
///        "AddKeyAlreadyExists"
///      ],
///      "properties": {
///        "AddKeyAlreadyExists": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeleteAccountStaking",
///      "description": "Account is staking and can not be deleted",
///      "type": "object",
///      "required": [
///        "DeleteAccountStaking"
///      ],
///      "properties": {
///        "DeleteAccountStaking": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "LackBalanceForState",
///      "description": "ActionReceipt can't be completed, because the remaining balance will not be enough to cover storage.",
///      "type": "object",
///      "required": [
///        "LackBalanceForState"
///      ],
///      "properties": {
///        "LackBalanceForState": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "amount"
///          ],
///          "properties": {
///            "account_id": {
///              "description": "An account which needs balance",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/AccountId"
///                }
///              ]
///            },
///            "amount": {
///              "description": "Balance required to complete an action.",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/NearToken"
///                }
///              ]
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "TriesToUnstake",
///      "description": "Account is not yet staked, but tries to unstake",
///      "type": "object",
///      "required": [
///        "TriesToUnstake"
///      ],
///      "properties": {
///        "TriesToUnstake": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "TriesToStake",
///      "description": "The account doesn't have enough balance to increase the stake.",
///      "type": "object",
///      "required": [
///        "TriesToStake"
///      ],
///      "properties": {
///        "TriesToStake": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "balance",
///            "locked",
///            "stake"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "balance": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "locked": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "stake": {
///              "$ref": "#/definitions/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InsufficientStake",
///      "type": "object",
///      "required": [
///        "InsufficientStake"
///      ],
///      "properties": {
///        "InsufficientStake": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "minimum_stake",
///            "stake"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "minimum_stake": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "stake": {
///              "$ref": "#/definitions/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "FunctionCallError",
///      "description": "An error occurred during a `FunctionCall` Action, parameter is debug message.",
///      "type": "object",
///      "required": [
///        "FunctionCallError"
///      ],
///      "properties": {
///        "FunctionCallError": {
///          "$ref": "#/definitions/FunctionCallError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "NewReceiptValidationError",
///      "description": "Error occurs when a new `ActionReceipt` created by the `FunctionCall` action fails\nreceipt validation.",
///      "type": "object",
///      "required": [
///        "NewReceiptValidationError"
///      ],
///      "properties": {
///        "NewReceiptValidationError": {
///          "$ref": "#/definitions/ReceiptValidationError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "OnlyImplicitAccountCreationAllowed",
///      "description": "Error occurs when a `CreateAccount` action is called on a NEAR-implicit or ETH-implicit account.\nSee NEAR-implicit account creation NEP: <https://github.com/nearprotocol/NEPs/pull/71>.\nAlso, see ETH-implicit account creation NEP: <https://github.com/near/NEPs/issues/518>.\n\nTODO(#8598): This error is named very poorly. A better name would be\n`OnlyNamedAccountCreationAllowed`.",
///      "type": "object",
///      "required": [
///        "OnlyImplicitAccountCreationAllowed"
///      ],
///      "properties": {
///        "OnlyImplicitAccountCreationAllowed": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeleteAccountWithLargeState",
///      "description": "Delete account whose state is large is temporarily banned.",
///      "type": "object",
///      "required": [
///        "DeleteAccountWithLargeState"
///      ],
///      "properties": {
///        "DeleteAccountWithLargeState": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Signature does not match the provided actions and given signer public key.",
///      "type": "string",
///      "const": "DelegateActionInvalidSignature"
///    },
///    {
///      "title": "DelegateActionSenderDoesNotMatchTxReceiver",
///      "description": "Receiver of the transaction doesn't match Sender of the delegate action",
///      "type": "object",
///      "required": [
///        "DelegateActionSenderDoesNotMatchTxReceiver"
///      ],
///      "properties": {
///        "DelegateActionSenderDoesNotMatchTxReceiver": {
///          "type": "object",
///          "required": [
///            "receiver_id",
///            "sender_id"
///          ],
///          "properties": {
///            "receiver_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "sender_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Delegate action has expired. `max_block_height` is less than actual block height.",
///      "type": "string",
///      "const": "DelegateActionExpired"
///    },
///    {
///      "title": "DelegateActionAccessKeyError",
///      "description": "The given public key doesn't exist for Sender account",
///      "type": "object",
///      "required": [
///        "DelegateActionAccessKeyError"
///      ],
///      "properties": {
///        "DelegateActionAccessKeyError": {
///          "$ref": "#/definitions/InvalidAccessKeyError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DelegateActionInvalidNonce",
///      "description": "DelegateAction nonce must be greater sender[public_key].nonce",
///      "type": "object",
///      "required": [
///        "DelegateActionInvalidNonce"
///      ],
///      "properties": {
///        "DelegateActionInvalidNonce": {
///          "type": "object",
///          "required": [
///            "ak_nonce",
///            "delegate_nonce"
///          ],
///          "properties": {
///            "ak_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "delegate_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DelegateActionNonceTooLarge",
///      "description": "DelegateAction nonce is larger than the upper bound given by the block height",
///      "type": "object",
///      "required": [
///        "DelegateActionNonceTooLarge"
///      ],
///      "properties": {
///        "DelegateActionNonceTooLarge": {
///          "type": "object",
///          "required": [
///            "delegate_nonce",
///            "upper_bound"
///          ],
///          "properties": {
///            "delegate_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "upper_bound": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "GlobalContractDoesNotExist",
///      "type": "object",
///      "required": [
///        "GlobalContractDoesNotExist"
///      ],
///      "properties": {
///        "GlobalContractDoesNotExist": {
///          "type": "object",
///          "required": [
///            "identifier"
///          ],
///          "properties": {
///            "identifier": {
///              "$ref": "#/definitions/GlobalContractIdentifier"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "GasKeyDoesNotExist",
///      "description": "Gas key does not exist for the specified public key",
///      "type": "object",
///      "required": [
///        "GasKeyDoesNotExist"
///      ],
///      "properties": {
///        "GasKeyDoesNotExist": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InsufficientGasKeyBalance",
///      "description": "Gas key does not have sufficient balance for the requested withdrawal",
///      "type": "object",
///      "required": [
///        "InsufficientGasKeyBalance"
///      ],
///      "properties": {
///        "InsufficientGasKeyBalance": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "balance",
///            "public_key",
///            "required"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "balance": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            },
///            "required": {
///              "$ref": "#/definitions/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ActionErrorKind {
    ///Happens when CreateAccount action tries to create an account with account_id which is already exists in the storage
    AccountAlreadyExists { account_id: AccountId },
    ///Happens when TX receiver_id doesn't exist (but action is not Action::CreateAccount)
    AccountDoesNotExist { account_id: AccountId },
    ///A top-level account ID can only be created by registrar.
    CreateAccountOnlyByRegistrar {
        account_id: AccountId,
        predecessor_id: AccountId,
        registrar_account_id: AccountId,
    },
    ///A newly created account must be under a namespace of the creator account
    CreateAccountNotAllowed { account_id: AccountId, predecessor_id: AccountId },
    /**Administrative actions like `DeployContract`, `Stake`, `AddKey`, `DeleteKey`. can be proceed only if sender=receiver
or the first TX action is a `CreateAccount` action*/
    ActorNoPermission { account_id: AccountId, actor_id: AccountId },
    ///Account tries to remove an access key that doesn't exist
    DeleteKeyDoesNotExist { account_id: AccountId, public_key: PublicKey },
    ///The public key is already used for an existing access key
    AddKeyAlreadyExists { account_id: AccountId, public_key: PublicKey },
    ///Account is staking and can not be deleted
    DeleteAccountStaking { account_id: AccountId },
    ///ActionReceipt can't be completed, because the remaining balance will not be enough to cover storage.
    LackBalanceForState {
        ///An account which needs balance
        account_id: AccountId,
        ///Balance required to complete an action.
        amount: NearToken,
    },
    ///Account is not yet staked, but tries to unstake
    TriesToUnstake { account_id: AccountId },
    ///The account doesn't have enough balance to increase the stake.
    TriesToStake {
        account_id: AccountId,
        balance: NearToken,
        locked: NearToken,
        stake: NearToken,
    },
    InsufficientStake {
        account_id: AccountId,
        minimum_stake: NearToken,
        stake: NearToken,
    },
    ///An error occurred during a `FunctionCall` Action, parameter is debug message.
    FunctionCallError(FunctionCallError),
    /**Error occurs when a new `ActionReceipt` created by the `FunctionCall` action fails
receipt validation.*/
    NewReceiptValidationError(ReceiptValidationError),
    /**Error occurs when a `CreateAccount` action is called on a NEAR-implicit or ETH-implicit account.
See NEAR-implicit account creation NEP: <https://github.com/nearprotocol/NEPs/pull/71>.
Also, see ETH-implicit account creation NEP: <https://github.com/near/NEPs/issues/518>.

TODO(#8598): This error is named very poorly. A better name would be
`OnlyNamedAccountCreationAllowed`.*/
    OnlyImplicitAccountCreationAllowed { account_id: AccountId },
    ///Delete account whose state is large is temporarily banned.
    DeleteAccountWithLargeState { account_id: AccountId },
    ///Signature does not match the provided actions and given signer public key.
    DelegateActionInvalidSignature,
    ///Receiver of the transaction doesn't match Sender of the delegate action
    DelegateActionSenderDoesNotMatchTxReceiver {
        receiver_id: AccountId,
        sender_id: AccountId,
    },
    ///Delegate action has expired. `max_block_height` is less than actual block height.
    DelegateActionExpired,
    ///The given public key doesn't exist for Sender account
    DelegateActionAccessKeyError(InvalidAccessKeyError),
    ///DelegateAction nonce must be greater sender[public_key].nonce
    DelegateActionInvalidNonce { ak_nonce: u64, delegate_nonce: u64 },
    ///DelegateAction nonce is larger than the upper bound given by the block height
    DelegateActionNonceTooLarge { delegate_nonce: u64, upper_bound: u64 },
    GlobalContractDoesNotExist { identifier: GlobalContractIdentifier },
    ///Gas key does not exist for the specified public key
    GasKeyDoesNotExist { account_id: AccountId, public_key: PublicKey },
    ///Gas key does not have sufficient balance for the requested withdrawal
    InsufficientGasKeyBalance {
        account_id: AccountId,
        balance: NearToken,
        public_key: PublicKey,
        required: NearToken,
    },
}
impl ::std::convert::From<FunctionCallError> for ActionErrorKind {
    fn from(value: FunctionCallError) -> Self {
        Self::FunctionCallError(value)
    }
}
impl ::std::convert::From<ReceiptValidationError> for ActionErrorKind {
    fn from(value: ReceiptValidationError) -> Self {
        Self::NewReceiptValidationError(value)
    }
}
impl ::std::convert::From<InvalidAccessKeyError> for ActionErrorKind {
    fn from(value: InvalidAccessKeyError) -> Self {
        Self::DelegateActionAccessKeyError(value)
    }
}
///`ActionView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ActionView",
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "CreateAccount"
///      ]
///    },
///    {
///      "title": "DeployContract",
///      "type": "object",
///      "required": [
///        "DeployContract"
///      ],
///      "properties": {
///        "DeployContract": {
///          "type": "object",
///          "required": [
///            "code"
///          ],
///          "properties": {
///            "code": {
///              "type": "string",
///              "format": "bytes"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "FunctionCall",
///      "type": "object",
///      "required": [
///        "FunctionCall"
///      ],
///      "properties": {
///        "FunctionCall": {
///          "type": "object",
///          "required": [
///            "args",
///            "deposit",
///            "gas",
///            "method_name"
///          ],
///          "properties": {
///            "args": {
///              "$ref": "#/definitions/FunctionArgs"
///            },
///            "deposit": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "gas": {
///              "$ref": "#/definitions/NearGas"
///            },
///            "method_name": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Transfer",
///      "type": "object",
///      "required": [
///        "Transfer"
///      ],
///      "properties": {
///        "Transfer": {
///          "type": "object",
///          "required": [
///            "deposit"
///          ],
///          "properties": {
///            "deposit": {
///              "$ref": "#/definitions/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Stake",
///      "type": "object",
///      "required": [
///        "Stake"
///      ],
///      "properties": {
///        "Stake": {
///          "type": "object",
///          "required": [
///            "public_key",
///            "stake"
///          ],
///          "properties": {
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            },
///            "stake": {
///              "$ref": "#/definitions/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AddKey",
///      "type": "object",
///      "required": [
///        "AddKey"
///      ],
///      "properties": {
///        "AddKey": {
///          "type": "object",
///          "required": [
///            "access_key",
///            "public_key"
///          ],
///          "properties": {
///            "access_key": {
///              "$ref": "#/definitions/AccessKeyView"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeleteKey",
///      "type": "object",
///      "required": [
///        "DeleteKey"
///      ],
///      "properties": {
///        "DeleteKey": {
///          "type": "object",
///          "required": [
///            "public_key"
///          ],
///          "properties": {
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeleteAccount",
///      "type": "object",
///      "required": [
///        "DeleteAccount"
///      ],
///      "properties": {
///        "DeleteAccount": {
///          "type": "object",
///          "required": [
///            "beneficiary_id"
///          ],
///          "properties": {
///            "beneficiary_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Delegate",
///      "type": "object",
///      "required": [
///        "Delegate"
///      ],
///      "properties": {
///        "Delegate": {
///          "type": "object",
///          "required": [
///            "delegate_action",
///            "signature"
///          ],
///          "properties": {
///            "delegate_action": {
///              "$ref": "#/definitions/DelegateAction"
///            },
///            "signature": {
///              "$ref": "#/definitions/Signature"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeployGlobalContract",
///      "type": "object",
///      "required": [
///        "DeployGlobalContract"
///      ],
///      "properties": {
///        "DeployGlobalContract": {
///          "type": "object",
///          "required": [
///            "code"
///          ],
///          "properties": {
///            "code": {
///              "type": "string",
///              "format": "bytes"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeployGlobalContractByAccountId",
///      "type": "object",
///      "required": [
///        "DeployGlobalContractByAccountId"
///      ],
///      "properties": {
///        "DeployGlobalContractByAccountId": {
///          "type": "object",
///          "required": [
///            "code"
///          ],
///          "properties": {
///            "code": {
///              "type": "string",
///              "format": "bytes"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "UseGlobalContract",
///      "type": "object",
///      "required": [
///        "UseGlobalContract"
///      ],
///      "properties": {
///        "UseGlobalContract": {
///          "type": "object",
///          "required": [
///            "code_hash"
///          ],
///          "properties": {
///            "code_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "UseGlobalContractByAccountId",
///      "type": "object",
///      "required": [
///        "UseGlobalContractByAccountId"
///      ],
///      "properties": {
///        "UseGlobalContractByAccountId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeterministicStateInit",
///      "type": "object",
///      "required": [
///        "DeterministicStateInit"
///      ],
///      "properties": {
///        "DeterministicStateInit": {
///          "type": "object",
///          "required": [
///            "code",
///            "data",
///            "deposit"
///          ],
///          "properties": {
///            "code": {
///              "$ref": "#/definitions/GlobalContractIdentifierView"
///            },
///            "data": {
///              "type": "object",
///              "additionalProperties": {
///                "type": "string"
///              }
///            },
///            "deposit": {
///              "$ref": "#/definitions/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "TransferToGasKey",
///      "type": "object",
///      "required": [
///        "TransferToGasKey"
///      ],
///      "properties": {
///        "TransferToGasKey": {
///          "type": "object",
///          "required": [
///            "deposit",
///            "public_key"
///          ],
///          "properties": {
///            "deposit": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "WithdrawFromGasKey",
///      "type": "object",
///      "required": [
///        "WithdrawFromGasKey"
///      ],
///      "properties": {
///        "WithdrawFromGasKey": {
///          "type": "object",
///          "required": [
///            "amount",
///            "public_key"
///          ],
///          "properties": {
///            "amount": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ActionView {
    CreateAccount,
    DeployContract { code: ::std::string::String },
    FunctionCall {
        args: FunctionArgs,
        deposit: NearToken,
        gas: NearGas,
        method_name: ::std::string::String,
    },
    Transfer { deposit: NearToken },
    Stake { public_key: PublicKey, stake: NearToken },
    AddKey { access_key: AccessKeyView, public_key: PublicKey },
    DeleteKey { public_key: PublicKey },
    DeleteAccount { beneficiary_id: AccountId },
    Delegate { delegate_action: DelegateAction, signature: Signature },
    DeployGlobalContract { code: ::std::string::String },
    DeployGlobalContractByAccountId { code: ::std::string::String },
    UseGlobalContract { code_hash: CryptoHash },
    UseGlobalContractByAccountId { account_id: AccountId },
    DeterministicStateInit {
        code: GlobalContractIdentifierView,
        data: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        deposit: NearToken,
    },
    TransferToGasKey { deposit: NearToken, public_key: PublicKey },
    WithdrawFromGasKey { amount: NearToken, public_key: PublicKey },
}
///Describes the error for validating a list of actions.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ActionsValidationError",
///  "description": "Describes the error for validating a list of actions.",
///  "oneOf": [
///    {
///      "description": "The delete action must be a final action in transaction",
///      "type": "string",
///      "const": "DeleteActionMustBeFinal"
///    },
///    {
///      "title": "TotalPrepaidGasExceeded",
///      "description": "The total prepaid gas (for all given actions) exceeded the limit.",
///      "type": "object",
///      "required": [
///        "TotalPrepaidGasExceeded"
///      ],
///      "properties": {
///        "TotalPrepaidGasExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "total_prepaid_gas"
///          ],
///          "properties": {
///            "limit": {
///              "$ref": "#/definitions/NearGas"
///            },
///            "total_prepaid_gas": {
///              "$ref": "#/definitions/NearGas"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "TotalNumberOfActionsExceeded",
///      "description": "The number of actions exceeded the given limit.",
///      "type": "object",
///      "required": [
///        "TotalNumberOfActionsExceeded"
///      ],
///      "properties": {
///        "TotalNumberOfActionsExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "total_number_of_actions"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "total_number_of_actions": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AddKeyMethodNamesNumberOfBytesExceeded",
///      "description": "The total number of bytes of the method names exceeded the limit in a Add Key action.",
///      "type": "object",
///      "required": [
///        "AddKeyMethodNamesNumberOfBytesExceeded"
///      ],
///      "properties": {
///        "AddKeyMethodNamesNumberOfBytesExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "total_number_of_bytes"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "total_number_of_bytes": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AddKeyMethodNameLengthExceeded",
///      "description": "The length of some method name exceeded the limit in a Add Key action.",
///      "type": "object",
///      "required": [
///        "AddKeyMethodNameLengthExceeded"
///      ],
///      "properties": {
///        "AddKeyMethodNameLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Integer overflow during a compute.",
///      "type": "string",
///      "const": "IntegerOverflow"
///    },
///    {
///      "title": "InvalidAccountId",
///      "description": "Invalid account ID.",
///      "type": "object",
///      "required": [
///        "InvalidAccountId"
///      ],
///      "properties": {
///        "InvalidAccountId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ContractSizeExceeded",
///      "description": "The size of the contract code exceeded the limit in a DeployContract action.",
///      "type": "object",
///      "required": [
///        "ContractSizeExceeded"
///      ],
///      "properties": {
///        "ContractSizeExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "size"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "size": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "FunctionCallMethodNameLengthExceeded",
///      "description": "The length of the method name exceeded the limit in a Function Call action.",
///      "type": "object",
///      "required": [
///        "FunctionCallMethodNameLengthExceeded"
///      ],
///      "properties": {
///        "FunctionCallMethodNameLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "FunctionCallArgumentsLengthExceeded",
///      "description": "The length of the arguments exceeded the limit in a Function Call action.",
///      "type": "object",
///      "required": [
///        "FunctionCallArgumentsLengthExceeded"
///      ],
///      "properties": {
///        "FunctionCallArgumentsLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "UnsuitableStakingKey",
///      "description": "An attempt to stake with a public key that is not convertible to ristretto.",
///      "type": "object",
///      "required": [
///        "UnsuitableStakingKey"
///      ],
///      "properties": {
///        "UnsuitableStakingKey": {
///          "type": "object",
///          "required": [
///            "public_key"
///          ],
///          "properties": {
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The attached amount of gas in a FunctionCall action has to be a positive number.",
///      "type": "string",
///      "const": "FunctionCallZeroAttachedGas"
///    },
///    {
///      "description": "There should be the only one DelegateAction",
///      "type": "string",
///      "const": "DelegateActionMustBeOnlyOne"
///    },
///    {
///      "title": "UnsupportedProtocolFeature",
///      "description": "The transaction includes a feature that the current protocol version\ndoes not support.\n\nNote: we stringify the protocol feature name instead of using\n`ProtocolFeature` here because we don't want to leak the internals of\nthat type into observable borsh serialization.",
///      "type": "object",
///      "required": [
///        "UnsupportedProtocolFeature"
///      ],
///      "properties": {
///        "UnsupportedProtocolFeature": {
///          "type": "object",
///          "required": [
///            "protocol_feature",
///            "version"
///          ],
///          "properties": {
///            "protocol_feature": {
///              "type": "string"
///            },
///            "version": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidDeterministicStateInitReceiver",
///      "type": "object",
///      "required": [
///        "InvalidDeterministicStateInitReceiver"
///      ],
///      "properties": {
///        "InvalidDeterministicStateInitReceiver": {
///          "type": "object",
///          "required": [
///            "derived_id",
///            "receiver_id"
///          ],
///          "properties": {
///            "derived_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "receiver_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeterministicStateInitKeyLengthExceeded",
///      "type": "object",
///      "required": [
///        "DeterministicStateInitKeyLengthExceeded"
///      ],
///      "properties": {
///        "DeterministicStateInitKeyLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeterministicStateInitValueLengthExceeded",
///      "type": "object",
///      "required": [
///        "DeterministicStateInitValueLengthExceeded"
///      ],
///      "properties": {
///        "DeterministicStateInitValueLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "GasKeyInvalidNumNonces",
///      "type": "object",
///      "required": [
///        "GasKeyInvalidNumNonces"
///      ],
///      "properties": {
///        "GasKeyInvalidNumNonces": {
///          "type": "object",
///          "required": [
///            "limit",
///            "requested_nonces"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint16",
///              "maximum": 65535.0,
///              "minimum": 0.0
///            },
///            "requested_nonces": {
///              "type": "integer",
///              "format": "uint16",
///              "maximum": 65535.0,
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AddGasKeyWithNonZeroBalance",
///      "type": "object",
///      "required": [
///        "AddGasKeyWithNonZeroBalance"
///      ],
///      "properties": {
///        "AddGasKeyWithNonZeroBalance": {
///          "type": "object",
///          "required": [
///            "balance"
///          ],
///          "properties": {
///            "balance": {
///              "$ref": "#/definitions/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Gas keys with FunctionCall permission cannot have an allowance set.",
///      "type": "string",
///      "const": "GasKeyFunctionCallAllowanceNotAllowed"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ActionsValidationError {
    ///The delete action must be a final action in transaction
    DeleteActionMustBeFinal,
    ///The total prepaid gas (for all given actions) exceeded the limit.
    TotalPrepaidGasExceeded { limit: NearGas, total_prepaid_gas: NearGas },
    ///The number of actions exceeded the given limit.
    TotalNumberOfActionsExceeded { limit: u64, total_number_of_actions: u64 },
    ///The total number of bytes of the method names exceeded the limit in a Add Key action.
    AddKeyMethodNamesNumberOfBytesExceeded { limit: u64, total_number_of_bytes: u64 },
    ///The length of some method name exceeded the limit in a Add Key action.
    AddKeyMethodNameLengthExceeded { length: u64, limit: u64 },
    ///Integer overflow during a compute.
    IntegerOverflow,
    ///Invalid account ID.
    InvalidAccountId { account_id: ::std::string::String },
    ///The size of the contract code exceeded the limit in a DeployContract action.
    ContractSizeExceeded { limit: u64, size: u64 },
    ///The length of the method name exceeded the limit in a Function Call action.
    FunctionCallMethodNameLengthExceeded { length: u64, limit: u64 },
    ///The length of the arguments exceeded the limit in a Function Call action.
    FunctionCallArgumentsLengthExceeded { length: u64, limit: u64 },
    ///An attempt to stake with a public key that is not convertible to ristretto.
    UnsuitableStakingKey { public_key: PublicKey },
    ///The attached amount of gas in a FunctionCall action has to be a positive number.
    FunctionCallZeroAttachedGas,
    ///There should be the only one DelegateAction
    DelegateActionMustBeOnlyOne,
    /**The transaction includes a feature that the current protocol version
does not support.

Note: we stringify the protocol feature name instead of using
`ProtocolFeature` here because we don't want to leak the internals of
that type into observable borsh serialization.*/
    UnsupportedProtocolFeature { protocol_feature: ::std::string::String, version: u32 },
    InvalidDeterministicStateInitReceiver {
        derived_id: AccountId,
        receiver_id: AccountId,
    },
    DeterministicStateInitKeyLengthExceeded { length: u64, limit: u64 },
    DeterministicStateInitValueLengthExceeded { length: u64, limit: u64 },
    GasKeyInvalidNumNonces { limit: u16, requested_nonces: u16 },
    AddGasKeyWithNonZeroBalance { balance: NearToken },
    ///Gas keys with FunctionCall permission cannot have an allowance set.
    GasKeyFunctionCallAllowanceNotAllowed,
}
///An action that adds key with public key associated
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "AddKeyAction",
///  "description": "An action that adds key with public key associated",
///  "type": "object",
///  "required": [
///    "access_key",
///    "public_key"
///  ],
///  "properties": {
///    "access_key": {
///      "description": "An access key with the permission",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccessKey"
///        }
///      ]
///    },
///    "public_key": {
///      "description": "A public key which will be associated with an access_key",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AddKeyAction {
    ///An access key with the permission
    pub access_key: AccessKey,
    ///A public key which will be associated with an access_key
    pub public_key: PublicKey,
}
/**`BandwidthRequest` describes the size of receipts that a shard would like to send to another shard.
When a shard wants to send a lot of receipts to another shard, it needs to create a request and wait
for a bandwidth grant from the bandwidth scheduler.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BandwidthRequest",
///  "description": "`BandwidthRequest` describes the size of receipts that a shard would like to send to another shard.\nWhen a shard wants to send a lot of receipts to another shard, it needs to create a request and wait\nfor a bandwidth grant from the bandwidth scheduler.",
///  "type": "object",
///  "required": [
///    "requested_values_bitmap",
///    "to_shard"
///  ],
///  "properties": {
///    "requested_values_bitmap": {
///      "description": "Bitmap which describes what values of bandwidth are requested.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/BandwidthRequestBitmap"
///        }
///      ]
///    },
///    "to_shard": {
///      "description": "Requesting bandwidth to this shard.",
///      "type": "integer",
///      "format": "uint16",
///      "maximum": 65535.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BandwidthRequest {
    ///Bitmap which describes what values of bandwidth are requested.
    pub requested_values_bitmap: BandwidthRequestBitmap,
    ///Requesting bandwidth to this shard.
    pub to_shard: u16,
}
/**Bitmap which describes which values from the predefined list are being requested.
The nth bit is set to 1 when the nth value from the list is being requested.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BandwidthRequestBitmap",
///  "description": "Bitmap which describes which values from the predefined list are being requested.\nThe nth bit is set to 1 when the nth value from the list is being requested.",
///  "type": "object",
///  "required": [
///    "data"
///  ],
///  "properties": {
///    "data": {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint8",
///        "maximum": 255.0,
///        "minimum": 0.0
///      },
///      "maxItems": 5,
///      "minItems": 5
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BandwidthRequestBitmap {
    pub data: [u8; 5usize],
}
/**A list of shard's bandwidth requests.
Describes how much the shard would like to send to other shards.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BandwidthRequests",
///  "description": "A list of shard's bandwidth requests.\nDescribes how much the shard would like to send to other shards.",
///  "oneOf": [
///    {
///      "title": "V1",
///      "type": "object",
///      "required": [
///        "V1"
///      ],
///      "properties": {
///        "V1": {
///          "$ref": "#/definitions/BandwidthRequestsV1"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum BandwidthRequests {
    V1(BandwidthRequestsV1),
}
impl ::std::convert::From<BandwidthRequestsV1> for BandwidthRequests {
    fn from(value: BandwidthRequestsV1) -> Self {
        Self::V1(value)
    }
}
///Version 1 of [`BandwidthRequest`].
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BandwidthRequestsV1",
///  "description": "Version 1 of [`BandwidthRequest`].",
///  "type": "object",
///  "required": [
///    "requests"
///  ],
///  "properties": {
///    "requests": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/BandwidthRequest"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BandwidthRequestsV1 {
    pub requests: ::std::vec::Vec<BandwidthRequest>,
}
///A part of a state for the current head of a light client. More info [here](https://nomicon.io/ChainSpec/LightClient).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BlockHeaderInnerLiteView",
///  "description": "A part of a state for the current head of a light client. More info [here](https://nomicon.io/ChainSpec/LightClient).",
///  "type": "object",
///  "required": [
///    "block_merkle_root",
///    "epoch_id",
///    "height",
///    "next_bp_hash",
///    "next_epoch_id",
///    "outcome_root",
///    "prev_state_root",
///    "timestamp",
///    "timestamp_nanosec"
///  ],
///  "properties": {
///    "block_merkle_root": {
///      "description": "The merkle root of all the block hashes",
///      "allOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      ]
///    },
///    "epoch_id": {
///      "description": "The epoch to which the block that is the current known head belongs",
///      "allOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      ]
///    },
///    "height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "next_bp_hash": {
///      "description": "The hash of the block producers set for the next epoch",
///      "allOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      ]
///    },
///    "next_epoch_id": {
///      "description": "The epoch that will follow the current epoch",
///      "allOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      ]
///    },
///    "outcome_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "prev_state_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "timestamp": {
///      "description": "Legacy json number. Should not be used.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timestamp_nanosec": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BlockHeaderInnerLiteView {
    ///The merkle root of all the block hashes
    pub block_merkle_root: CryptoHash,
    ///The epoch to which the block that is the current known head belongs
    pub epoch_id: CryptoHash,
    pub height: u64,
    ///The hash of the block producers set for the next epoch
    pub next_bp_hash: CryptoHash,
    ///The epoch that will follow the current epoch
    pub next_epoch_id: CryptoHash,
    pub outcome_root: CryptoHash,
    pub prev_state_root: CryptoHash,
    ///Legacy json number. Should not be used.
    pub timestamp: u64,
    pub timestamp_nanosec: ::std::string::String,
}
///Contains main info about the block.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BlockHeaderView",
///  "description": "Contains main info about the block.",
///  "type": "object",
///  "required": [
///    "approvals",
///    "block_merkle_root",
///    "challenges_result",
///    "challenges_root",
///    "chunk_headers_root",
///    "chunk_mask",
///    "chunk_receipts_root",
///    "chunk_tx_root",
///    "chunks_included",
///    "epoch_id",
///    "gas_price",
///    "hash",
///    "height",
///    "last_ds_final_block",
///    "last_final_block",
///    "latest_protocol_version",
///    "next_bp_hash",
///    "next_epoch_id",
///    "outcome_root",
///    "prev_hash",
///    "prev_state_root",
///    "random_value",
///    "signature",
///    "timestamp",
///    "timestamp_nanosec",
///    "total_supply",
///    "validator_proposals"
///  ],
///  "properties": {
///    "approvals": {
///      "type": "array",
///      "items": {
///        "oneOf": [
///          {
///            "$ref": "#/definitions/Signature"
///          },
///          {
///            "type": "null"
///          }
///        ]
///      }
///    },
///    "block_body_hash": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "block_merkle_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "block_ordinal": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "challenges_result": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/SlashedValidator"
///      }
///    },
///    "challenges_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "chunk_endorsements": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "array",
///        "items": {
///          "type": "integer",
///          "format": "uint8",
///          "maximum": 255.0,
///          "minimum": 0.0
///        }
///      }
///    },
///    "chunk_headers_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "chunk_mask": {
///      "type": "array",
///      "items": {
///        "type": "boolean"
///      }
///    },
///    "chunk_receipts_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "chunk_tx_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "chunks_included": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "epoch_id": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "epoch_sync_data_hash": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "gas_price": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "last_ds_final_block": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "last_final_block": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "latest_protocol_version": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "next_bp_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "next_epoch_id": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "outcome_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "prev_hash": {
///      "description": "The hash of the previous Block",
///      "allOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      ]
///    },
///    "prev_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "prev_state_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "random_value": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "rent_paid": {
///      "description": "TODO(2271): deprecated.",
///      "default": "0",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "shard_split": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": [
///        {
///          "$ref": "#/definitions/ShardId"
///        },
///        {
///          "$ref": "#/definitions/AccountId"
///        }
///      ],
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "signature": {
///      "description": "Signature of the block producer.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Signature"
///        }
///      ]
///    },
///    "timestamp": {
///      "description": "Legacy json number. Should not be used.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timestamp_nanosec": {
///      "type": "string"
///    },
///    "total_supply": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "validator_proposals": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ValidatorStakeView"
///      }
///    },
///    "validator_reward": {
///      "description": "TODO(2271): deprecated.",
///      "default": "0",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BlockHeaderView {
    pub approvals: ::std::vec::Vec<::std::option::Option<Signature>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_body_hash: ::std::option::Option<CryptoHash>,
    pub block_merkle_root: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_ordinal: ::std::option::Option<u64>,
    pub challenges_result: ::std::vec::Vec<SlashedValidator>,
    pub challenges_root: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_endorsements: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<u8>>>,
    pub chunk_headers_root: CryptoHash,
    pub chunk_mask: ::std::vec::Vec<bool>,
    pub chunk_receipts_root: CryptoHash,
    pub chunk_tx_root: CryptoHash,
    pub chunks_included: u64,
    pub epoch_id: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_sync_data_hash: ::std::option::Option<CryptoHash>,
    pub gas_price: NearToken,
    pub hash: CryptoHash,
    pub height: u64,
    pub last_ds_final_block: CryptoHash,
    pub last_final_block: CryptoHash,
    pub latest_protocol_version: u32,
    pub next_bp_hash: CryptoHash,
    pub next_epoch_id: CryptoHash,
    pub outcome_root: CryptoHash,
    ///The hash of the previous Block
    pub prev_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prev_height: ::std::option::Option<u64>,
    pub prev_state_root: CryptoHash,
    pub random_value: CryptoHash,
    ///TODO(2271): deprecated.
    #[serde(default = "defaults::block_header_view_rent_paid")]
    pub rent_paid: NearToken,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shard_split: ::std::option::Option<(ShardId, AccountId)>,
    ///Signature of the block producer.
    pub signature: Signature,
    ///Legacy json number. Should not be used.
    pub timestamp: u64,
    pub timestamp_nanosec: ::std::string::String,
    pub total_supply: NearToken,
    pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
    ///TODO(2271): deprecated.
    #[serde(default = "defaults::block_header_view_validator_reward")]
    pub validator_reward: NearToken,
}
///`BlockHeightRange`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BlockHeightRange",
///  "type": "object",
///  "required": [
///    "end",
///    "start"
///  ],
///  "properties": {
///    "end": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "start": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BlockHeightRange {
    pub end: u64,
    pub start: u64,
}
///`BlockHeightRanges`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BlockHeightRanges",
///  "type": "array",
///  "items": {
///    "$ref": "#/definitions/BlockHeightRange"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct BlockHeightRanges(pub ::std::vec::Vec<BlockHeightRange>);
impl ::std::ops::Deref for BlockHeightRanges {
    type Target = ::std::vec::Vec<BlockHeightRange>;
    fn deref(&self) -> &::std::vec::Vec<BlockHeightRange> {
        &self.0
    }
}
impl ::std::convert::From<BlockHeightRanges> for ::std::vec::Vec<BlockHeightRange> {
    fn from(value: BlockHeightRanges) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<BlockHeightRange>> for BlockHeightRanges {
    fn from(value: ::std::vec::Vec<BlockHeightRange>) -> Self {
        Self(value)
    }
}
///`BlockId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BlockId",
///  "oneOf": [
///    {
///      "title": "BlockHeight",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum BlockId {
    BlockHeight(u64),
    CryptoHash(CryptoHash),
}
impl ::std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::BlockHeight(x) => x.fmt(f),
            Self::CryptoHash(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<u64> for BlockId {
    fn from(value: u64) -> Self {
        Self::BlockHeight(value)
    }
}
impl ::std::convert::From<CryptoHash> for BlockId {
    fn from(value: CryptoHash) -> Self {
        Self::CryptoHash(value)
    }
}
///`BlockReference`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BlockReference",
///  "oneOf": [
///    {
///      "title": "BlockId",
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        }
///      }
///    },
///    {
///      "title": "Finality",
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/definitions/Finality"
///        }
///      }
///    },
///    {
///      "title": "SyncCheckpoint",
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/definitions/SyncCheckpoint"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum BlockReference {
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "finality")]
    Finality(Finality),
    #[serde(rename = "sync_checkpoint")]
    SyncCheckpoint(SyncCheckpoint),
}
impl ::std::convert::From<BlockId> for BlockReference {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
impl ::std::convert::From<Finality> for BlockReference {
    fn from(value: Finality) -> Self {
        Self::Finality(value)
    }
}
impl ::std::convert::From<SyncCheckpoint> for BlockReference {
    fn from(value: SyncCheckpoint) -> Self {
        Self::SyncCheckpoint(value)
    }
}
///Height and hash of a block
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BlockStatusView",
///  "description": "Height and hash of a block",
///  "type": "object",
///  "required": [
///    "hash",
///    "height"
///  ],
///  "properties": {
///    "hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BlockStatusView {
    pub hash: CryptoHash,
    pub height: u64,
}
///A result returned by contract method
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CallResult",
///  "description": "A result returned by contract method",
///  "type": "object",
///  "required": [
///    "logs",
///    "result"
///  ],
///  "properties": {
///    "logs": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "result": {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint8",
///        "maximum": 255.0,
///        "minimum": 0.0
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CallResult {
    pub logs: ::std::vec::Vec<::std::string::String>,
    pub result: ::std::vec::Vec<u8>,
}
///Status of the [catchup](https://near.github.io/nearcore/architecture/how/sync.html#catchup) process
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CatchupStatusView",
///  "description": "Status of the [catchup](https://near.github.io/nearcore/architecture/how/sync.html#catchup) process",
///  "type": "object",
///  "required": [
///    "blocks_to_catchup",
///    "shard_sync_status",
///    "sync_block_hash",
///    "sync_block_height"
///  ],
///  "properties": {
///    "blocks_to_catchup": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/BlockStatusView"
///      }
///    },
///    "shard_sync_status": {
///      "type": "object",
///      "patternProperties": {
///        "^\\d+$": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "sync_block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "sync_block_height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CatchupStatusView {
    pub blocks_to_catchup: ::std::vec::Vec<BlockStatusView>,
    pub shard_sync_status: ::std::collections::HashMap<
        CatchupStatusViewShardSyncStatusKey,
        ::std::string::String,
    >,
    pub sync_block_hash: CryptoHash,
    pub sync_block_height: u64,
}
///`CatchupStatusViewShardSyncStatusKey`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CatchupStatusViewShardSyncStatusKey(::std::string::String);
impl ::std::ops::Deref for CatchupStatusViewShardSyncStatusKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CatchupStatusViewShardSyncStatusKey>
for ::std::string::String {
    fn from(value: CatchupStatusViewShardSyncStatusKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CatchupStatusViewShardSyncStatusKey {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CatchupStatusViewShardSyncStatusKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CatchupStatusViewShardSyncStatusKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CatchupStatusViewShardSyncStatusKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CatchupStatusViewShardSyncStatusKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Config for the Chunk Distribution Network feature.
This allows nodes to push and pull chunks from a central stream.
The two benefits of this approach are: (1) less request/response traffic
on the peer-to-peer network and (2) lower latency for RPC nodes indexing the chain.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ChunkDistributionNetworkConfig",
///  "description": "Config for the Chunk Distribution Network feature.\nThis allows nodes to push and pull chunks from a central stream.\nThe two benefits of this approach are: (1) less request/response traffic\non the peer-to-peer network and (2) lower latency for RPC nodes indexing the chain.",
///  "type": "object",
///  "required": [
///    "enabled",
///    "uris"
///  ],
///  "properties": {
///    "enabled": {
///      "type": "boolean"
///    },
///    "uris": {
///      "$ref": "#/definitions/ChunkDistributionUris"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChunkDistributionNetworkConfig {
    pub enabled: bool,
    pub uris: ChunkDistributionUris,
}
///URIs for the Chunk Distribution Network feature.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ChunkDistributionUris",
///  "description": "URIs for the Chunk Distribution Network feature.",
///  "type": "object",
///  "required": [
///    "get",
///    "set"
///  ],
///  "properties": {
///    "get": {
///      "description": "URI for pulling chunks from the stream.",
///      "type": "string"
///    },
///    "set": {
///      "description": "URI for publishing chunks to the stream.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChunkDistributionUris {
    ///URI for pulling chunks from the stream.
    pub get: ::std::string::String,
    ///URI for publishing chunks to the stream.
    pub set: ::std::string::String,
}
///Contains main info about the chunk.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ChunkHeaderView",
///  "description": "Contains main info about the chunk.",
///  "type": "object",
///  "required": [
///    "balance_burnt",
///    "chunk_hash",
///    "encoded_length",
///    "encoded_merkle_root",
///    "gas_limit",
///    "gas_used",
///    "height_created",
///    "height_included",
///    "outcome_root",
///    "outgoing_receipts_root",
///    "prev_block_hash",
///    "prev_state_root",
///    "shard_id",
///    "signature",
///    "tx_root",
///    "validator_proposals"
///  ],
///  "properties": {
///    "balance_burnt": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "bandwidth_requests": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/BandwidthRequests"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "chunk_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "congestion_info": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/CongestionInfoView"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "encoded_length": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "encoded_merkle_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "gas_limit": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "gas_used": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "height_created": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "height_included": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "outcome_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "outgoing_receipts_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "prev_block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "prev_state_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "rent_paid": {
///      "description": "TODO(2271): deprecated.",
///      "default": "0",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "shard_id": {
///      "$ref": "#/definitions/ShardId"
///    },
///    "signature": {
///      "$ref": "#/definitions/Signature"
///    },
///    "tx_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "validator_proposals": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ValidatorStakeView"
///      }
///    },
///    "validator_reward": {
///      "description": "TODO(2271): deprecated.",
///      "default": "0",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChunkHeaderView {
    pub balance_burnt: NearToken,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bandwidth_requests: ::std::option::Option<BandwidthRequests>,
    pub chunk_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub congestion_info: ::std::option::Option<CongestionInfoView>,
    pub encoded_length: u64,
    pub encoded_merkle_root: CryptoHash,
    pub gas_limit: NearGas,
    pub gas_used: NearGas,
    pub height_created: u64,
    pub height_included: u64,
    pub outcome_root: CryptoHash,
    pub outgoing_receipts_root: CryptoHash,
    pub prev_block_hash: CryptoHash,
    pub prev_state_root: CryptoHash,
    ///TODO(2271): deprecated.
    #[serde(default = "defaults::chunk_header_view_rent_paid")]
    pub rent_paid: NearToken,
    pub shard_id: ShardId,
    pub signature: Signature,
    pub tx_root: CryptoHash,
    pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
    ///TODO(2271): deprecated.
    #[serde(default = "defaults::chunk_header_view_validator_reward")]
    pub validator_reward: NearToken,
}
/**Configuration for a cloud-based archival writer. If this config is present, the writer is enabled and
writes chunk-related data based on the tracked shards. This config also controls additional archival
behavior such as block data and polling interval.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CloudArchivalWriterConfig",
///  "description": "Configuration for a cloud-based archival writer. If this config is present, the writer is enabled and\nwrites chunk-related data based on the tracked shards. This config also controls additional archival\nbehavior such as block data and polling interval.",
///  "type": "object",
///  "properties": {
///    "archive_block_data": {
///      "description": "Determines whether block-related data should be written to cloud storage.",
///      "default": false,
///      "type": "boolean"
///    },
///    "polling_interval": {
///      "description": "Interval at which the system checks for new blocks or chunks to archive.",
///      "default": {
///        "nanos": 0,
///        "secs": 1
///      },
///      "allOf": [
///        {
///          "$ref": "#/definitions/DurationAsStdSchemaProvider"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CloudArchivalWriterConfig {
    ///Determines whether block-related data should be written to cloud storage.
    #[serde(default)]
    pub archive_block_data: bool,
    ///Interval at which the system checks for new blocks or chunks to archive.
    #[serde(default = "defaults::cloud_archival_writer_config_polling_interval")]
    pub polling_interval: DurationAsStdSchemaProvider,
}
impl ::std::default::Default for CloudArchivalWriterConfig {
    fn default() -> Self {
        Self {
            archive_block_data: Default::default(),
            polling_interval: defaults::cloud_archival_writer_config_polling_interval(),
        }
    }
}
///`CompilationError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CompilationError",
///  "oneOf": [
///    {
///      "title": "CodeDoesNotExist",
///      "type": "object",
///      "required": [
///        "CodeDoesNotExist"
///      ],
///      "properties": {
///        "CodeDoesNotExist": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "PrepareError",
///      "type": "object",
///      "required": [
///        "PrepareError"
///      ],
///      "properties": {
///        "PrepareError": {
///          "$ref": "#/definitions/PrepareError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "WasmerCompileError",
///      "description": "This is for defense in depth.\nWe expect our runtime-independent preparation code to fully catch all invalid wasms,\nbut, if it ever misses something we’ll emit this error",
///      "type": "object",
///      "required": [
///        "WasmerCompileError"
///      ],
///      "properties": {
///        "WasmerCompileError": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum CompilationError {
    CodeDoesNotExist { account_id: AccountId },
    PrepareError(PrepareError),
    /**This is for defense in depth.
We expect our runtime-independent preparation code to fully catch all invalid wasms,
but, if it ever misses something we’ll emit this error*/
    WasmerCompileError { msg: ::std::string::String },
}
impl ::std::convert::From<PrepareError> for CompilationError {
    fn from(value: PrepareError) -> Self {
        Self::PrepareError(value)
    }
}
///The configuration for congestion control. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CongestionControlConfigView",
///  "description": "The configuration for congestion control. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)",
///  "type": "object",
///  "required": [
///    "allowed_shard_outgoing_gas",
///    "max_congestion_incoming_gas",
///    "max_congestion_memory_consumption",
///    "max_congestion_missed_chunks",
///    "max_congestion_outgoing_gas",
///    "max_outgoing_gas",
///    "max_tx_gas",
///    "min_outgoing_gas",
///    "min_tx_gas",
///    "outgoing_receipts_big_size_limit",
///    "outgoing_receipts_usual_size_limit",
///    "reject_tx_congestion_threshold"
///  ],
///  "properties": {
///    "allowed_shard_outgoing_gas": {
///      "description": "How much gas the chosen allowed shard can send to a 100% congested shard.\n\nSee [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "max_congestion_incoming_gas": {
///      "description": "How much gas in delayed receipts of a shard is 100% incoming congestion.\n\nSee [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "max_congestion_memory_consumption": {
///      "description": "How much memory space of all delayed and buffered receipts in a shard is\nconsidered 100% congested.\n\nSee [`CongestionControlConfig`] for more details.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_congestion_missed_chunks": {
///      "description": "How many missed chunks in a row in a shard is considered 100% congested.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_congestion_outgoing_gas": {
///      "description": "How much gas in outgoing buffered receipts of a shard is 100% congested.\n\nOutgoing congestion contributes to overall congestion, which reduces how\nmuch other shards are allowed to forward to this shard.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "max_outgoing_gas": {
///      "description": "The maximum amount of gas attached to receipts a shard can forward to\nanother shard per chunk.\n\nSee [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "max_tx_gas": {
///      "description": "The maximum amount of gas in a chunk spent on converting new transactions to\nreceipts.\n\nSee [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "min_outgoing_gas": {
///      "description": "The minimum gas each shard can send to a shard that is not fully congested.\n\nSee [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "min_tx_gas": {
///      "description": "The minimum amount of gas in a chunk spent on converting new transactions\nto receipts, as long as the receiving shard is not congested.\n\nSee [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "outgoing_receipts_big_size_limit": {
///      "description": "Large size limit for outgoing receipts to a shard, used when it's safe\nto send a lot of receipts without making the state witness too large.\nIt limits the total sum of outgoing receipts, not individual receipts.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "outgoing_receipts_usual_size_limit": {
///      "description": "The standard size limit for outgoing receipts aimed at a single shard.\nThis limit is pretty small to keep the size of source_receipt_proofs under control.\nIt limits the total sum of outgoing receipts, not individual receipts.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "reject_tx_congestion_threshold": {
///      "description": "How much congestion a shard can tolerate before it stops all shards from\naccepting new transactions with the receiver set to the congested shard.",
///      "type": "number",
///      "format": "double"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CongestionControlConfigView {
    /**How much gas the chosen allowed shard can send to a 100% congested shard.

See [`CongestionControlConfig`] for more details.*/
    pub allowed_shard_outgoing_gas: NearGas,
    /**How much gas in delayed receipts of a shard is 100% incoming congestion.

See [`CongestionControlConfig`] for more details.*/
    pub max_congestion_incoming_gas: NearGas,
    /**How much memory space of all delayed and buffered receipts in a shard is
considered 100% congested.

See [`CongestionControlConfig`] for more details.*/
    pub max_congestion_memory_consumption: u64,
    ///How many missed chunks in a row in a shard is considered 100% congested.
    pub max_congestion_missed_chunks: u64,
    /**How much gas in outgoing buffered receipts of a shard is 100% congested.

Outgoing congestion contributes to overall congestion, which reduces how
much other shards are allowed to forward to this shard.*/
    pub max_congestion_outgoing_gas: NearGas,
    /**The maximum amount of gas attached to receipts a shard can forward to
another shard per chunk.

See [`CongestionControlConfig`] for more details.*/
    pub max_outgoing_gas: NearGas,
    /**The maximum amount of gas in a chunk spent on converting new transactions to
receipts.

See [`CongestionControlConfig`] for more details.*/
    pub max_tx_gas: NearGas,
    /**The minimum gas each shard can send to a shard that is not fully congested.

See [`CongestionControlConfig`] for more details.*/
    pub min_outgoing_gas: NearGas,
    /**The minimum amount of gas in a chunk spent on converting new transactions
to receipts, as long as the receiving shard is not congested.

See [`CongestionControlConfig`] for more details.*/
    pub min_tx_gas: NearGas,
    /**Large size limit for outgoing receipts to a shard, used when it's safe
to send a lot of receipts without making the state witness too large.
It limits the total sum of outgoing receipts, not individual receipts.*/
    pub outgoing_receipts_big_size_limit: u64,
    /**The standard size limit for outgoing receipts aimed at a single shard.
This limit is pretty small to keep the size of source_receipt_proofs under control.
It limits the total sum of outgoing receipts, not individual receipts.*/
    pub outgoing_receipts_usual_size_limit: u64,
    pub reject_tx_congestion_threshold: f64,
}
///Stores the congestion level of a shard. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CongestionInfoView",
///  "description": "Stores the congestion level of a shard. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)",
///  "type": "object",
///  "required": [
///    "allowed_shard",
///    "buffered_receipts_gas",
///    "delayed_receipts_gas",
///    "receipt_bytes"
///  ],
///  "properties": {
///    "allowed_shard": {
///      "type": "integer",
///      "format": "uint16",
///      "maximum": 65535.0,
///      "minimum": 0.0
///    },
///    "buffered_receipts_gas": {
///      "type": "string"
///    },
///    "delayed_receipts_gas": {
///      "type": "string"
///    },
///    "receipt_bytes": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CongestionInfoView {
    pub allowed_shard: u16,
    pub buffered_receipts_gas: ::std::string::String,
    pub delayed_receipts_gas: ::std::string::String,
    pub receipt_bytes: u64,
}
///A view of the contract code.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ContractCodeView",
///  "description": "A view of the contract code.",
///  "type": "object",
///  "required": [
///    "code_base64",
///    "hash"
///  ],
///  "properties": {
///    "code_base64": {
///      "type": "string"
///    },
///    "hash": {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ContractCodeView {
    pub code_base64: ::std::string::String,
    pub hash: CryptoHash,
}
///Shows gas profile. More info [here](https://near.github.io/nearcore/architecture/gas/gas_profile.html?highlight=WASM_HOST_COST#example-transaction-gas-profile).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CostGasUsed",
///  "description": "Shows gas profile. More info [here](https://near.github.io/nearcore/architecture/gas/gas_profile.html?highlight=WASM_HOST_COST#example-transaction-gas-profile).",
///  "type": "object",
///  "required": [
///    "cost",
///    "cost_category",
///    "gas_used"
///  ],
///  "properties": {
///    "cost": {
///      "type": "string"
///    },
///    "cost_category": {
///      "description": "Either ACTION_COST or WASM_HOST_COST.",
///      "type": "string"
///    },
///    "gas_used": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CostGasUsed {
    pub cost: ::std::string::String,
    ///Either ACTION_COST or WASM_HOST_COST.
    pub cost_category: ::std::string::String,
    pub gas_used: ::std::string::String,
}
///Create account action
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CreateAccountAction",
///  "description": "Create account action",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CreateAccountAction(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for CreateAccountAction {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<CreateAccountAction>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: CreateAccountAction) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for CreateAccountAction {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`CryptoHash`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CryptoHash",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct CryptoHash(pub ::std::string::String);
impl ::std::ops::Deref for CryptoHash {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CryptoHash> for ::std::string::String {
    fn from(value: CryptoHash) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for CryptoHash {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for CryptoHash {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for CryptoHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///Describes information about the current epoch validator
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CurrentEpochValidatorInfo",
///  "description": "Describes information about the current epoch validator",
///  "type": "object",
///  "required": [
///    "account_id",
///    "is_slashed",
///    "num_expected_blocks",
///    "num_produced_blocks",
///    "public_key",
///    "shards",
///    "stake"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "is_slashed": {
///      "type": "boolean"
///    },
///    "num_expected_blocks": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_expected_chunks": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_expected_chunks_per_shard": {
///      "description": "Number of chunks this validator was expected to produce in each shard.\nEach entry in the array corresponds to the shard in the `shards_produced` array.",
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_expected_endorsements": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_expected_endorsements_per_shard": {
///      "description": "Number of chunks this validator was expected to validate and endorse in each shard.\nEach entry in the array corresponds to the shard in the `shards_endorsed` array.",
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_produced_blocks": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_produced_chunks": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_produced_chunks_per_shard": {
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_produced_endorsements": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_produced_endorsements_per_shard": {
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "public_key": {
///      "$ref": "#/definitions/PublicKey"
///    },
///    "shards": {
///      "description": "Shards this validator is assigned to as chunk producer in the current epoch.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ShardId"
///      }
///    },
///    "shards_endorsed": {
///      "description": "Shards this validator is assigned to as chunk validator in the current epoch.",
///      "default": [],
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ShardId"
///      }
///    },
///    "stake": {
///      "$ref": "#/definitions/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CurrentEpochValidatorInfo {
    pub account_id: AccountId,
    pub is_slashed: bool,
    pub num_expected_blocks: u64,
    #[serde(default)]
    pub num_expected_chunks: u64,
    /**Number of chunks this validator was expected to produce in each shard.
Each entry in the array corresponds to the shard in the `shards_produced` array.*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_expected_chunks_per_shard: ::std::vec::Vec<u64>,
    #[serde(default)]
    pub num_expected_endorsements: u64,
    /**Number of chunks this validator was expected to validate and endorse in each shard.
Each entry in the array corresponds to the shard in the `shards_endorsed` array.*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_expected_endorsements_per_shard: ::std::vec::Vec<u64>,
    pub num_produced_blocks: u64,
    #[serde(default)]
    pub num_produced_chunks: u64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_produced_chunks_per_shard: ::std::vec::Vec<u64>,
    #[serde(default)]
    pub num_produced_endorsements: u64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_produced_endorsements_per_shard: ::std::vec::Vec<u64>,
    pub public_key: PublicKey,
    ///Shards this validator is assigned to as chunk producer in the current epoch.
    pub shards: ::std::vec::Vec<ShardId>,
    ///Shards this validator is assigned to as chunk validator in the current epoch.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shards_endorsed: ::std::vec::Vec<ShardId>,
    pub stake: NearToken,
}
///The fees settings for a data receipt creation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DataReceiptCreationConfigView",
///  "description": "The fees settings for a data receipt creation",
///  "type": "object",
///  "required": [
///    "base_cost",
///    "cost_per_byte"
///  ],
///  "properties": {
///    "base_cost": {
///      "description": "Base cost of creating a data receipt.\nBoth `send` and `exec` costs are burned when a new receipt has input dependencies. The gas\nis charged for each input dependency. The dependencies are specified when a receipt is\ncreated using `promise_then` and `promise_batch_then`.\nNOTE: Any receipt with output dependencies will produce data receipts. Even if it fails.\nEven if the last action is not a function call (in case of success it will return empty\nvalue).",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "cost_per_byte": {
///      "description": "Additional cost per byte sent.\nBoth `send` and `exec` costs are burned when a function call finishes execution and returns\n`N` bytes of data to every output dependency. For each output dependency the cost is\n`(send(sir) + exec()) * N`.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DataReceiptCreationConfigView {
    /**Base cost of creating a data receipt.
Both `send` and `exec` costs are burned when a new receipt has input dependencies. The gas
is charged for each input dependency. The dependencies are specified when a receipt is
created using `promise_then` and `promise_batch_then`.
NOTE: Any receipt with output dependencies will produce data receipts. Even if it fails.
Even if the last action is not a function call (in case of success it will return empty
value).*/
    pub base_cost: Fee,
    /**Additional cost per byte sent.
Both `send` and `exec` costs are burned when a function call finishes execution and returns
`N` bytes of data to every output dependency. For each output dependency the cost is
`(send(sir) + exec()) * N`.*/
    pub cost_per_byte: Fee,
}
///`DataReceiverView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DataReceiverView",
///  "type": "object",
///  "required": [
///    "data_id",
///    "receiver_id"
///  ],
///  "properties": {
///    "data_id": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "receiver_id": {
///      "$ref": "#/definitions/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DataReceiverView {
    pub data_id: CryptoHash,
    pub receiver_id: AccountId,
}
///This action allows to execute the inner actions behalf of the defined sender.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DelegateAction",
///  "description": "This action allows to execute the inner actions behalf of the defined sender.",
///  "type": "object",
///  "required": [
///    "actions",
///    "max_block_height",
///    "nonce",
///    "public_key",
///    "receiver_id",
///    "sender_id"
///  ],
///  "properties": {
///    "actions": {
///      "description": "List of actions to be executed.\n\nWith the meta transactions MVP defined in NEP-366, nested\nDelegateActions are not allowed. A separate type is used to enforce it.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/NonDelegateAction"
///      }
///    },
///    "max_block_height": {
///      "description": "The maximal height of the block in the blockchain below which the given DelegateAction is valid.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "nonce": {
///      "description": "Nonce to ensure that the same delegate action is not sent twice by a\nrelayer and should match for given account's `public_key`.\nAfter this action is processed it will increment.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "public_key": {
///      "description": "Public key used to sign this delegated action.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    },
///    "receiver_id": {
///      "description": "Receiver of the delegated actions.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        }
///      ]
///    },
///    "sender_id": {
///      "description": "Signer of the delegated actions",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DelegateAction {
    /**List of actions to be executed.

With the meta transactions MVP defined in NEP-366, nested
DelegateActions are not allowed. A separate type is used to enforce it.*/
    pub actions: ::std::vec::Vec<NonDelegateAction>,
    ///The maximal height of the block in the blockchain below which the given DelegateAction is valid.
    pub max_block_height: u64,
    /**Nonce to ensure that the same delegate action is not sent twice by a
relayer and should match for given account's `public_key`.
After this action is processed it will increment.*/
    pub nonce: u64,
    ///Public key used to sign this delegated action.
    pub public_key: PublicKey,
    ///Receiver of the delegated actions.
    pub receiver_id: AccountId,
    ///Signer of the delegated actions
    pub sender_id: AccountId,
}
///`DeleteAccountAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DeleteAccountAction",
///  "type": "object",
///  "required": [
///    "beneficiary_id"
///  ],
///  "properties": {
///    "beneficiary_id": {
///      "$ref": "#/definitions/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeleteAccountAction {
    pub beneficiary_id: AccountId,
}
///`DeleteKeyAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DeleteKeyAction",
///  "type": "object",
///  "required": [
///    "public_key"
///  ],
///  "properties": {
///    "public_key": {
///      "description": "A public key associated with the access_key to be deleted.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeleteKeyAction {
    ///A public key associated with the access_key to be deleted.
    pub public_key: PublicKey,
}
///Deploy contract action
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DeployContractAction",
///  "description": "Deploy contract action",
///  "type": "object",
///  "required": [
///    "code"
///  ],
///  "properties": {
///    "code": {
///      "description": "WebAssembly binary",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeployContractAction {
    ///WebAssembly binary
    pub code: ::std::string::String,
}
///Deploy global contract action
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DeployGlobalContractAction",
///  "description": "Deploy global contract action",
///  "type": "object",
///  "required": [
///    "code",
///    "deploy_mode"
///  ],
///  "properties": {
///    "code": {
///      "description": "WebAssembly binary",
///      "type": "string"
///    },
///    "deploy_mode": {
///      "$ref": "#/definitions/GlobalContractDeployMode"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeployGlobalContractAction {
    ///WebAssembly binary
    pub code: ::std::string::String,
    pub deploy_mode: GlobalContractDeployMode,
}
///`DetailedDebugStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DetailedDebugStatus",
///  "type": "object",
///  "required": [
///    "block_production_delay_millis",
///    "catchup_status",
///    "current_head_status",
///    "current_header_head_status",
///    "network_info",
///    "sync_status"
///  ],
///  "properties": {
///    "block_production_delay_millis": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "catchup_status": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/CatchupStatusView"
///      }
///    },
///    "current_head_status": {
///      "$ref": "#/definitions/BlockStatusView"
///    },
///    "current_header_head_status": {
///      "$ref": "#/definitions/BlockStatusView"
///    },
///    "network_info": {
///      "$ref": "#/definitions/NetworkInfoView"
///    },
///    "sync_status": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DetailedDebugStatus {
    pub block_production_delay_millis: u64,
    pub catchup_status: ::std::vec::Vec<CatchupStatusView>,
    pub current_head_status: BlockStatusView,
    pub current_header_head_status: BlockStatusView,
    pub network_info: NetworkInfoView,
    pub sync_status: ::std::string::String,
}
///`DeterministicAccountStateInit`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DeterministicAccountStateInit",
///  "oneOf": [
///    {
///      "title": "V1",
///      "type": "object",
///      "required": [
///        "V1"
///      ],
///      "properties": {
///        "V1": {
///          "$ref": "#/definitions/DeterministicAccountStateInitV1"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum DeterministicAccountStateInit {
    V1(DeterministicAccountStateInitV1),
}
impl ::std::convert::From<DeterministicAccountStateInitV1>
for DeterministicAccountStateInit {
    fn from(value: DeterministicAccountStateInitV1) -> Self {
        Self::V1(value)
    }
}
///`DeterministicAccountStateInitV1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DeterministicAccountStateInitV1",
///  "type": "object",
///  "required": [
///    "code",
///    "data"
///  ],
///  "properties": {
///    "code": {
///      "$ref": "#/definitions/GlobalContractIdentifier"
///    },
///    "data": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeterministicAccountStateInitV1 {
    pub code: GlobalContractIdentifier,
    pub data: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
///`DeterministicStateInitAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DeterministicStateInitAction",
///  "type": "object",
///  "required": [
///    "deposit",
///    "state_init"
///  ],
///  "properties": {
///    "deposit": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "state_init": {
///      "$ref": "#/definitions/DeterministicAccountStateInit"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeterministicStateInitAction {
    pub deposit: NearToken,
    pub state_init: DeterministicAccountStateInit,
}
///`Direction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Direction",
///  "type": "string",
///  "enum": [
///    "Left",
///    "Right"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum Direction {
    Left,
    Right,
}
impl ::std::fmt::Display for Direction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Left => f.write_str("Left"),
            Self::Right => f.write_str("Right"),
        }
    }
}
impl ::std::str::FromStr for Direction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Left" => Ok(Self::Left),
            "Right" => Ok(Self::Right),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Configures how to dump state to external storage.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DumpConfig",
///  "description": "Configures how to dump state to external storage.",
///  "type": "object",
///  "required": [
///    "location"
///  ],
///  "properties": {
///    "credentials_file": {
///      "description": "Location of a json file with credentials allowing access to the bucket.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "iteration_delay": {
///      "description": "How often to check if a new epoch has started.\nFeel free to set to `None`, defaults are sensible.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/DurationAsStdSchemaProvider"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "location": {
///      "description": "Specifies where to write the obtained state parts.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ExternalStorageLocation"
///        }
///      ]
///    },
///    "restart_dump_for_shards": {
///      "description": "Use in case a node that dumps state to the external storage\ngets in trouble.",
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/definitions/ShardId"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DumpConfig {
    ///Location of a json file with credentials allowing access to the bucket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credentials_file: ::std::option::Option<::std::string::String>,
    /**How often to check if a new epoch has started.
Feel free to set to `None`, defaults are sensible.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub iteration_delay: ::std::option::Option<DurationAsStdSchemaProvider>,
    ///Specifies where to write the obtained state parts.
    pub location: ExternalStorageLocation,
    /**Use in case a node that dumps state to the external storage
gets in trouble.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub restart_dump_for_shards: ::std::option::Option<::std::vec::Vec<ShardId>>,
}
///`DurationAsStdSchemaProvider`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DurationAsStdSchemaProvider",
///  "type": "object",
///  "required": [
///    "nanos",
///    "secs"
///  ],
///  "properties": {
///    "nanos": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "secs": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DurationAsStdSchemaProvider {
    pub nanos: i32,
    pub secs: i64,
}
/**Epoch identifier -- wrapped hash, to make it easier to distinguish.
EpochId of epoch T is the hash of last block in T-2
EpochId of first two epochs is 0*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "EpochId",
///  "description": "Epoch identifier -- wrapped hash, to make it easier to distinguish.\nEpochId of epoch T is the hash of last block in T-2\nEpochId of first two epochs is 0",
///  "allOf": [
///    {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EpochId(pub CryptoHash);
impl ::std::ops::Deref for EpochId {
    type Target = CryptoHash;
    fn deref(&self) -> &CryptoHash {
        &self.0
    }
}
impl ::std::convert::From<EpochId> for CryptoHash {
    fn from(value: EpochId) -> Self {
        value.0
    }
}
impl ::std::convert::From<CryptoHash> for EpochId {
    fn from(value: CryptoHash) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for EpochId {
    type Err = <CryptoHash as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for EpochId {
    type Error = <CryptoHash as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EpochId {
    type Error = <CryptoHash as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for EpochId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`EpochSyncConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "EpochSyncConfig",
///  "type": "object",
///  "required": [
///    "epoch_sync_horizon",
///    "timeout_for_epoch_sync"
///  ],
///  "properties": {
///    "epoch_sync_horizon": {
///      "description": "This serves as two purposes: (1) the node will not epoch sync and instead resort to\nheader sync, if the genesis block is within this many blocks from the current block;\n(2) the node will reject an epoch sync proof if the provided proof is for an epoch\nthat is more than this many blocks behind the current block.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timeout_for_epoch_sync": {
///      "description": "Timeout for epoch sync requests. The node will continue retrying indefinitely even\nif this timeout is exceeded.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/DurationAsStdSchemaProvider"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct EpochSyncConfig {
    /**This serves as two purposes: (1) the node will not epoch sync and instead resort to
header sync, if the genesis block is within this many blocks from the current block;
(2) the node will reject an epoch sync proof if the provided proof is for an epoch
that is more than this many blocks behind the current block.*/
    pub epoch_sync_horizon: u64,
    /**Timeout for epoch sync requests. The node will continue retrying indefinitely even
if this timeout is exceeded.*/
    pub timeout_for_epoch_sync: DurationAsStdSchemaProvider,
}
///`ExecutionMetadataView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ExecutionMetadataView",
///  "type": "object",
///  "required": [
///    "version"
///  ],
///  "properties": {
///    "gas_profile": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/definitions/CostGasUsed"
///      }
///    },
///    "version": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExecutionMetadataView {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gas_profile: ::std::option::Option<::std::vec::Vec<CostGasUsed>>,
    pub version: u32,
}
///`ExecutionOutcomeView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ExecutionOutcomeView",
///  "type": "object",
///  "required": [
///    "executor_id",
///    "gas_burnt",
///    "logs",
///    "receipt_ids",
///    "status",
///    "tokens_burnt"
///  ],
///  "properties": {
///    "executor_id": {
///      "description": "The id of the account on which the execution happens. For transaction this is signer_id,\nfor receipt this is receiver_id.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        }
///      ]
///    },
///    "gas_burnt": {
///      "description": "The amount of the gas burnt by the given transaction or receipt.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "logs": {
///      "description": "Logs from this transaction or receipt.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "metadata": {
///      "description": "Execution metadata, versioned",
///      "default": {
///        "gas_profile": null,
///        "version": 1
///      },
///      "allOf": [
///        {
///          "$ref": "#/definitions/ExecutionMetadataView"
///        }
///      ]
///    },
///    "receipt_ids": {
///      "description": "Receipt IDs generated by this transaction or receipt.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/CryptoHash"
///      }
///    },
///    "status": {
///      "description": "Execution status. Contains the result in case of successful execution.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ExecutionStatusView"
///        }
///      ]
///    },
///    "tokens_burnt": {
///      "description": "The amount of tokens burnt corresponding to the burnt gas amount.\nThis value doesn't always equal to the `gas_burnt` multiplied by the gas price, because\nthe prepaid gas price might be lower than the actual gas price and it creates a deficit.\n`tokens_burnt` also contains the penalty subtracted from refunds, while\n`gas_burnt` only contains the gas that we actually burn for the execution.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExecutionOutcomeView {
    /**The id of the account on which the execution happens. For transaction this is signer_id,
for receipt this is receiver_id.*/
    pub executor_id: AccountId,
    ///The amount of the gas burnt by the given transaction or receipt.
    pub gas_burnt: NearGas,
    ///Logs from this transaction or receipt.
    pub logs: ::std::vec::Vec<::std::string::String>,
    ///Execution metadata, versioned
    #[serde(default = "defaults::execution_outcome_view_metadata")]
    pub metadata: ExecutionMetadataView,
    ///Receipt IDs generated by this transaction or receipt.
    pub receipt_ids: ::std::vec::Vec<CryptoHash>,
    ///Execution status. Contains the result in case of successful execution.
    pub status: ExecutionStatusView,
    /**The amount of tokens burnt corresponding to the burnt gas amount.
This value doesn't always equal to the `gas_burnt` multiplied by the gas price, because
the prepaid gas price might be lower than the actual gas price and it creates a deficit.
`tokens_burnt` also contains the penalty subtracted from refunds, while
`gas_burnt` only contains the gas that we actually burn for the execution.*/
    pub tokens_burnt: NearToken,
}
///`ExecutionOutcomeWithIdView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ExecutionOutcomeWithIdView",
///  "type": "object",
///  "required": [
///    "block_hash",
///    "id",
///    "outcome",
///    "proof"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "id": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "outcome": {
///      "$ref": "#/definitions/ExecutionOutcomeView"
///    },
///    "proof": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/MerklePathItem"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExecutionOutcomeWithIdView {
    pub block_hash: CryptoHash,
    pub id: CryptoHash,
    pub outcome: ExecutionOutcomeView,
    pub proof: ::std::vec::Vec<MerklePathItem>,
}
///`ExecutionStatusView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ExecutionStatusView",
///  "oneOf": [
///    {
///      "description": "The execution is pending or unknown.",
///      "type": "string",
///      "const": "Unknown"
///    },
///    {
///      "title": "Failure",
///      "description": "The execution has failed.",
///      "type": "object",
///      "required": [
///        "Failure"
///      ],
///      "properties": {
///        "Failure": {
///          "$ref": "#/definitions/TxExecutionError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "SuccessValue",
///      "description": "The final action succeeded and returned some value or an empty vec encoded in base64.",
///      "type": "object",
///      "required": [
///        "SuccessValue"
///      ],
///      "properties": {
///        "SuccessValue": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "SuccessReceiptId",
///      "description": "The final action of the receipt returned a promise or the signed transaction was converted\nto a receipt. Contains the receipt_id of the generated receipt.",
///      "type": "object",
///      "required": [
///        "SuccessReceiptId"
///      ],
///      "properties": {
///        "SuccessReceiptId": {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ExecutionStatusView {
    ///The execution is pending or unknown.
    Unknown,
    ///The execution has failed.
    Failure(TxExecutionError),
    ///The final action succeeded and returned some value or an empty vec encoded in base64.
    SuccessValue(::std::string::String),
    /**The final action of the receipt returned a promise or the signed transaction was converted
to a receipt. Contains the receipt_id of the generated receipt.*/
    SuccessReceiptId(CryptoHash),
}
impl ::std::convert::From<TxExecutionError> for ExecutionStatusView {
    fn from(value: TxExecutionError) -> Self {
        Self::Failure(value)
    }
}
impl ::std::convert::From<CryptoHash> for ExecutionStatusView {
    fn from(value: CryptoHash) -> Self {
        Self::SuccessReceiptId(value)
    }
}
/**Typed view of ExtCostsConfig to preserve JSON output field names in protocol
config RPC output.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ExtCostsConfigView",
///  "description": "Typed view of ExtCostsConfig to preserve JSON output field names in protocol\nconfig RPC output.",
///  "type": "object",
///  "required": [
///    "alt_bn128_g1_multiexp_base",
///    "alt_bn128_g1_multiexp_element",
///    "alt_bn128_g1_sum_base",
///    "alt_bn128_g1_sum_element",
///    "alt_bn128_pairing_check_base",
///    "alt_bn128_pairing_check_element",
///    "base",
///    "bls12381_g1_multiexp_base",
///    "bls12381_g1_multiexp_element",
///    "bls12381_g2_multiexp_base",
///    "bls12381_g2_multiexp_element",
///    "bls12381_map_fp2_to_g2_base",
///    "bls12381_map_fp2_to_g2_element",
///    "bls12381_map_fp_to_g1_base",
///    "bls12381_map_fp_to_g1_element",
///    "bls12381_p1_decompress_base",
///    "bls12381_p1_decompress_element",
///    "bls12381_p1_sum_base",
///    "bls12381_p1_sum_element",
///    "bls12381_p2_decompress_base",
///    "bls12381_p2_decompress_element",
///    "bls12381_p2_sum_base",
///    "bls12381_p2_sum_element",
///    "bls12381_pairing_base",
///    "bls12381_pairing_element",
///    "contract_compile_base",
///    "contract_compile_bytes",
///    "contract_loading_base",
///    "contract_loading_bytes",
///    "ecrecover_base",
///    "ed25519_verify_base",
///    "ed25519_verify_byte",
///    "keccak256_base",
///    "keccak256_byte",
///    "keccak512_base",
///    "keccak512_byte",
///    "log_base",
///    "log_byte",
///    "promise_and_base",
///    "promise_and_per_promise",
///    "promise_return",
///    "read_cached_trie_node",
///    "read_memory_base",
///    "read_memory_byte",
///    "read_register_base",
///    "read_register_byte",
///    "ripemd160_base",
///    "ripemd160_block",
///    "sha256_base",
///    "sha256_byte",
///    "storage_has_key_base",
///    "storage_has_key_byte",
///    "storage_iter_create_from_byte",
///    "storage_iter_create_prefix_base",
///    "storage_iter_create_prefix_byte",
///    "storage_iter_create_range_base",
///    "storage_iter_create_to_byte",
///    "storage_iter_next_base",
///    "storage_iter_next_key_byte",
///    "storage_iter_next_value_byte",
///    "storage_large_read_overhead_base",
///    "storage_large_read_overhead_byte",
///    "storage_read_base",
///    "storage_read_key_byte",
///    "storage_read_value_byte",
///    "storage_remove_base",
///    "storage_remove_key_byte",
///    "storage_remove_ret_value_byte",
///    "storage_write_base",
///    "storage_write_evicted_byte",
///    "storage_write_key_byte",
///    "storage_write_value_byte",
///    "touching_trie_node",
///    "utf16_decoding_base",
///    "utf16_decoding_byte",
///    "utf8_decoding_base",
///    "utf8_decoding_byte",
///    "validator_stake_base",
///    "validator_total_stake_base",
///    "write_memory_base",
///    "write_memory_byte",
///    "write_register_base",
///    "write_register_byte",
///    "yield_create_base",
///    "yield_create_byte",
///    "yield_resume_base",
///    "yield_resume_byte"
///  ],
///  "properties": {
///    "alt_bn128_g1_multiexp_base": {
///      "description": "Base cost for multiexp",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_g1_multiexp_element": {
///      "description": "Per element cost for multiexp",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_g1_sum_base": {
///      "description": "Base cost for sum",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_g1_sum_element": {
///      "description": "Per element cost for sum",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_pairing_check_base": {
///      "description": "Base cost for pairing check",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_pairing_check_element": {
///      "description": "Per element cost for pairing check",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "base": {
///      "description": "Base cost for calling a host function.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "bls12381_g1_multiexp_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_g1_multiexp_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_g2_multiexp_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_g2_multiexp_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_map_fp2_to_g2_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_map_fp2_to_g2_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_map_fp_to_g1_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_map_fp_to_g1_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_p1_decompress_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_p1_decompress_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_p1_sum_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_p1_sum_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_p2_decompress_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_p2_decompress_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_p2_sum_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_p2_sum_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_pairing_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "bls12381_pairing_element": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "contract_compile_base": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "contract_compile_bytes": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "contract_loading_base": {
///      "description": "Base cost of loading a pre-compiled contract",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "contract_loading_bytes": {
///      "description": "Cost per byte of loading a pre-compiled contract",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "ecrecover_base": {
///      "description": "Cost of calling ecrecover",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "ed25519_verify_base": {
///      "description": "Cost of getting ed25519 base",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "ed25519_verify_byte": {
///      "description": "Cost of getting ed25519 per byte",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "keccak256_base": {
///      "description": "Cost of getting sha256 base",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "keccak256_byte": {
///      "description": "Cost of getting sha256 per byte",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "keccak512_base": {
///      "description": "Cost of getting sha256 base",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "keccak512_byte": {
///      "description": "Cost of getting sha256 per byte",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "log_base": {
///      "description": "Cost for calling logging.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "log_byte": {
///      "description": "Cost for logging per byte",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "promise_and_base": {
///      "description": "Cost for calling `promise_and`",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "promise_and_per_promise": {
///      "description": "Cost for calling `promise_and` for each promise",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "promise_return": {
///      "description": "Cost for calling `promise_return`",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "read_cached_trie_node": {
///      "description": "Cost for reading trie node from memory",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "read_memory_base": {
///      "description": "Base cost for guest memory read",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "read_memory_byte": {
///      "description": "Cost for guest memory read",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "read_register_base": {
///      "description": "Base cost for reading from register",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "read_register_byte": {
///      "description": "Cost for reading byte from register",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "ripemd160_base": {
///      "description": "Cost of getting ripemd160 base",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "ripemd160_block": {
///      "description": "Cost of getting ripemd160 per message block",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "sha256_base": {
///      "description": "Cost of getting sha256 base",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "sha256_byte": {
///      "description": "Cost of getting sha256 per byte",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_has_key_base": {
///      "description": "Storage trie check for key existence cost base",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_has_key_byte": {
///      "description": "Storage trie check for key existence per key byte",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_from_byte": {
///      "description": "Create trie range iterator cost per byte of from key.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_prefix_base": {
///      "description": "Create trie prefix iterator cost base",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_prefix_byte": {
///      "description": "Create trie prefix iterator cost per byte.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_range_base": {
///      "description": "Create trie range iterator cost base",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_to_byte": {
///      "description": "Create trie range iterator cost per byte of to key.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_iter_next_base": {
///      "description": "Trie iterator per key base cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_iter_next_key_byte": {
///      "description": "Trie iterator next key byte cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_iter_next_value_byte": {
///      "description": "Trie iterator next key byte cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_large_read_overhead_base": {
///      "description": "Storage trie read key overhead base cost, when doing large reads",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_large_read_overhead_byte": {
///      "description": "Storage trie read key overhead  per-byte cost, when doing large reads",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_read_base": {
///      "description": "Storage trie read key base cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_read_key_byte": {
///      "description": "Storage trie read key per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_read_value_byte": {
///      "description": "Storage trie read value cost per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_remove_base": {
///      "description": "Remove key from trie base cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_remove_key_byte": {
///      "description": "Remove key from trie per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_remove_ret_value_byte": {
///      "description": "Remove key from trie ret value byte cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_write_base": {
///      "description": "Storage trie write key base cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_write_evicted_byte": {
///      "description": "Storage trie write cost per byte of evicted value.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_write_key_byte": {
///      "description": "Storage trie write key per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "storage_write_value_byte": {
///      "description": "Storage trie write value per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "touching_trie_node": {
///      "description": "Cost per reading trie node from DB",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "utf16_decoding_base": {
///      "description": "Base cost of decoding utf16. It's used for `log_utf16`.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "utf16_decoding_byte": {
///      "description": "Cost per byte of decoding utf16. It's used for `log_utf16`.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "utf8_decoding_base": {
///      "description": "Base cost of decoding utf8. It's used for `log_utf8` and `panic_utf8`.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "utf8_decoding_byte": {
///      "description": "Cost per byte of decoding utf8. It's used for `log_utf8` and `panic_utf8`.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "validator_stake_base": {
///      "description": "Cost of calling `validator_stake`.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "validator_total_stake_base": {
///      "description": "Cost of calling `validator_total_stake`.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "write_memory_base": {
///      "description": "Base cost for guest memory write",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "write_memory_byte": {
///      "description": "Cost for guest memory write per byte",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "write_register_base": {
///      "description": "Base cost for writing into register",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "write_register_byte": {
///      "description": "Cost for writing byte into register",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "yield_create_base": {
///      "description": "Base cost for creating a yield promise.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "yield_create_byte": {
///      "description": "Per byte cost of arguments and method name.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "yield_resume_base": {
///      "description": "Base cost for resuming a yield receipt.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "yield_resume_byte": {
///      "description": "Per byte cost of resume payload.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExtCostsConfigView {
    ///Base cost for multiexp
    pub alt_bn128_g1_multiexp_base: NearGas,
    ///Per element cost for multiexp
    pub alt_bn128_g1_multiexp_element: NearGas,
    ///Base cost for sum
    pub alt_bn128_g1_sum_base: NearGas,
    ///Per element cost for sum
    pub alt_bn128_g1_sum_element: NearGas,
    ///Base cost for pairing check
    pub alt_bn128_pairing_check_base: NearGas,
    ///Per element cost for pairing check
    pub alt_bn128_pairing_check_element: NearGas,
    ///Base cost for calling a host function.
    pub base: NearGas,
    pub bls12381_g1_multiexp_base: NearGas,
    pub bls12381_g1_multiexp_element: NearGas,
    pub bls12381_g2_multiexp_base: NearGas,
    pub bls12381_g2_multiexp_element: NearGas,
    pub bls12381_map_fp2_to_g2_base: NearGas,
    pub bls12381_map_fp2_to_g2_element: NearGas,
    pub bls12381_map_fp_to_g1_base: NearGas,
    pub bls12381_map_fp_to_g1_element: NearGas,
    pub bls12381_p1_decompress_base: NearGas,
    pub bls12381_p1_decompress_element: NearGas,
    pub bls12381_p1_sum_base: NearGas,
    pub bls12381_p1_sum_element: NearGas,
    pub bls12381_p2_decompress_base: NearGas,
    pub bls12381_p2_decompress_element: NearGas,
    pub bls12381_p2_sum_base: NearGas,
    pub bls12381_p2_sum_element: NearGas,
    pub bls12381_pairing_base: NearGas,
    pub bls12381_pairing_element: NearGas,
    pub contract_compile_base: NearGas,
    pub contract_compile_bytes: NearGas,
    ///Base cost of loading a pre-compiled contract
    pub contract_loading_base: NearGas,
    ///Cost per byte of loading a pre-compiled contract
    pub contract_loading_bytes: NearGas,
    ///Cost of calling ecrecover
    pub ecrecover_base: NearGas,
    ///Cost of getting ed25519 base
    pub ed25519_verify_base: NearGas,
    ///Cost of getting ed25519 per byte
    pub ed25519_verify_byte: NearGas,
    ///Cost of getting sha256 base
    pub keccak256_base: NearGas,
    ///Cost of getting sha256 per byte
    pub keccak256_byte: NearGas,
    ///Cost of getting sha256 base
    pub keccak512_base: NearGas,
    ///Cost of getting sha256 per byte
    pub keccak512_byte: NearGas,
    ///Cost for calling logging.
    pub log_base: NearGas,
    ///Cost for logging per byte
    pub log_byte: NearGas,
    ///Cost for calling `promise_and`
    pub promise_and_base: NearGas,
    ///Cost for calling `promise_and` for each promise
    pub promise_and_per_promise: NearGas,
    ///Cost for calling `promise_return`
    pub promise_return: NearGas,
    ///Cost for reading trie node from memory
    pub read_cached_trie_node: NearGas,
    ///Base cost for guest memory read
    pub read_memory_base: NearGas,
    ///Cost for guest memory read
    pub read_memory_byte: NearGas,
    ///Base cost for reading from register
    pub read_register_base: NearGas,
    ///Cost for reading byte from register
    pub read_register_byte: NearGas,
    ///Cost of getting ripemd160 base
    pub ripemd160_base: NearGas,
    ///Cost of getting ripemd160 per message block
    pub ripemd160_block: NearGas,
    ///Cost of getting sha256 base
    pub sha256_base: NearGas,
    ///Cost of getting sha256 per byte
    pub sha256_byte: NearGas,
    ///Storage trie check for key existence cost base
    pub storage_has_key_base: NearGas,
    ///Storage trie check for key existence per key byte
    pub storage_has_key_byte: NearGas,
    ///Create trie range iterator cost per byte of from key.
    pub storage_iter_create_from_byte: NearGas,
    ///Create trie prefix iterator cost base
    pub storage_iter_create_prefix_base: NearGas,
    ///Create trie prefix iterator cost per byte.
    pub storage_iter_create_prefix_byte: NearGas,
    ///Create trie range iterator cost base
    pub storage_iter_create_range_base: NearGas,
    ///Create trie range iterator cost per byte of to key.
    pub storage_iter_create_to_byte: NearGas,
    ///Trie iterator per key base cost
    pub storage_iter_next_base: NearGas,
    ///Trie iterator next key byte cost
    pub storage_iter_next_key_byte: NearGas,
    ///Trie iterator next key byte cost
    pub storage_iter_next_value_byte: NearGas,
    ///Storage trie read key overhead base cost, when doing large reads
    pub storage_large_read_overhead_base: NearGas,
    ///Storage trie read key overhead  per-byte cost, when doing large reads
    pub storage_large_read_overhead_byte: NearGas,
    ///Storage trie read key base cost
    pub storage_read_base: NearGas,
    ///Storage trie read key per byte cost
    pub storage_read_key_byte: NearGas,
    ///Storage trie read value cost per byte cost
    pub storage_read_value_byte: NearGas,
    ///Remove key from trie base cost
    pub storage_remove_base: NearGas,
    ///Remove key from trie per byte cost
    pub storage_remove_key_byte: NearGas,
    ///Remove key from trie ret value byte cost
    pub storage_remove_ret_value_byte: NearGas,
    ///Storage trie write key base cost
    pub storage_write_base: NearGas,
    ///Storage trie write cost per byte of evicted value.
    pub storage_write_evicted_byte: NearGas,
    ///Storage trie write key per byte cost
    pub storage_write_key_byte: NearGas,
    ///Storage trie write value per byte cost
    pub storage_write_value_byte: NearGas,
    ///Cost per reading trie node from DB
    pub touching_trie_node: NearGas,
    ///Base cost of decoding utf16. It's used for `log_utf16`.
    pub utf16_decoding_base: NearGas,
    ///Cost per byte of decoding utf16. It's used for `log_utf16`.
    pub utf16_decoding_byte: NearGas,
    ///Base cost of decoding utf8. It's used for `log_utf8` and `panic_utf8`.
    pub utf8_decoding_base: NearGas,
    ///Cost per byte of decoding utf8. It's used for `log_utf8` and `panic_utf8`.
    pub utf8_decoding_byte: NearGas,
    ///Cost of calling `validator_stake`.
    pub validator_stake_base: NearGas,
    ///Cost of calling `validator_total_stake`.
    pub validator_total_stake_base: NearGas,
    ///Base cost for guest memory write
    pub write_memory_base: NearGas,
    ///Cost for guest memory write per byte
    pub write_memory_byte: NearGas,
    ///Base cost for writing into register
    pub write_register_base: NearGas,
    ///Cost for writing byte into register
    pub write_register_byte: NearGas,
    ///Base cost for creating a yield promise.
    pub yield_create_base: NearGas,
    ///Per byte cost of arguments and method name.
    pub yield_create_byte: NearGas,
    ///Base cost for resuming a yield receipt.
    pub yield_resume_base: NearGas,
    ///Per byte cost of resume payload.
    pub yield_resume_byte: NearGas,
}
///`ExternalStorageConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ExternalStorageConfig",
///  "type": "object",
///  "required": [
///    "location"
///  ],
///  "properties": {
///    "external_storage_fallback_threshold": {
///      "description": "The number of attempts the node will make to obtain a part from peers in\nthe network before it fetches from external storage.",
///      "default": 3,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "location": {
///      "description": "Location of state parts.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ExternalStorageLocation"
///        }
///      ]
///    },
///    "num_concurrent_requests": {
///      "description": "When fetching state parts from external storage, throttle fetch requests\nto this many concurrent requests.",
///      "default": 25,
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "num_concurrent_requests_during_catchup": {
///      "description": "During catchup, the node will use a different number of concurrent requests\nto reduce the performance impact of state sync.",
///      "default": 5,
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExternalStorageConfig {
    /**The number of attempts the node will make to obtain a part from peers in
the network before it fetches from external storage.*/
    #[serde(default = "defaults::default_u64::<u64, 3>")]
    pub external_storage_fallback_threshold: u64,
    ///Location of state parts.
    pub location: ExternalStorageLocation,
    /**When fetching state parts from external storage, throttle fetch requests
to this many concurrent requests.*/
    #[serde(default = "defaults::default_u64::<u8, 25>")]
    pub num_concurrent_requests: u8,
    /**During catchup, the node will use a different number of concurrent requests
to reduce the performance impact of state sync.*/
    #[serde(default = "defaults::default_u64::<u8, 5>")]
    pub num_concurrent_requests_during_catchup: u8,
}
///Supported external storage backends and their minimal config.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ExternalStorageLocation",
///  "description": "Supported external storage backends and their minimal config.",
///  "oneOf": [
///    {
///      "title": "S3",
///      "type": "object",
///      "required": [
///        "S3"
///      ],
///      "properties": {
///        "S3": {
///          "type": "object",
///          "required": [
///            "bucket",
///            "region"
///          ],
///          "properties": {
///            "bucket": {
///              "description": "Location on S3.",
///              "type": "string"
///            },
///            "region": {
///              "description": "Data may only be available in certain locations.",
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Filesystem",
///      "description": "Local filesystem root for storing data.",
///      "type": "object",
///      "required": [
///        "Filesystem"
///      ],
///      "properties": {
///        "Filesystem": {
///          "type": "object",
///          "required": [
///            "root_dir"
///          ],
///          "properties": {
///            "root_dir": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "GCS",
///      "description": "Google Cloud Storage bucket name.",
///      "type": "object",
///      "required": [
///        "GCS"
///      ],
///      "properties": {
///        "GCS": {
///          "type": "object",
///          "required": [
///            "bucket"
///          ],
///          "properties": {
///            "bucket": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ExternalStorageLocation {
    S3 {
        ///Location on S3.
        bucket: ::std::string::String,
        ///Data may only be available in certain locations.
        region: ::std::string::String,
    },
    ///Local filesystem root for storing data.
    Filesystem { root_dir: ::std::string::String },
    ///Google Cloud Storage bucket name.
    #[serde(rename = "GCS")]
    Gcs { bucket: ::std::string::String },
}
/**Costs associated with an object that can only be sent over the network (and executed
by the receiver).
NOTE: `send_sir` or `send_not_sir` fees are usually burned when the item is being created.
And `execution` fee is burned when the item is being executed.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Fee",
///  "description": "Costs associated with an object that can only be sent over the network (and executed\nby the receiver).\nNOTE: `send_sir` or `send_not_sir` fees are usually burned when the item is being created.\nAnd `execution` fee is burned when the item is being executed.",
///  "type": "object",
///  "required": [
///    "execution",
///    "send_not_sir",
///    "send_sir"
///  ],
///  "properties": {
///    "execution": {
///      "description": "Fee for executing the object.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "send_not_sir": {
///      "description": "Fee for sending an object potentially across the shards.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "send_sir": {
///      "description": "Fee for sending an object from the sender to itself, guaranteeing that it does not leave\nthe shard.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Fee {
    ///Fee for executing the object.
    pub execution: NearGas,
    ///Fee for sending an object potentially across the shards.
    pub send_not_sir: NearGas,
    /**Fee for sending an object from the sender to itself, guaranteeing that it does not leave
the shard.*/
    pub send_sir: NearGas,
}
/**Execution outcome of the transaction and all the subsequent receipts.
Could be not finalized yet*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "FinalExecutionOutcomeView",
///  "description": "Execution outcome of the transaction and all the subsequent receipts.\nCould be not finalized yet",
///  "type": "object",
///  "required": [
///    "receipts_outcome",
///    "status",
///    "transaction",
///    "transaction_outcome"
///  ],
///  "properties": {
///    "receipts_outcome": {
///      "description": "The execution outcome of receipts.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ExecutionOutcomeWithIdView"
///      }
///    },
///    "status": {
///      "description": "Execution status defined by chain.rs:get_final_transaction_result\nFinalExecutionStatus::NotStarted - the tx is not converted to the receipt yet\nFinalExecutionStatus::Started - we have at least 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished the execution\nFinalExecutionStatus::Failure - the result of the first leaf receipt_id\nFinalExecutionStatus::SuccessValue - the result of the first leaf receipt_id",
///      "allOf": [
///        {
///          "$ref": "#/definitions/FinalExecutionStatus"
///        }
///      ]
///    },
///    "transaction": {
///      "description": "Signed Transaction",
///      "allOf": [
///        {
///          "$ref": "#/definitions/SignedTransactionView"
///        }
///      ]
///    },
///    "transaction_outcome": {
///      "description": "The execution outcome of the signed transaction.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ExecutionOutcomeWithIdView"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FinalExecutionOutcomeView {
    ///The execution outcome of receipts.
    pub receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
    /**Execution status defined by chain.rs:get_final_transaction_result
FinalExecutionStatus::NotStarted - the tx is not converted to the receipt yet
FinalExecutionStatus::Started - we have at least 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished the execution
FinalExecutionStatus::Failure - the result of the first leaf receipt_id
FinalExecutionStatus::SuccessValue - the result of the first leaf receipt_id*/
    pub status: FinalExecutionStatus,
    ///Signed Transaction
    pub transaction: SignedTransactionView,
    ///The execution outcome of the signed transaction.
    pub transaction_outcome: ExecutionOutcomeWithIdView,
}
/**Final execution outcome of the transaction and all of subsequent the receipts. Also includes
the generated receipt.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "FinalExecutionOutcomeWithReceiptView",
///  "description": "Final execution outcome of the transaction and all of subsequent the receipts. Also includes\nthe generated receipt.",
///  "type": "object",
///  "required": [
///    "receipts",
///    "receipts_outcome",
///    "status",
///    "transaction",
///    "transaction_outcome"
///  ],
///  "properties": {
///    "receipts": {
///      "description": "Receipts generated from the transaction",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ReceiptView"
///      }
///    },
///    "receipts_outcome": {
///      "description": "The execution outcome of receipts.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ExecutionOutcomeWithIdView"
///      }
///    },
///    "status": {
///      "description": "Execution status defined by chain.rs:get_final_transaction_result\nFinalExecutionStatus::NotStarted - the tx is not converted to the receipt yet\nFinalExecutionStatus::Started - we have at least 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished the execution\nFinalExecutionStatus::Failure - the result of the first leaf receipt_id\nFinalExecutionStatus::SuccessValue - the result of the first leaf receipt_id",
///      "allOf": [
///        {
///          "$ref": "#/definitions/FinalExecutionStatus"
///        }
///      ]
///    },
///    "transaction": {
///      "description": "Signed Transaction",
///      "allOf": [
///        {
///          "$ref": "#/definitions/SignedTransactionView"
///        }
///      ]
///    },
///    "transaction_outcome": {
///      "description": "The execution outcome of the signed transaction.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ExecutionOutcomeWithIdView"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FinalExecutionOutcomeWithReceiptView {
    ///Receipts generated from the transaction
    pub receipts: ::std::vec::Vec<ReceiptView>,
    ///The execution outcome of receipts.
    pub receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
    /**Execution status defined by chain.rs:get_final_transaction_result
FinalExecutionStatus::NotStarted - the tx is not converted to the receipt yet
FinalExecutionStatus::Started - we have at least 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished the execution
FinalExecutionStatus::Failure - the result of the first leaf receipt_id
FinalExecutionStatus::SuccessValue - the result of the first leaf receipt_id*/
    pub status: FinalExecutionStatus,
    ///Signed Transaction
    pub transaction: SignedTransactionView,
    ///The execution outcome of the signed transaction.
    pub transaction_outcome: ExecutionOutcomeWithIdView,
}
///`FinalExecutionStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "FinalExecutionStatus",
///  "oneOf": [
///    {
///      "description": "The execution has not yet started.",
///      "type": "string",
///      "const": "NotStarted"
///    },
///    {
///      "description": "The execution has started and still going.",
///      "type": "string",
///      "const": "Started"
///    },
///    {
///      "title": "Failure",
///      "description": "The execution has failed with the given error.",
///      "type": "object",
///      "required": [
///        "Failure"
///      ],
///      "properties": {
///        "Failure": {
///          "$ref": "#/definitions/TxExecutionError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "SuccessValue",
///      "description": "The execution has succeeded and returned some value or an empty vec encoded in base64.",
///      "type": "object",
///      "required": [
///        "SuccessValue"
///      ],
///      "properties": {
///        "SuccessValue": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum FinalExecutionStatus {
    ///The execution has not yet started.
    NotStarted,
    ///The execution has started and still going.
    Started,
    ///The execution has failed with the given error.
    Failure(TxExecutionError),
    ///The execution has succeeded and returned some value or an empty vec encoded in base64.
    SuccessValue(::std::string::String),
}
impl ::std::convert::From<TxExecutionError> for FinalExecutionStatus {
    fn from(value: TxExecutionError) -> Self {
        Self::Failure(value)
    }
}
///Different types of finality.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Finality",
///  "description": "Different types of finality.",
///  "type": "string",
///  "enum": [
///    "optimistic",
///    "near-final",
///    "final"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum Finality {
    #[serde(rename = "optimistic")]
    Optimistic,
    #[serde(rename = "near-final")]
    NearFinal,
    #[serde(rename = "final")]
    Final,
}
impl ::std::fmt::Display for Finality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Optimistic => f.write_str("optimistic"),
            Self::NearFinal => f.write_str("near-final"),
            Self::Final => f.write_str("final"),
        }
    }
}
impl ::std::str::FromStr for Finality {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "optimistic" => Ok(Self::Optimistic),
            "near-final" => Ok(Self::NearFinal),
            "final" => Ok(Self::Final),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Finality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Finality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Finality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**This type is used to mark function arguments.

NOTE: The main reason for this to exist (except the type-safety) is that the value is
transparently serialized and deserialized as a base64-encoded string when serde is used
(serde_json).*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "FunctionArgs",
///  "description": "This type is used to mark function arguments.\n\nNOTE: The main reason for this to exist (except the type-safety) is that the value is\ntransparently serialized and deserialized as a base64-encoded string when serde is used\n(serde_json).",
///  "type": "string",
///  "format": "bytes"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct FunctionArgs(pub ::std::string::String);
impl ::std::ops::Deref for FunctionArgs {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionArgs> for ::std::string::String {
    fn from(value: FunctionArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for FunctionArgs {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for FunctionArgs {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for FunctionArgs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`FunctionCallAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "FunctionCallAction",
///  "type": "object",
///  "required": [
///    "args",
///    "deposit",
///    "gas",
///    "method_name"
///  ],
///  "properties": {
///    "args": {
///      "type": "string"
///    },
///    "deposit": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "gas": {
///      "$ref": "#/definitions/NearGas"
///    },
///    "method_name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FunctionCallAction {
    pub args: ::std::string::String,
    pub deposit: NearToken,
    pub gas: NearGas,
    pub method_name: ::std::string::String,
}
/**Serializable version of `near-vm-runner::FunctionCallError`.

Must never reorder/remove elements, can only add new variants at the end (but do that very
carefully). It describes stable serialization format, and only used by serialization logic.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "FunctionCallError",
///  "description": "Serializable version of `near-vm-runner::FunctionCallError`.\n\nMust never reorder/remove elements, can only add new variants at the end (but do that very\ncarefully). It describes stable serialization format, and only used by serialization logic.",
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "WasmUnknownError",
///        "_EVMError"
///      ]
///    },
///    {
///      "title": "CompilationError",
///      "description": "Wasm compilation error",
///      "type": "object",
///      "required": [
///        "CompilationError"
///      ],
///      "properties": {
///        "CompilationError": {
///          "$ref": "#/definitions/CompilationError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "LinkError",
///      "description": "Wasm binary env link error\n\nNote: this is only to deserialize old data, use execution error for new data",
///      "type": "object",
///      "required": [
///        "LinkError"
///      ],
///      "properties": {
///        "LinkError": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "MethodResolveError",
///      "description": "Import/export resolve error",
///      "type": "object",
///      "required": [
///        "MethodResolveError"
///      ],
///      "properties": {
///        "MethodResolveError": {
///          "$ref": "#/definitions/MethodResolveError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "WasmTrap",
///      "description": "A trap happened during execution of a binary\n\nNote: this is only to deserialize old data, use execution error for new data",
///      "type": "object",
///      "required": [
///        "WasmTrap"
///      ],
///      "properties": {
///        "WasmTrap": {
///          "$ref": "#/definitions/WasmTrap"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "HostError",
///      "description": "Note: this is only to deserialize old data, use execution error for new data",
///      "type": "object",
///      "required": [
///        "HostError"
///      ],
///      "properties": {
///        "HostError": {
///          "$ref": "#/definitions/HostError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ExecutionError",
///      "type": "object",
///      "required": [
///        "ExecutionError"
///      ],
///      "properties": {
///        "ExecutionError": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum FunctionCallError {
    WasmUnknownError,
    #[serde(rename = "_EVMError")]
    EvmError,
    ///Wasm compilation error
    CompilationError(CompilationError),
    /**Wasm binary env link error

Note: this is only to deserialize old data, use execution error for new data*/
    LinkError { msg: ::std::string::String },
    ///Import/export resolve error
    MethodResolveError(MethodResolveError),
    /**A trap happened during execution of a binary

Note: this is only to deserialize old data, use execution error for new data*/
    WasmTrap(WasmTrap),
    ///Note: this is only to deserialize old data, use execution error for new data
    HostError(HostError),
    ExecutionError(::std::string::String),
}
impl ::std::convert::From<CompilationError> for FunctionCallError {
    fn from(value: CompilationError) -> Self {
        Self::CompilationError(value)
    }
}
impl ::std::convert::From<MethodResolveError> for FunctionCallError {
    fn from(value: MethodResolveError) -> Self {
        Self::MethodResolveError(value)
    }
}
impl ::std::convert::From<WasmTrap> for FunctionCallError {
    fn from(value: WasmTrap) -> Self {
        Self::WasmTrap(value)
    }
}
impl ::std::convert::From<HostError> for FunctionCallError {
    fn from(value: HostError) -> Self {
        Self::HostError(value)
    }
}
/**Grants limited permission to make transactions with FunctionCallActions
The permission can limit the allowed balance to be spent on the prepaid gas.
It also restrict the account ID of the receiver for this function call.
It also can restrict the method name for the allowed function calls.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "FunctionCallPermission",
///  "description": "Grants limited permission to make transactions with FunctionCallActions\nThe permission can limit the allowed balance to be spent on the prepaid gas.\nIt also restrict the account ID of the receiver for this function call.\nIt also can restrict the method name for the allowed function calls.",
///  "type": "object",
///  "required": [
///    "method_names",
///    "receiver_id"
///  ],
///  "properties": {
///    "allowance": {
///      "description": "Allowance is a balance limit to use by this access key to pay for function call gas and\ntransaction fees. When this access key is used, both account balance and the allowance is\ndecreased by the same value.\n`None` means unlimited allowance.\nNOTE: To change or increase the allowance, the old access key needs to be deleted and a new\naccess key should be created.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "method_names": {
///      "description": "A list of method names that can be used. The access key only allows transactions with the\nfunction call of one of the given method names.\nEmpty list means any method name can be used.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "receiver_id": {
///      "description": "The access key only allows transactions with the given receiver's account id.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FunctionCallPermission {
    /**Allowance is a balance limit to use by this access key to pay for function call gas and
transaction fees. When this access key is used, both account balance and the allowance is
decreased by the same value.
`None` means unlimited allowance.
NOTE: To change or increase the allowance, the old access key needs to be deleted and a new
access key should be created.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub allowance: ::std::option::Option<NearToken>,
    /**A list of method names that can be used. The access key only allows transactions with the
function call of one of the given method names.
Empty list means any method name can be used.*/
    pub method_names: ::std::vec::Vec<::std::string::String>,
    ///The access key only allows transactions with the given receiver's account id.
    pub receiver_id: ::std::string::String,
}
///`GasKeyInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GasKeyInfo",
///  "type": "object",
///  "required": [
///    "balance",
///    "num_nonces"
///  ],
///  "properties": {
///    "balance": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "num_nonces": {
///      "type": "integer",
///      "format": "uint16",
///      "maximum": 65535.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GasKeyInfo {
    pub balance: NearToken,
    pub num_nonces: u16,
}
///Configuration for garbage collection.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GCConfig",
///  "description": "Configuration for garbage collection.",
///  "type": "object",
///  "properties": {
///    "gc_blocks_limit": {
///      "description": "Maximum number of blocks to garbage collect at every garbage collection\ncall.",
///      "default": 2,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "gc_fork_clean_step": {
///      "description": "Maximum number of height to go through at each garbage collection step\nwhen cleaning forks during garbage collection.",
///      "default": 100,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "gc_num_epochs_to_keep": {
///      "description": "Number of epochs for which we keep store data.",
///      "default": 5,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "gc_step_period": {
///      "description": "How often gc should be run",
///      "default": {
///        "nanos": 500000000,
///        "secs": 0
///      },
///      "allOf": [
///        {
///          "$ref": "#/definitions/DurationAsStdSchemaProvider"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GcConfig {
    /**Maximum number of blocks to garbage collect at every garbage collection
call.*/
    #[serde(default = "defaults::default_u64::<u64, 2>")]
    pub gc_blocks_limit: u64,
    /**Maximum number of height to go through at each garbage collection step
when cleaning forks during garbage collection.*/
    #[serde(default = "defaults::default_u64::<u64, 100>")]
    pub gc_fork_clean_step: u64,
    ///Number of epochs for which we keep store data.
    #[serde(default = "defaults::default_u64::<u64, 5>")]
    pub gc_num_epochs_to_keep: u64,
    ///How often gc should be run
    #[serde(default = "defaults::gc_config_gc_step_period")]
    pub gc_step_period: DurationAsStdSchemaProvider,
}
impl ::std::default::Default for GcConfig {
    fn default() -> Self {
        Self {
            gc_blocks_limit: defaults::default_u64::<u64, 2>(),
            gc_fork_clean_step: defaults::default_u64::<u64, 100>(),
            gc_num_epochs_to_keep: defaults::default_u64::<u64, 5>(),
            gc_step_period: defaults::gc_config_gc_step_period(),
        }
    }
}
///`GenesisConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GenesisConfig",
///  "type": "object",
///  "required": [
///    "avg_hidden_validator_seats_per_shard",
///    "block_producer_kickout_threshold",
///    "chain_id",
///    "chunk_producer_kickout_threshold",
///    "dynamic_resharding",
///    "epoch_length",
///    "fishermen_threshold",
///    "gas_limit",
///    "gas_price_adjustment_rate",
///    "genesis_height",
///    "genesis_time",
///    "max_gas_price",
///    "max_inflation_rate",
///    "min_gas_price",
///    "num_block_producer_seats",
///    "num_block_producer_seats_per_shard",
///    "num_blocks_per_year",
///    "protocol_reward_rate",
///    "protocol_treasury_account",
///    "protocol_version",
///    "total_supply",
///    "transaction_validity_period",
///    "validators"
///  ],
///  "properties": {
///    "avg_hidden_validator_seats_per_shard": {
///      "description": "Expected number of hidden validators per shard.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "block_producer_kickout_threshold": {
///      "description": "Threshold for kicking out block producers, between 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "chain_id": {
///      "description": "ID of the blockchain. This must be unique for every blockchain.\nIf your testnet blockchains do not have unique chain IDs, you will have a bad time.",
///      "type": "string"
///    },
///    "chunk_producer_assignment_changes_limit": {
///      "description": "Limits the number of shard changes in chunk producer assignments,\nif algorithm is able to choose assignment with better balance of\nnumber of chunk producers for shards.",
///      "default": 5,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "chunk_producer_kickout_threshold": {
///      "description": "Threshold for kicking out chunk producers, between 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "chunk_validator_only_kickout_threshold": {
///      "description": "Threshold for kicking out nodes which are only chunk validators, between 0 and 100.",
///      "default": 80,
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "dynamic_resharding": {
///      "description": "Enable dynamic re-sharding.",
///      "type": "boolean"
///    },
///    "epoch_length": {
///      "description": "Epoch length counted in block heights.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "fishermen_threshold": {
///      "description": "Fishermen stake threshold.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "gas_limit": {
///      "description": "Initial gas limit.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "gas_price_adjustment_rate": {
///      "description": "Gas price adjustment rate",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "genesis_height": {
///      "description": "Height of genesis block.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "genesis_time": {
///      "description": "Official time of blockchain start.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "max_gas_price": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "max_inflation_rate": {
///      "description": "Maximum inflation on the total supply every epoch.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "max_kickout_stake_perc": {
///      "description": "Max stake percentage of the validators we will kick out.",
///      "default": 100,
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "min_gas_price": {
///      "description": "Minimum gas price. It is also the initial gas price.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "minimum_stake_divisor": {
///      "description": "The minimum stake required for staking is last seat price divided by this number.",
///      "default": 10,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "minimum_stake_ratio": {
///      "description": "The lowest ratio s/s_total any block producer can have.\nSee <https://github.com/near/NEPs/pull/167> for details",
///      "default": [
///        1,
///        6250
///      ],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "minimum_validators_per_shard": {
///      "description": "The minimum number of validators each shard must have",
///      "default": 1,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats": {
///      "description": "Number of block producer seats at genesis.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats_per_shard": {
///      "description": "Defines number of shards and number of block producer seats per each shard at genesis.\nNote: not used with protocol_feature_chunk_only_producers -- replaced by minimum_validators_per_shard\nNote: not used before as all block producers produce chunks for all shards",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_blocks_per_year": {
///      "description": "Expected number of blocks per year",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_chunk_only_producer_seats": {
///      "description": "Deprecated.",
///      "default": 300,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_chunk_producer_seats": {
///      "description": "Number of chunk producers.\nDon't mess it up with chunk-only producers feature which is deprecated.",
///      "default": 100,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_chunk_validator_seats": {
///      "default": 300,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "online_max_threshold": {
///      "description": "Online maximum threshold above which validator gets full reward.",
///      "default": [
///        99,
///        100
///      ],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "online_min_threshold": {
///      "description": "Online minimum threshold below which validator doesn't receive reward.",
///      "default": [
///        9,
///        10
///      ],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_reward_rate": {
///      "description": "Protocol treasury rate",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_treasury_account": {
///      "description": "Protocol treasury account",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        }
///      ]
///    },
///    "protocol_upgrade_stake_threshold": {
///      "description": "Threshold of stake that needs to indicate that they ready for upgrade.",
///      "default": [
///        4,
///        5
///      ],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_version": {
///      "description": "Protocol version that this genesis works with.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "shard_layout": {
///      "description": "Layout information regarding how to split accounts to shards",
///      "default": {
///        "V2": {
///          "boundary_accounts": [],
///          "id_to_index_map": {
///            "0": 0
///          },
///          "index_to_id_map": {
///            "0": 0
///          },
///          "shard_ids": [
///            0
///          ],
///          "shards_parent_map": null,
///          "shards_split_map": null,
///          "version": 0
///        }
///      },
///      "allOf": [
///        {
///          "$ref": "#/definitions/ShardLayout"
///        }
///      ]
///    },
///    "shuffle_shard_assignment_for_chunk_producers": {
///      "description": "If true, shuffle the chunk producers across shards. In other words, if\nthe shard assignments were `[S_0, S_1, S_2, S_3]` where `S_i` represents\nthe set of chunk producers for shard `i`, if this flag were true, the\nshard assignments might become, for example, `[S_2, S_0, S_3, S_1]`.",
///      "default": false,
///      "type": "boolean"
///    },
///    "target_validator_mandates_per_shard": {
///      "description": "Number of target chunk validator mandates for each shard.",
///      "default": 68,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "total_supply": {
///      "description": "Total supply of tokens at genesis.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "transaction_validity_period": {
///      "description": "Number of blocks for which a given transaction is valid",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "use_production_config": {
///      "description": "This is only for test purposes. We hard code some configs for mainnet and testnet\nin AllEpochConfig, and we want to have a way to test that code path. This flag is for that.\nIf set to true, the node will use the same config override path as mainnet and testnet.",
///      "default": false,
///      "type": "boolean"
///    },
///    "validators": {
///      "description": "List of initial validators.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AccountInfo"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GenesisConfig {
    ///Expected number of hidden validators per shard.
    pub avg_hidden_validator_seats_per_shard: ::std::vec::Vec<u64>,
    ///Threshold for kicking out block producers, between 0 and 100.
    pub block_producer_kickout_threshold: u8,
    /**ID of the blockchain. This must be unique for every blockchain.
If your testnet blockchains do not have unique chain IDs, you will have a bad time.*/
    pub chain_id: ::std::string::String,
    /**Limits the number of shard changes in chunk producer assignments,
if algorithm is able to choose assignment with better balance of
number of chunk producers for shards.*/
    #[serde(default = "defaults::default_u64::<u64, 5>")]
    pub chunk_producer_assignment_changes_limit: u64,
    ///Threshold for kicking out chunk producers, between 0 and 100.
    pub chunk_producer_kickout_threshold: u8,
    ///Threshold for kicking out nodes which are only chunk validators, between 0 and 100.
    #[serde(default = "defaults::default_u64::<u8, 80>")]
    pub chunk_validator_only_kickout_threshold: u8,
    ///Enable dynamic re-sharding.
    pub dynamic_resharding: bool,
    ///Epoch length counted in block heights.
    pub epoch_length: u64,
    ///Fishermen stake threshold.
    pub fishermen_threshold: NearToken,
    ///Initial gas limit.
    pub gas_limit: NearGas,
    ///Gas price adjustment rate
    pub gas_price_adjustment_rate: [i32; 2usize],
    ///Height of genesis block.
    pub genesis_height: u64,
    ///Official time of blockchain start.
    pub genesis_time: ::chrono::DateTime<::chrono::offset::Utc>,
    pub max_gas_price: NearToken,
    ///Maximum inflation on the total supply every epoch.
    pub max_inflation_rate: [i32; 2usize],
    ///Max stake percentage of the validators we will kick out.
    #[serde(default = "defaults::default_u64::<u8, 100>")]
    pub max_kickout_stake_perc: u8,
    ///Minimum gas price. It is also the initial gas price.
    pub min_gas_price: NearToken,
    ///The minimum stake required for staking is last seat price divided by this number.
    #[serde(default = "defaults::default_u64::<u64, 10>")]
    pub minimum_stake_divisor: u64,
    /**The lowest ratio s/s_total any block producer can have.
See <https://github.com/near/NEPs/pull/167> for details*/
    #[serde(default = "defaults::genesis_config_minimum_stake_ratio")]
    pub minimum_stake_ratio: [i32; 2usize],
    ///The minimum number of validators each shard must have
    #[serde(default = "defaults::default_u64::<u64, 1>")]
    pub minimum_validators_per_shard: u64,
    ///Number of block producer seats at genesis.
    pub num_block_producer_seats: u64,
    /**Defines number of shards and number of block producer seats per each shard at genesis.
Note: not used with protocol_feature_chunk_only_producers -- replaced by minimum_validators_per_shard
Note: not used before as all block producers produce chunks for all shards*/
    pub num_block_producer_seats_per_shard: ::std::vec::Vec<u64>,
    ///Expected number of blocks per year
    pub num_blocks_per_year: u64,
    ///Deprecated.
    #[serde(default = "defaults::default_u64::<u64, 300>")]
    pub num_chunk_only_producer_seats: u64,
    /**Number of chunk producers.
Don't mess it up with chunk-only producers feature which is deprecated.*/
    #[serde(default = "defaults::default_u64::<u64, 100>")]
    pub num_chunk_producer_seats: u64,
    #[serde(default = "defaults::default_u64::<u64, 300>")]
    pub num_chunk_validator_seats: u64,
    ///Online maximum threshold above which validator gets full reward.
    #[serde(default = "defaults::genesis_config_online_max_threshold")]
    pub online_max_threshold: [i32; 2usize],
    ///Online minimum threshold below which validator doesn't receive reward.
    #[serde(default = "defaults::genesis_config_online_min_threshold")]
    pub online_min_threshold: [i32; 2usize],
    ///Protocol treasury rate
    pub protocol_reward_rate: [i32; 2usize],
    ///Protocol treasury account
    pub protocol_treasury_account: AccountId,
    ///Threshold of stake that needs to indicate that they ready for upgrade.
    #[serde(default = "defaults::genesis_config_protocol_upgrade_stake_threshold")]
    pub protocol_upgrade_stake_threshold: [i32; 2usize],
    ///Protocol version that this genesis works with.
    pub protocol_version: u32,
    ///Layout information regarding how to split accounts to shards
    #[serde(default = "defaults::genesis_config_shard_layout")]
    pub shard_layout: ShardLayout,
    /**If true, shuffle the chunk producers across shards. In other words, if
the shard assignments were `[S_0, S_1, S_2, S_3]` where `S_i` represents
the set of chunk producers for shard `i`, if this flag were true, the
shard assignments might become, for example, `[S_2, S_0, S_3, S_1]`.*/
    #[serde(default)]
    pub shuffle_shard_assignment_for_chunk_producers: bool,
    ///Number of target chunk validator mandates for each shard.
    #[serde(default = "defaults::default_u64::<u64, 68>")]
    pub target_validator_mandates_per_shard: u64,
    ///Total supply of tokens at genesis.
    pub total_supply: NearToken,
    ///Number of blocks for which a given transaction is valid
    pub transaction_validity_period: u64,
    /**This is only for test purposes. We hard code some configs for mainnet and testnet
in AllEpochConfig, and we want to have a way to test that code path. This flag is for that.
If set to true, the node will use the same config override path as mainnet and testnet.*/
    #[serde(default)]
    pub use_production_config: bool,
    ///List of initial validators.
    pub validators: ::std::vec::Vec<AccountInfo>,
}
///`GenesisConfigRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GenesisConfigRequest",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GenesisConfigRequest(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for GenesisConfigRequest {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<GenesisConfigRequest>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: GenesisConfigRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for GenesisConfigRequest {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`GlobalContractDeployMode`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GlobalContractDeployMode",
///  "oneOf": [
///    {
///      "description": "Contract is deployed under its code hash.\nUsers will be able reference it by that hash.\nThis effectively makes the contract immutable.",
///      "type": "string",
///      "const": "CodeHash"
///    },
///    {
///      "description": "Contract is deployed under the owner account id.\nUsers will be able reference it by that account id.\nThis allows the owner to update the contract for all its users.",
///      "type": "string",
///      "const": "AccountId"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum GlobalContractDeployMode {
    /**Contract is deployed under its code hash.
Users will be able reference it by that hash.
This effectively makes the contract immutable.*/
    CodeHash,
    /**Contract is deployed under the owner account id.
Users will be able reference it by that account id.
This allows the owner to update the contract for all its users.*/
    AccountId,
}
impl ::std::fmt::Display for GlobalContractDeployMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeHash => f.write_str("CodeHash"),
            Self::AccountId => f.write_str("AccountId"),
        }
    }
}
impl ::std::str::FromStr for GlobalContractDeployMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CodeHash" => Ok(Self::CodeHash),
            "AccountId" => Ok(Self::AccountId),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GlobalContractDeployMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GlobalContractDeployMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GlobalContractDeployMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`GlobalContractIdentifier`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GlobalContractIdentifier",
///  "oneOf": [
///    {
///      "title": "CodeHash",
///      "type": "object",
///      "required": [
///        "CodeHash"
///      ],
///      "properties": {
///        "CodeHash": {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AccountId",
///      "type": "object",
///      "required": [
///        "AccountId"
///      ],
///      "properties": {
///        "AccountId": {
///          "$ref": "#/definitions/AccountId"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum GlobalContractIdentifier {
    CodeHash(CryptoHash),
    AccountId(AccountId),
}
impl ::std::convert::From<CryptoHash> for GlobalContractIdentifier {
    fn from(value: CryptoHash) -> Self {
        Self::CodeHash(value)
    }
}
impl ::std::convert::From<AccountId> for GlobalContractIdentifier {
    fn from(value: AccountId) -> Self {
        Self::AccountId(value)
    }
}
///`GlobalContractIdentifierView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GlobalContractIdentifierView",
///  "oneOf": [
///    {
///      "title": "Hash",
///      "type": "object",
///      "required": [
///        "hash"
///      ],
///      "properties": {
///        "hash": {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AccountId",
///      "type": "object",
///      "required": [
///        "account_id"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum GlobalContractIdentifierView {
    #[serde(rename = "hash")]
    Hash(CryptoHash),
    #[serde(rename = "account_id")]
    AccountId(AccountId),
}
impl ::std::convert::From<CryptoHash> for GlobalContractIdentifierView {
    fn from(value: CryptoHash) -> Self {
        Self::Hash(value)
    }
}
impl ::std::convert::From<AccountId> for GlobalContractIdentifierView {
    fn from(value: AccountId) -> Self {
        Self::AccountId(value)
    }
}
///`HostError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "HostError",
///  "oneOf": [
///    {
///      "description": "String encoding is bad UTF-16 sequence",
///      "type": "string",
///      "const": "BadUTF16"
///    },
///    {
///      "description": "String encoding is bad UTF-8 sequence",
///      "type": "string",
///      "const": "BadUTF8"
///    },
///    {
///      "description": "Exceeded the prepaid gas",
///      "type": "string",
///      "const": "GasExceeded"
///    },
///    {
///      "description": "Exceeded the maximum amount of gas allowed to burn per contract",
///      "type": "string",
///      "const": "GasLimitExceeded"
///    },
///    {
///      "description": "Exceeded the account balance",
///      "type": "string",
///      "const": "BalanceExceeded"
///    },
///    {
///      "description": "Tried to call an empty method name",
///      "type": "string",
///      "const": "EmptyMethodName"
///    },
///    {
///      "title": "GuestPanic",
///      "description": "Smart contract panicked",
///      "type": "object",
///      "required": [
///        "GuestPanic"
///      ],
///      "properties": {
///        "GuestPanic": {
///          "type": "object",
///          "required": [
///            "panic_msg"
///          ],
///          "properties": {
///            "panic_msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "IntegerOverflow happened during a contract execution",
///      "type": "string",
///      "const": "IntegerOverflow"
///    },
///    {
///      "title": "InvalidPromiseIndex",
///      "description": "`promise_idx` does not correspond to existing promises",
///      "type": "object",
///      "required": [
///        "InvalidPromiseIndex"
///      ],
///      "properties": {
///        "InvalidPromiseIndex": {
///          "type": "object",
///          "required": [
///            "promise_idx"
///          ],
///          "properties": {
///            "promise_idx": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Actions can only be appended to non-joint promise.",
///      "type": "string",
///      "const": "CannotAppendActionToJointPromise"
///    },
///    {
///      "description": "Returning joint promise is currently prohibited",
///      "type": "string",
///      "const": "CannotReturnJointPromise"
///    },
///    {
///      "title": "InvalidPromiseResultIndex",
///      "description": "Accessed invalid promise result index",
///      "type": "object",
///      "required": [
///        "InvalidPromiseResultIndex"
///      ],
///      "properties": {
///        "InvalidPromiseResultIndex": {
///          "type": "object",
///          "required": [
///            "result_idx"
///          ],
///          "properties": {
///            "result_idx": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidRegisterId",
///      "description": "Accessed invalid register id",
///      "type": "object",
///      "required": [
///        "InvalidRegisterId"
///      ],
///      "properties": {
///        "InvalidRegisterId": {
///          "type": "object",
///          "required": [
///            "register_id"
///          ],
///          "properties": {
///            "register_id": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "IteratorWasInvalidated",
///      "description": "Iterator `iterator_index` was invalidated after its creation by performing a mutable operation on trie",
///      "type": "object",
///      "required": [
///        "IteratorWasInvalidated"
///      ],
///      "properties": {
///        "IteratorWasInvalidated": {
///          "type": "object",
///          "required": [
///            "iterator_index"
///          ],
///          "properties": {
///            "iterator_index": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Accessed memory outside the bounds",
///      "type": "string",
///      "const": "MemoryAccessViolation"
///    },
///    {
///      "title": "InvalidReceiptIndex",
///      "description": "VM Logic returned an invalid receipt index",
///      "type": "object",
///      "required": [
///        "InvalidReceiptIndex"
///      ],
///      "properties": {
///        "InvalidReceiptIndex": {
///          "type": "object",
///          "required": [
///            "receipt_index"
///          ],
///          "properties": {
///            "receipt_index": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidIteratorIndex",
///      "description": "Iterator index `iterator_index` does not exist",
///      "type": "object",
///      "required": [
///        "InvalidIteratorIndex"
///      ],
///      "properties": {
///        "InvalidIteratorIndex": {
///          "type": "object",
///          "required": [
///            "iterator_index"
///          ],
///          "properties": {
///            "iterator_index": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "VM Logic returned an invalid account id",
///      "type": "string",
///      "const": "InvalidAccountId"
///    },
///    {
///      "description": "VM Logic returned an invalid method name",
///      "type": "string",
///      "const": "InvalidMethodName"
///    },
///    {
///      "description": "VM Logic provided an invalid public key",
///      "type": "string",
///      "const": "InvalidPublicKey"
///    },
///    {
///      "title": "ProhibitedInView",
///      "description": "`method_name` is not allowed in view calls",
///      "type": "object",
///      "required": [
///        "ProhibitedInView"
///      ],
///      "properties": {
///        "ProhibitedInView": {
///          "type": "object",
///          "required": [
///            "method_name"
///          ],
///          "properties": {
///            "method_name": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "NumberOfLogsExceeded",
///      "description": "The total number of logs will exceed the limit.",
///      "type": "object",
///      "required": [
///        "NumberOfLogsExceeded"
///      ],
///      "properties": {
///        "NumberOfLogsExceeded": {
///          "type": "object",
///          "required": [
///            "limit"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "KeyLengthExceeded",
///      "description": "The storage key length exceeded the limit.",
///      "type": "object",
///      "required": [
///        "KeyLengthExceeded"
///      ],
///      "properties": {
///        "KeyLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ValueLengthExceeded",
///      "description": "The storage value length exceeded the limit.",
///      "type": "object",
///      "required": [
///        "ValueLengthExceeded"
///      ],
///      "properties": {
///        "ValueLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "TotalLogLengthExceeded",
///      "description": "The total log length exceeded the limit.",
///      "type": "object",
///      "required": [
///        "TotalLogLengthExceeded"
///      ],
///      "properties": {
///        "TotalLogLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "NumberPromisesExceeded",
///      "description": "The maximum number of promises within a FunctionCall exceeded the limit.",
///      "type": "object",
///      "required": [
///        "NumberPromisesExceeded"
///      ],
///      "properties": {
///        "NumberPromisesExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "number_of_promises"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "number_of_promises": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "NumberInputDataDependenciesExceeded",
///      "description": "The maximum number of input data dependencies exceeded the limit.",
///      "type": "object",
///      "required": [
///        "NumberInputDataDependenciesExceeded"
///      ],
///      "properties": {
///        "NumberInputDataDependenciesExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "number_of_input_data_dependencies"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "number_of_input_data_dependencies": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ReturnedValueLengthExceeded",
///      "description": "The returned value length exceeded the limit.",
///      "type": "object",
///      "required": [
///        "ReturnedValueLengthExceeded"
///      ],
///      "properties": {
///        "ReturnedValueLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ContractSizeExceeded",
///      "description": "The contract size for DeployContract action exceeded the limit.",
///      "type": "object",
///      "required": [
///        "ContractSizeExceeded"
///      ],
///      "properties": {
///        "ContractSizeExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "size"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "size": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Deprecated",
///      "description": "The host function was deprecated.",
///      "type": "object",
///      "required": [
///        "Deprecated"
///      ],
///      "properties": {
///        "Deprecated": {
///          "type": "object",
///          "required": [
///            "method_name"
///          ],
///          "properties": {
///            "method_name": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ECRecoverError",
///      "description": "General errors for ECDSA recover.",
///      "type": "object",
///      "required": [
///        "ECRecoverError"
///      ],
///      "properties": {
///        "ECRecoverError": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AltBn128InvalidInput",
///      "description": "Invalid input to alt_bn128 family of functions (e.g., point which isn't\non the curve).",
///      "type": "object",
///      "required": [
///        "AltBn128InvalidInput"
///      ],
///      "properties": {
///        "AltBn128InvalidInput": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Ed25519VerifyInvalidInput",
///      "description": "Invalid input to ed25519 signature verification function (e.g. signature cannot be\nderived from bytes).",
///      "type": "object",
///      "required": [
///        "Ed25519VerifyInvalidInput"
///      ],
///      "properties": {
///        "Ed25519VerifyInvalidInput": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum HostError {
    ///String encoding is bad UTF-16 sequence
    #[serde(rename = "BadUTF16")]
    BadUtf16,
    ///String encoding is bad UTF-8 sequence
    #[serde(rename = "BadUTF8")]
    BadUtf8,
    ///Exceeded the prepaid gas
    GasExceeded,
    ///Exceeded the maximum amount of gas allowed to burn per contract
    GasLimitExceeded,
    ///Exceeded the account balance
    BalanceExceeded,
    ///Tried to call an empty method name
    EmptyMethodName,
    ///Smart contract panicked
    GuestPanic { panic_msg: ::std::string::String },
    ///IntegerOverflow happened during a contract execution
    IntegerOverflow,
    ///`promise_idx` does not correspond to existing promises
    InvalidPromiseIndex { promise_idx: u64 },
    ///Actions can only be appended to non-joint promise.
    CannotAppendActionToJointPromise,
    ///Returning joint promise is currently prohibited
    CannotReturnJointPromise,
    ///Accessed invalid promise result index
    InvalidPromiseResultIndex { result_idx: u64 },
    ///Accessed invalid register id
    InvalidRegisterId { register_id: u64 },
    ///Iterator `iterator_index` was invalidated after its creation by performing a mutable operation on trie
    IteratorWasInvalidated { iterator_index: u64 },
    ///Accessed memory outside the bounds
    MemoryAccessViolation,
    ///VM Logic returned an invalid receipt index
    InvalidReceiptIndex { receipt_index: u64 },
    ///Iterator index `iterator_index` does not exist
    InvalidIteratorIndex { iterator_index: u64 },
    ///VM Logic returned an invalid account id
    InvalidAccountId,
    ///VM Logic returned an invalid method name
    InvalidMethodName,
    ///VM Logic provided an invalid public key
    InvalidPublicKey,
    ///`method_name` is not allowed in view calls
    ProhibitedInView { method_name: ::std::string::String },
    ///The total number of logs will exceed the limit.
    NumberOfLogsExceeded { limit: u64 },
    ///The storage key length exceeded the limit.
    KeyLengthExceeded { length: u64, limit: u64 },
    ///The storage value length exceeded the limit.
    ValueLengthExceeded { length: u64, limit: u64 },
    ///The total log length exceeded the limit.
    TotalLogLengthExceeded { length: u64, limit: u64 },
    ///The maximum number of promises within a FunctionCall exceeded the limit.
    NumberPromisesExceeded { limit: u64, number_of_promises: u64 },
    ///The maximum number of input data dependencies exceeded the limit.
    NumberInputDataDependenciesExceeded {
        limit: u64,
        number_of_input_data_dependencies: u64,
    },
    ///The returned value length exceeded the limit.
    ReturnedValueLengthExceeded { length: u64, limit: u64 },
    ///The contract size for DeployContract action exceeded the limit.
    ContractSizeExceeded { limit: u64, size: u64 },
    ///The host function was deprecated.
    Deprecated { method_name: ::std::string::String },
    ///General errors for ECDSA recover.
    #[serde(rename = "ECRecoverError")]
    EcRecoverError { msg: ::std::string::String },
    /**Invalid input to alt_bn128 family of functions (e.g., point which isn't
on the curve).*/
    AltBn128InvalidInput { msg: ::std::string::String },
    /**Invalid input to ed25519 signature verification function (e.g. signature cannot be
derived from bytes).*/
    Ed25519VerifyInvalidInput { msg: ::std::string::String },
}
///`InvalidAccessKeyError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "InvalidAccessKeyError",
///  "oneOf": [
///    {
///      "title": "AccessKeyNotFound",
///      "description": "The access key identified by the `public_key` doesn't exist for the account",
///      "type": "object",
///      "required": [
///        "AccessKeyNotFound"
///      ],
///      "properties": {
///        "AccessKeyNotFound": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ReceiverMismatch",
///      "description": "Transaction `receiver_id` doesn't match the access key receiver_id",
///      "type": "object",
///      "required": [
///        "ReceiverMismatch"
///      ],
///      "properties": {
///        "ReceiverMismatch": {
///          "type": "object",
///          "required": [
///            "ak_receiver",
///            "tx_receiver"
///          ],
///          "properties": {
///            "ak_receiver": {
///              "type": "string"
///            },
///            "tx_receiver": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "MethodNameMismatch",
///      "description": "Transaction method name isn't allowed by the access key",
///      "type": "object",
///      "required": [
///        "MethodNameMismatch"
///      ],
///      "properties": {
///        "MethodNameMismatch": {
///          "type": "object",
///          "required": [
///            "method_name"
///          ],
///          "properties": {
///            "method_name": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Transaction requires a full permission access key.",
///      "type": "string",
///      "const": "RequiresFullAccess"
///    },
///    {
///      "title": "NotEnoughAllowance",
///      "description": "Access Key does not have enough allowance to cover transaction cost",
///      "type": "object",
///      "required": [
///        "NotEnoughAllowance"
///      ],
///      "properties": {
///        "NotEnoughAllowance": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "allowance",
///            "cost",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "allowance": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "cost": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Having a deposit with a function call action is not allowed with a function call access key.",
///      "type": "string",
///      "const": "DepositWithFunctionCall"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum InvalidAccessKeyError {
    ///The access key identified by the `public_key` doesn't exist for the account
    AccessKeyNotFound { account_id: AccountId, public_key: PublicKey },
    ///Transaction `receiver_id` doesn't match the access key receiver_id
    ReceiverMismatch { ak_receiver: ::std::string::String, tx_receiver: AccountId },
    ///Transaction method name isn't allowed by the access key
    MethodNameMismatch { method_name: ::std::string::String },
    ///Transaction requires a full permission access key.
    RequiresFullAccess,
    ///Access Key does not have enough allowance to cover transaction cost
    NotEnoughAllowance {
        account_id: AccountId,
        allowance: NearToken,
        cost: NearToken,
        public_key: PublicKey,
    },
    ///Having a deposit with a function call action is not allowed with a function call access key.
    DepositWithFunctionCall,
}
///An error happened during TX execution
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "InvalidTxError",
///  "description": "An error happened during TX execution",
///  "oneOf": [
///    {
///      "title": "InvalidAccessKeyError",
///      "description": "Happens if a wrong AccessKey used or AccessKey has not enough permissions",
///      "type": "object",
///      "required": [
///        "InvalidAccessKeyError"
///      ],
///      "properties": {
///        "InvalidAccessKeyError": {
///          "$ref": "#/definitions/InvalidAccessKeyError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidSignerId",
///      "description": "TX signer_id is not a valid [`AccountId`]",
///      "type": "object",
///      "required": [
///        "InvalidSignerId"
///      ],
///      "properties": {
///        "InvalidSignerId": {
///          "type": "object",
///          "required": [
///            "signer_id"
///          ],
///          "properties": {
///            "signer_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "SignerDoesNotExist",
///      "description": "TX signer_id is not found in a storage",
///      "type": "object",
///      "required": [
///        "SignerDoesNotExist"
///      ],
///      "properties": {
///        "SignerDoesNotExist": {
///          "type": "object",
///          "required": [
///            "signer_id"
///          ],
///          "properties": {
///            "signer_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidNonce",
///      "description": "Transaction nonce must be strictly greater than `account[access_key].nonce`.",
///      "type": "object",
///      "required": [
///        "InvalidNonce"
///      ],
///      "properties": {
///        "InvalidNonce": {
///          "type": "object",
///          "required": [
///            "ak_nonce",
///            "tx_nonce"
///          ],
///          "properties": {
///            "ak_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "tx_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "NonceTooLarge",
///      "description": "Transaction nonce is larger than the upper bound given by the block height",
///      "type": "object",
///      "required": [
///        "NonceTooLarge"
///      ],
///      "properties": {
///        "NonceTooLarge": {
///          "type": "object",
///          "required": [
///            "tx_nonce",
///            "upper_bound"
///          ],
///          "properties": {
///            "tx_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "upper_bound": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidReceiverId",
///      "description": "TX receiver_id is not a valid AccountId",
///      "type": "object",
///      "required": [
///        "InvalidReceiverId"
///      ],
///      "properties": {
///        "InvalidReceiverId": {
///          "type": "object",
///          "required": [
///            "receiver_id"
///          ],
///          "properties": {
///            "receiver_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "TX signature is not valid",
///      "type": "string",
///      "const": "InvalidSignature"
///    },
///    {
///      "title": "NotEnoughBalance",
///      "description": "Account does not have enough balance to cover TX cost",
///      "type": "object",
///      "required": [
///        "NotEnoughBalance"
///      ],
///      "properties": {
///        "NotEnoughBalance": {
///          "type": "object",
///          "required": [
///            "balance",
///            "cost",
///            "signer_id"
///          ],
///          "properties": {
///            "balance": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "cost": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "signer_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "LackBalanceForState",
///      "description": "Signer account doesn't have enough balance after transaction.",
///      "type": "object",
///      "required": [
///        "LackBalanceForState"
///      ],
///      "properties": {
///        "LackBalanceForState": {
///          "type": "object",
///          "required": [
///            "amount",
///            "signer_id"
///          ],
///          "properties": {
///            "amount": {
///              "description": "Required balance to cover the state.",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/NearToken"
///                }
///              ]
///            },
///            "signer_id": {
///              "description": "An account which doesn't have enough balance to cover storage.",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/AccountId"
///                }
///              ]
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "An integer overflow occurred during transaction cost estimation.",
///      "type": "string",
///      "const": "CostOverflow"
///    },
///    {
///      "description": "Transaction parent block hash doesn't belong to the current chain",
///      "type": "string",
///      "const": "InvalidChain"
///    },
///    {
///      "description": "Transaction has expired",
///      "type": "string",
///      "const": "Expired"
///    },
///    {
///      "title": "ActionsValidation",
///      "description": "An error occurred while validating actions of a Transaction.",
///      "type": "object",
///      "required": [
///        "ActionsValidation"
///      ],
///      "properties": {
///        "ActionsValidation": {
///          "$ref": "#/definitions/ActionsValidationError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "TransactionSizeExceeded",
///      "description": "The size of serialized transaction exceeded the limit.",
///      "type": "object",
///      "required": [
///        "TransactionSizeExceeded"
///      ],
///      "properties": {
///        "TransactionSizeExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "size"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "size": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Transaction version is invalid.",
///      "type": "string",
///      "const": "InvalidTransactionVersion"
///    },
///    {
///      "title": "StorageError",
///      "type": "object",
///      "required": [
///        "StorageError"
///      ],
///      "properties": {
///        "StorageError": {
///          "$ref": "#/definitions/StorageError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ShardCongested",
///      "description": "The receiver shard of the transaction is too congested to accept new\ntransactions at the moment.",
///      "type": "object",
///      "required": [
///        "ShardCongested"
///      ],
///      "properties": {
///        "ShardCongested": {
///          "type": "object",
///          "required": [
///            "congestion_level",
///            "shard_id"
///          ],
///          "properties": {
///            "congestion_level": {
///              "description": "A value between 0 (no congestion) and 1 (max congestion).",
///              "type": "number",
///              "format": "double"
///            },
///            "shard_id": {
///              "description": "The congested shard.",
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ShardStuck",
///      "description": "The receiver shard of the transaction missed several chunks and rejects\nnew transaction until it can make progress again.",
///      "type": "object",
///      "required": [
///        "ShardStuck"
///      ],
///      "properties": {
///        "ShardStuck": {
///          "type": "object",
///          "required": [
///            "missed_chunks",
///            "shard_id"
///          ],
///          "properties": {
///            "missed_chunks": {
///              "description": "The number of blocks since the last included chunk of the shard.",
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "shard_id": {
///              "description": "The shard that fails making progress.",
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidNonceIndex",
///      "description": "Transaction is specifying an invalid nonce index. Gas key transactions\nmust have a nonce_index in valid range, regular transactions must not.",
///      "type": "object",
///      "required": [
///        "InvalidNonceIndex"
///      ],
///      "properties": {
///        "InvalidNonceIndex": {
///          "type": "object",
///          "required": [
///            "num_nonces"
///          ],
///          "properties": {
///            "num_nonces": {
///              "description": "Number of nonces supported by the key. 0 means no nonce_index allowed (regular key).",
///              "type": "integer",
///              "format": "uint16",
///              "maximum": 65535.0,
///              "minimum": 0.0
///            },
///            "tx_nonce_index": {
///              "description": "The nonce_index from the transaction (None if missing).",
///              "type": [
///                "integer",
///                "null"
///              ],
///              "format": "uint16",
///              "maximum": 65535.0,
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "NotEnoughGasKeyBalance",
///      "description": "Gas key does not have enough balance to cover gas costs.",
///      "type": "object",
///      "required": [
///        "NotEnoughGasKeyBalance"
///      ],
///      "properties": {
///        "NotEnoughGasKeyBalance": {
///          "type": "object",
///          "required": [
///            "balance",
///            "cost",
///            "signer_id"
///          ],
///          "properties": {
///            "balance": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "cost": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "signer_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum InvalidTxError {
    ///Happens if a wrong AccessKey used or AccessKey has not enough permissions
    InvalidAccessKeyError(InvalidAccessKeyError),
    ///TX signer_id is not a valid [`AccountId`]
    InvalidSignerId { signer_id: ::std::string::String },
    ///TX signer_id is not found in a storage
    SignerDoesNotExist { signer_id: AccountId },
    ///Transaction nonce must be strictly greater than `account[access_key].nonce`.
    InvalidNonce { ak_nonce: u64, tx_nonce: u64 },
    ///Transaction nonce is larger than the upper bound given by the block height
    NonceTooLarge { tx_nonce: u64, upper_bound: u64 },
    ///TX receiver_id is not a valid AccountId
    InvalidReceiverId { receiver_id: ::std::string::String },
    ///TX signature is not valid
    InvalidSignature,
    ///Account does not have enough balance to cover TX cost
    NotEnoughBalance { balance: NearToken, cost: NearToken, signer_id: AccountId },
    ///Signer account doesn't have enough balance after transaction.
    LackBalanceForState {
        ///Required balance to cover the state.
        amount: NearToken,
        ///An account which doesn't have enough balance to cover storage.
        signer_id: AccountId,
    },
    ///An integer overflow occurred during transaction cost estimation.
    CostOverflow,
    ///Transaction parent block hash doesn't belong to the current chain
    InvalidChain,
    ///Transaction has expired
    Expired,
    ///An error occurred while validating actions of a Transaction.
    ActionsValidation(ActionsValidationError),
    ///The size of serialized transaction exceeded the limit.
    TransactionSizeExceeded { limit: u64, size: u64 },
    ///Transaction version is invalid.
    InvalidTransactionVersion,
    StorageError(StorageError),
    /**The receiver shard of the transaction is too congested to accept new
transactions at the moment.*/
    ShardCongested {
        congestion_level: f64,
        ///The congested shard.
        shard_id: u32,
    },
    /**The receiver shard of the transaction missed several chunks and rejects
new transaction until it can make progress again.*/
    ShardStuck {
        ///The number of blocks since the last included chunk of the shard.
        missed_chunks: u64,
        ///The shard that fails making progress.
        shard_id: u32,
    },
    /**Transaction is specifying an invalid nonce index. Gas key transactions
must have a nonce_index in valid range, regular transactions must not.*/
    InvalidNonceIndex {
        ///Number of nonces supported by the key. 0 means no nonce_index allowed (regular key).
        num_nonces: u16,
        ///The nonce_index from the transaction (None if missing).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        tx_nonce_index: ::std::option::Option<u16>,
    },
    ///Gas key does not have enough balance to cover gas costs.
    NotEnoughGasKeyBalance { balance: NearToken, cost: NearToken, signer_id: AccountId },
}
impl ::std::convert::From<InvalidAccessKeyError> for InvalidTxError {
    fn from(value: InvalidAccessKeyError) -> Self {
        Self::InvalidAccessKeyError(value)
    }
}
impl ::std::convert::From<ActionsValidationError> for InvalidTxError {
    fn from(value: ActionsValidationError) -> Self {
        Self::ActionsValidation(value)
    }
}
impl ::std::convert::From<StorageError> for InvalidTxError {
    fn from(value: StorageError) -> Self {
        Self::StorageError(value)
    }
}
/**Information about a Producer: its account name, peer_id and a list of connected peers that
the node can use to send message for this producer.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "KnownProducerView",
///  "description": "Information about a Producer: its account name, peer_id and a list of connected peers that\nthe node can use to send message for this producer.",
///  "type": "object",
///  "required": [
///    "account_id",
///    "peer_id"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "next_hops": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/definitions/PublicKey"
///      }
///    },
///    "peer_id": {
///      "$ref": "#/definitions/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct KnownProducerView {
    pub account_id: AccountId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next_hops: ::std::option::Option<::std::vec::Vec<PublicKey>>,
    pub peer_id: PublicKey,
}
///`LightClientBlockLiteView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "LightClientBlockLiteView",
///  "type": "object",
///  "required": [
///    "inner_lite",
///    "inner_rest_hash",
///    "prev_block_hash"
///  ],
///  "properties": {
///    "inner_lite": {
///      "$ref": "#/definitions/BlockHeaderInnerLiteView"
///    },
///    "inner_rest_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "prev_block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LightClientBlockLiteView {
    pub inner_lite: BlockHeaderInnerLiteView,
    pub inner_rest_hash: CryptoHash,
    pub prev_block_hash: CryptoHash,
}
/**Describes limits for VM and Runtime.
TODO #4139: consider switching to strongly-typed wrappers instead of raw quantities*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "LimitConfig",
///  "description": "Describes limits for VM and Runtime.\nTODO #4139: consider switching to strongly-typed wrappers instead of raw quantities",
///  "type": "object",
///  "required": [
///    "initial_memory_pages",
///    "max_actions_per_receipt",
///    "max_arguments_length",
///    "max_contract_size",
///    "max_gas_burnt",
///    "max_length_method_name",
///    "max_length_returned_data",
///    "max_length_storage_key",
///    "max_length_storage_value",
///    "max_memory_pages",
///    "max_number_bytes_method_names",
///    "max_number_input_data_dependencies",
///    "max_number_logs",
///    "max_number_registers",
///    "max_promises_per_function_call_action",
///    "max_receipt_size",
///    "max_register_size",
///    "max_stack_height",
///    "max_total_log_length",
///    "max_total_prepaid_gas",
///    "max_transaction_size",
///    "max_yield_payload_size",
///    "per_receipt_storage_proof_size_limit",
///    "registers_memory_limit",
///    "yield_timeout_length_in_blocks"
///  ],
///  "properties": {
///    "account_id_validity_rules_version": {
///      "description": "Whether to enforce account_id well-formed-ness where it wasn't enforced\nhistorically.",
///      "default": 0,
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountIdValidityRulesVersion"
///        }
///      ]
///    },
///    "initial_memory_pages": {
///      "description": "The initial number of memory pages.\nNOTE: It's not a limiter itself, but it's a value we use for initial_memory_pages.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "max_actions_per_receipt": {
///      "description": "Max number of actions per receipt.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_arguments_length": {
///      "description": "Max length of arguments in a function call action.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_contract_size": {
///      "description": "Max contract size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_elements_per_contract_table": {
///      "description": "If present, stores max number of elements in a single contract's table",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "max_functions_number_per_contract": {
///      "description": "If present, stores max number of functions in one contract",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_gas_burnt": {
///      "description": "Max amount of gas that can be used, excluding gas attached to promises.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "max_length_method_name": {
///      "description": "Max length of any method name (without terminating character).",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_length_returned_data": {
///      "description": "Max length of returned data",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_length_storage_key": {
///      "description": "Max storage key size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_length_storage_value": {
///      "description": "Max storage value size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_locals_per_contract": {
///      "description": "If present, stores max number of locals declared globally in one contract",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_memory_pages": {
///      "description": "What is the maximal memory pages amount is allowed to have for a contract.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "max_number_bytes_method_names": {
///      "description": "Max total length of all method names (including terminating character) for a function call\npermission access key.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_number_input_data_dependencies": {
///      "description": "Max number of input data dependencies",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_number_logs": {
///      "description": "Maximum number of log entries.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_number_registers": {
///      "description": "Maximum number of registers that can be used simultaneously.\n\nNote that due to an implementation quirk [read: a bug] in VMLogic, if we\nhave this number of registers, no subsequent writes to the registers\nwill succeed even if they replace an existing register.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_promises_per_function_call_action": {
///      "description": "Max number of promises that a function call can create",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_receipt_size": {
///      "description": "Max receipt size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_register_size": {
///      "description": "Maximum number of bytes that can be stored in a single register.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_stack_height": {
///      "description": "How tall the stack is allowed to grow?\n\nSee <https://wiki.parity.io/WebAssembly-StackHeight> to find out how the stack frame cost\nis calculated.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "max_tables_per_contract": {
///      "description": "If present, stores max number of tables declared globally in one contract",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "max_total_log_length": {
///      "description": "Maximum total length in bytes of all log messages.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_total_prepaid_gas": {
///      "description": "Max total prepaid gas for all function call actions per receipt.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "max_transaction_size": {
///      "description": "Max transaction size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_yield_payload_size": {
///      "description": "Maximum number of bytes for payload passed over a yield resume.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "per_receipt_storage_proof_size_limit": {
///      "description": "Hard limit on the size of storage proof generated while executing a single receipt.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "registers_memory_limit": {
///      "description": "Limit of memory used by registers.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "yield_timeout_length_in_blocks": {
///      "description": "Number of blocks after which a yielded promise times out.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LimitConfig {
    /**Whether to enforce account_id well-formed-ness where it wasn't enforced
historically.*/
    #[serde(default = "defaults::limit_config_account_id_validity_rules_version")]
    pub account_id_validity_rules_version: AccountIdValidityRulesVersion,
    /**The initial number of memory pages.
NOTE: It's not a limiter itself, but it's a value we use for initial_memory_pages.*/
    pub initial_memory_pages: u32,
    ///Max number of actions per receipt.
    pub max_actions_per_receipt: u64,
    ///Max length of arguments in a function call action.
    pub max_arguments_length: u64,
    ///Max contract size
    pub max_contract_size: u64,
    ///If present, stores max number of elements in a single contract's table
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_elements_per_contract_table: ::std::option::Option<u32>,
    ///If present, stores max number of functions in one contract
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_functions_number_per_contract: ::std::option::Option<u64>,
    ///Max amount of gas that can be used, excluding gas attached to promises.
    pub max_gas_burnt: NearGas,
    ///Max length of any method name (without terminating character).
    pub max_length_method_name: u64,
    ///Max length of returned data
    pub max_length_returned_data: u64,
    ///Max storage key size
    pub max_length_storage_key: u64,
    ///Max storage value size
    pub max_length_storage_value: u64,
    ///If present, stores max number of locals declared globally in one contract
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_locals_per_contract: ::std::option::Option<u64>,
    ///What is the maximal memory pages amount is allowed to have for a contract.
    pub max_memory_pages: u32,
    /**Max total length of all method names (including terminating character) for a function call
permission access key.*/
    pub max_number_bytes_method_names: u64,
    ///Max number of input data dependencies
    pub max_number_input_data_dependencies: u64,
    ///Maximum number of log entries.
    pub max_number_logs: u64,
    /**Maximum number of registers that can be used simultaneously.

Note that due to an implementation quirk [read: a bug] in VMLogic, if we
have this number of registers, no subsequent writes to the registers
will succeed even if they replace an existing register.*/
    pub max_number_registers: u64,
    ///Max number of promises that a function call can create
    pub max_promises_per_function_call_action: u64,
    ///Max receipt size
    pub max_receipt_size: u64,
    ///Maximum number of bytes that can be stored in a single register.
    pub max_register_size: u64,
    /**How tall the stack is allowed to grow?

See <https://wiki.parity.io/WebAssembly-StackHeight> to find out how the stack frame cost
is calculated.*/
    pub max_stack_height: u32,
    ///If present, stores max number of tables declared globally in one contract
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tables_per_contract: ::std::option::Option<u32>,
    ///Maximum total length in bytes of all log messages.
    pub max_total_log_length: u64,
    ///Max total prepaid gas for all function call actions per receipt.
    pub max_total_prepaid_gas: NearGas,
    ///Max transaction size
    pub max_transaction_size: u64,
    ///Maximum number of bytes for payload passed over a yield resume.
    pub max_yield_payload_size: u64,
    ///Hard limit on the size of storage proof generated while executing a single receipt.
    pub per_receipt_storage_proof_size_limit: u32,
    ///Limit of memory used by registers.
    pub registers_memory_limit: u64,
    ///Number of blocks after which a yielded promise times out.
    pub yield_timeout_length_in_blocks: u64,
}
///`LogSummaryStyle`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "LogSummaryStyle",
///  "type": "string",
///  "enum": [
///    "plain",
///    "colored"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum LogSummaryStyle {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "colored")]
    Colored,
}
impl ::std::fmt::Display for LogSummaryStyle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Plain => f.write_str("plain"),
            Self::Colored => f.write_str("colored"),
        }
    }
}
impl ::std::str::FromStr for LogSummaryStyle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "plain" => Ok(Self::Plain),
            "colored" => Ok(Self::Colored),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LogSummaryStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LogSummaryStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LogSummaryStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MerklePathItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "MerklePathItem",
///  "type": "object",
///  "required": [
///    "direction",
///    "hash"
///  ],
///  "properties": {
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "hash": {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MerklePathItem {
    pub direction: Direction,
    pub hash: CryptoHash,
}
///`MethodResolveError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "MethodResolveError",
///  "type": "string",
///  "enum": [
///    "MethodEmptyName",
///    "MethodNotFound",
///    "MethodInvalidSignature"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum MethodResolveError {
    MethodEmptyName,
    MethodNotFound,
    MethodInvalidSignature,
}
impl ::std::fmt::Display for MethodResolveError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MethodEmptyName => f.write_str("MethodEmptyName"),
            Self::MethodNotFound => f.write_str("MethodNotFound"),
            Self::MethodInvalidSignature => f.write_str("MethodInvalidSignature"),
        }
    }
}
impl ::std::str::FromStr for MethodResolveError {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MethodEmptyName" => Ok(Self::MethodEmptyName),
            "MethodNotFound" => Ok(Self::MethodNotFound),
            "MethodInvalidSignature" => Ok(Self::MethodInvalidSignature),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MethodResolveError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MethodResolveError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MethodResolveError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MissingTrieValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "MissingTrieValue",
///  "type": "object",
///  "required": [
///    "context",
///    "hash"
///  ],
///  "properties": {
///    "context": {
///      "$ref": "#/definitions/MissingTrieValueContext"
///    },
///    "hash": {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MissingTrieValue {
    pub context: MissingTrieValueContext,
    pub hash: CryptoHash,
}
///Contexts in which `StorageError::MissingTrieValue` error might occur.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "MissingTrieValueContext",
///  "description": "Contexts in which `StorageError::MissingTrieValue` error might occur.",
///  "oneOf": [
///    {
///      "description": "Missing trie value when reading from TrieIterator.",
///      "type": "string",
///      "const": "TrieIterator"
///    },
///    {
///      "description": "Missing trie value when reading from TriePrefetchingStorage.",
///      "type": "string",
///      "const": "TriePrefetchingStorage"
///    },
///    {
///      "description": "Missing trie value when reading from TrieMemoryPartialStorage.",
///      "type": "string",
///      "const": "TrieMemoryPartialStorage"
///    },
///    {
///      "description": "Missing trie value when reading from TrieStorage.",
///      "type": "string",
///      "const": "TrieStorage"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum MissingTrieValueContext {
    ///Missing trie value when reading from TrieIterator.
    TrieIterator,
    ///Missing trie value when reading from TriePrefetchingStorage.
    TriePrefetchingStorage,
    ///Missing trie value when reading from TrieMemoryPartialStorage.
    TrieMemoryPartialStorage,
    ///Missing trie value when reading from TrieStorage.
    TrieStorage,
}
impl ::std::fmt::Display for MissingTrieValueContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TrieIterator => f.write_str("TrieIterator"),
            Self::TriePrefetchingStorage => f.write_str("TriePrefetchingStorage"),
            Self::TrieMemoryPartialStorage => f.write_str("TrieMemoryPartialStorage"),
            Self::TrieStorage => f.write_str("TrieStorage"),
        }
    }
}
impl ::std::str::FromStr for MissingTrieValueContext {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TrieIterator" => Ok(Self::TrieIterator),
            "TriePrefetchingStorage" => Ok(Self::TriePrefetchingStorage),
            "TrieMemoryPartialStorage" => Ok(Self::TrieMemoryPartialStorage),
            "TrieStorage" => Ok(Self::TrieStorage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MissingTrieValueContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MissingTrieValueContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MissingTrieValueContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MutableConfigValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "MutableConfigValue",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct MutableConfigValue(pub ::std::string::String);
impl ::std::ops::Deref for MutableConfigValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MutableConfigValue> for ::std::string::String {
    fn from(value: MutableConfigValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for MutableConfigValue {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for MutableConfigValue {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for MutableConfigValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`NearGas`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "NearGas",
///  "type": "integer",
///  "format": "uint64",
///  "minimum": 0.0
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NearGas(pub u64);
impl ::std::ops::Deref for NearGas {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl ::std::convert::From<NearGas> for u64 {
    fn from(value: NearGas) -> Self {
        value.0
    }
}
impl ::std::convert::From<u64> for NearGas {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for NearGas {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for NearGas {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NearGas {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for NearGas {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`NearToken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "NearToken",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct NearToken(pub ::std::string::String);
impl ::std::ops::Deref for NearToken {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NearToken> for ::std::string::String {
    fn from(value: NearToken) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for NearToken {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for NearToken {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for NearToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`NetworkInfoView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "NetworkInfoView",
///  "type": "object",
///  "required": [
///    "connected_peers",
///    "known_producers",
///    "num_connected_peers",
///    "peer_max_count",
///    "tier1_accounts_data",
///    "tier1_accounts_keys",
///    "tier1_connections"
///  ],
///  "properties": {
///    "connected_peers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PeerInfoView"
///      }
///    },
///    "known_producers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/KnownProducerView"
///      }
///    },
///    "num_connected_peers": {
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "peer_max_count": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "tier1_accounts_data": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AccountDataView"
///      }
///    },
///    "tier1_accounts_keys": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PublicKey"
///      }
///    },
///    "tier1_connections": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PeerInfoView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct NetworkInfoView {
    pub connected_peers: ::std::vec::Vec<PeerInfoView>,
    pub known_producers: ::std::vec::Vec<KnownProducerView>,
    pub num_connected_peers: u32,
    pub peer_max_count: u32,
    pub tier1_accounts_data: ::std::vec::Vec<AccountDataView>,
    pub tier1_accounts_keys: ::std::vec::Vec<PublicKey>,
    pub tier1_connections: ::std::vec::Vec<PeerInfoView>,
}
///`NextEpochValidatorInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "NextEpochValidatorInfo",
///  "type": "object",
///  "required": [
///    "account_id",
///    "public_key",
///    "shards",
///    "stake"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "public_key": {
///      "$ref": "#/definitions/PublicKey"
///    },
///    "shards": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ShardId"
///      }
///    },
///    "stake": {
///      "$ref": "#/definitions/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct NextEpochValidatorInfo {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub shards: ::std::vec::Vec<ShardId>,
    pub stake: NearToken,
}
///An Action that can be included in a transaction or receipt, excluding delegate actions. This type represents all possible action types except DelegateAction to prevent infinite recursion in meta-transactions.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "NonDelegateAction",
///  "description": "An Action that can be included in a transaction or receipt, excluding delegate actions. This type represents all possible action types except DelegateAction to prevent infinite recursion in meta-transactions.",
///  "oneOf": [
///    {
///      "title": "CreateAccount",
///      "description": "Create an (sub)account using a transaction `receiver_id` as an ID for\na new account ID must pass validation rules described here\n<https://nomicon.io/DataStructures/Account>.",
///      "type": "object",
///      "required": [
///        "CreateAccount"
///      ],
///      "properties": {
///        "CreateAccount": {
///          "$ref": "#/definitions/CreateAccountAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeployContract",
///      "description": "Sets a Wasm code to a receiver_id",
///      "type": "object",
///      "required": [
///        "DeployContract"
///      ],
///      "properties": {
///        "DeployContract": {
///          "$ref": "#/definitions/DeployContractAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "FunctionCall",
///      "type": "object",
///      "required": [
///        "FunctionCall"
///      ],
///      "properties": {
///        "FunctionCall": {
///          "$ref": "#/definitions/FunctionCallAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Transfer",
///      "type": "object",
///      "required": [
///        "Transfer"
///      ],
///      "properties": {
///        "Transfer": {
///          "$ref": "#/definitions/TransferAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Stake",
///      "type": "object",
///      "required": [
///        "Stake"
///      ],
///      "properties": {
///        "Stake": {
///          "$ref": "#/definitions/StakeAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "AddKey",
///      "type": "object",
///      "required": [
///        "AddKey"
///      ],
///      "properties": {
///        "AddKey": {
///          "$ref": "#/definitions/AddKeyAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeleteKey",
///      "type": "object",
///      "required": [
///        "DeleteKey"
///      ],
///      "properties": {
///        "DeleteKey": {
///          "$ref": "#/definitions/DeleteKeyAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeleteAccount",
///      "type": "object",
///      "required": [
///        "DeleteAccount"
///      ],
///      "properties": {
///        "DeleteAccount": {
///          "$ref": "#/definitions/DeleteAccountAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeployGlobalContract",
///      "type": "object",
///      "required": [
///        "DeployGlobalContract"
///      ],
///      "properties": {
///        "DeployGlobalContract": {
///          "$ref": "#/definitions/DeployGlobalContractAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "UseGlobalContract",
///      "type": "object",
///      "required": [
///        "UseGlobalContract"
///      ],
///      "properties": {
///        "UseGlobalContract": {
///          "$ref": "#/definitions/UseGlobalContractAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "DeterministicStateInit",
///      "type": "object",
///      "required": [
///        "DeterministicStateInit"
///      ],
///      "properties": {
///        "DeterministicStateInit": {
///          "$ref": "#/definitions/DeterministicStateInitAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "TransferToGasKey",
///      "type": "object",
///      "required": [
///        "TransferToGasKey"
///      ],
///      "properties": {
///        "TransferToGasKey": {
///          "$ref": "#/definitions/TransferToGasKeyAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "WithdrawFromGasKey",
///      "type": "object",
///      "required": [
///        "WithdrawFromGasKey"
///      ],
///      "properties": {
///        "WithdrawFromGasKey": {
///          "$ref": "#/definitions/WithdrawFromGasKeyAction"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum NonDelegateAction {
    /**Create an (sub)account using a transaction `receiver_id` as an ID for
a new account ID must pass validation rules described here
<https://nomicon.io/DataStructures/Account>.*/
    CreateAccount(CreateAccountAction),
    ///Sets a Wasm code to a receiver_id
    DeployContract(DeployContractAction),
    FunctionCall(FunctionCallAction),
    Transfer(TransferAction),
    Stake(StakeAction),
    AddKey(AddKeyAction),
    DeleteKey(DeleteKeyAction),
    DeleteAccount(DeleteAccountAction),
    DeployGlobalContract(DeployGlobalContractAction),
    UseGlobalContract(UseGlobalContractAction),
    DeterministicStateInit(DeterministicStateInitAction),
    TransferToGasKey(TransferToGasKeyAction),
    WithdrawFromGasKey(WithdrawFromGasKeyAction),
}
impl ::std::convert::From<CreateAccountAction> for NonDelegateAction {
    fn from(value: CreateAccountAction) -> Self {
        Self::CreateAccount(value)
    }
}
impl ::std::convert::From<DeployContractAction> for NonDelegateAction {
    fn from(value: DeployContractAction) -> Self {
        Self::DeployContract(value)
    }
}
impl ::std::convert::From<FunctionCallAction> for NonDelegateAction {
    fn from(value: FunctionCallAction) -> Self {
        Self::FunctionCall(value)
    }
}
impl ::std::convert::From<TransferAction> for NonDelegateAction {
    fn from(value: TransferAction) -> Self {
        Self::Transfer(value)
    }
}
impl ::std::convert::From<StakeAction> for NonDelegateAction {
    fn from(value: StakeAction) -> Self {
        Self::Stake(value)
    }
}
impl ::std::convert::From<AddKeyAction> for NonDelegateAction {
    fn from(value: AddKeyAction) -> Self {
        Self::AddKey(value)
    }
}
impl ::std::convert::From<DeleteKeyAction> for NonDelegateAction {
    fn from(value: DeleteKeyAction) -> Self {
        Self::DeleteKey(value)
    }
}
impl ::std::convert::From<DeleteAccountAction> for NonDelegateAction {
    fn from(value: DeleteAccountAction) -> Self {
        Self::DeleteAccount(value)
    }
}
impl ::std::convert::From<DeployGlobalContractAction> for NonDelegateAction {
    fn from(value: DeployGlobalContractAction) -> Self {
        Self::DeployGlobalContract(value)
    }
}
impl ::std::convert::From<UseGlobalContractAction> for NonDelegateAction {
    fn from(value: UseGlobalContractAction) -> Self {
        Self::UseGlobalContract(value)
    }
}
impl ::std::convert::From<DeterministicStateInitAction> for NonDelegateAction {
    fn from(value: DeterministicStateInitAction) -> Self {
        Self::DeterministicStateInit(value)
    }
}
impl ::std::convert::From<TransferToGasKeyAction> for NonDelegateAction {
    fn from(value: TransferToGasKeyAction) -> Self {
        Self::TransferToGasKey(value)
    }
}
impl ::std::convert::From<WithdrawFromGasKeyAction> for NonDelegateAction {
    fn from(value: WithdrawFromGasKeyAction) -> Self {
        Self::WithdrawFromGasKey(value)
    }
}
///Peer id is the public key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "PeerId",
///  "description": "Peer id is the public key.",
///  "allOf": [
///    {
///      "$ref": "#/definitions/PublicKey"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PeerId(pub PublicKey);
impl ::std::ops::Deref for PeerId {
    type Target = PublicKey;
    fn deref(&self) -> &PublicKey {
        &self.0
    }
}
impl ::std::convert::From<PeerId> for PublicKey {
    fn from(value: PeerId) -> Self {
        value.0
    }
}
impl ::std::convert::From<PublicKey> for PeerId {
    fn from(value: PublicKey) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PeerId {
    type Err = <PublicKey as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PeerId {
    type Error = <PublicKey as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PeerId {
    type Error = <PublicKey as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for PeerId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`PeerInfoView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "PeerInfoView",
///  "type": "object",
///  "required": [
///    "addr",
///    "archival",
///    "connection_established_time_millis",
///    "is_highest_block_invalid",
///    "is_outbound_peer",
///    "last_time_peer_requested_millis",
///    "last_time_received_message_millis",
///    "nonce",
///    "peer_id",
///    "received_bytes_per_sec",
///    "sent_bytes_per_sec",
///    "tracked_shards"
///  ],
///  "properties": {
///    "account_id": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "addr": {
///      "type": "string"
///    },
///    "archival": {
///      "type": "boolean"
///    },
///    "block_hash": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "connection_established_time_millis": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "is_highest_block_invalid": {
///      "type": "boolean"
///    },
///    "is_outbound_peer": {
///      "type": "boolean"
///    },
///    "last_time_peer_requested_millis": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "last_time_received_message_millis": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "nonce": {
///      "description": "Connection nonce.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "peer_id": {
///      "$ref": "#/definitions/PublicKey"
///    },
///    "received_bytes_per_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "sent_bytes_per_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "tracked_shards": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ShardId"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PeerInfoView {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub account_id: ::std::option::Option<AccountId>,
    pub addr: ::std::string::String,
    pub archival: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_hash: ::std::option::Option<CryptoHash>,
    pub connection_established_time_millis: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<u64>,
    pub is_highest_block_invalid: bool,
    pub is_outbound_peer: bool,
    pub last_time_peer_requested_millis: u64,
    pub last_time_received_message_millis: u64,
    ///Connection nonce.
    pub nonce: u64,
    pub peer_id: PublicKey,
    pub received_bytes_per_sec: u64,
    pub sent_bytes_per_sec: u64,
    pub tracked_shards: ::std::vec::Vec<ShardId>,
}
///Error that can occur while preparing or executing Wasm smart-contract.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "PrepareError",
///  "description": "Error that can occur while preparing or executing Wasm smart-contract.",
///  "oneOf": [
///    {
///      "description": "Error happened while serializing the module.",
///      "type": "string",
///      "const": "Serialization"
///    },
///    {
///      "description": "Error happened while deserializing the module.",
///      "type": "string",
///      "const": "Deserialization"
///    },
///    {
///      "description": "Internal memory declaration has been found in the module.",
///      "type": "string",
///      "const": "InternalMemoryDeclared"
///    },
///    {
///      "description": "Gas instrumentation failed.\n\nThis most likely indicates the module isn't valid.",
///      "type": "string",
///      "const": "GasInstrumentation"
///    },
///    {
///      "description": "Stack instrumentation failed.\n\nThis  most likely indicates the module isn't valid.",
///      "type": "string",
///      "const": "StackHeightInstrumentation"
///    },
///    {
///      "description": "Error happened during instantiation.\n\nThis might indicate that `start` function trapped, or module isn't\ninstantiable and/or un-linkable.",
///      "type": "string",
///      "const": "Instantiate"
///    },
///    {
///      "description": "Error creating memory.",
///      "type": "string",
///      "const": "Memory"
///    },
///    {
///      "description": "Contract contains too many functions.",
///      "type": "string",
///      "const": "TooManyFunctions"
///    },
///    {
///      "description": "Contract contains too many locals.",
///      "type": "string",
///      "const": "TooManyLocals"
///    },
///    {
///      "description": "Contract contains too many tables.",
///      "type": "string",
///      "const": "TooManyTables"
///    },
///    {
///      "description": "Contract contains too many table elements.",
///      "type": "string",
///      "const": "TooManyTableElements"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PrepareError {
    ///Error happened while serializing the module.
    Serialization,
    ///Error happened while deserializing the module.
    Deserialization,
    ///Internal memory declaration has been found in the module.
    InternalMemoryDeclared,
    /**Gas instrumentation failed.

This most likely indicates the module isn't valid.*/
    GasInstrumentation,
    /**Stack instrumentation failed.

This  most likely indicates the module isn't valid.*/
    StackHeightInstrumentation,
    /**Error happened during instantiation.

This might indicate that `start` function trapped, or module isn't
instantiable and/or un-linkable.*/
    Instantiate,
    ///Error creating memory.
    Memory,
    ///Contract contains too many functions.
    TooManyFunctions,
    ///Contract contains too many locals.
    TooManyLocals,
    ///Contract contains too many tables.
    TooManyTables,
    ///Contract contains too many table elements.
    TooManyTableElements,
}
impl ::std::fmt::Display for PrepareError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Serialization => f.write_str("Serialization"),
            Self::Deserialization => f.write_str("Deserialization"),
            Self::InternalMemoryDeclared => f.write_str("InternalMemoryDeclared"),
            Self::GasInstrumentation => f.write_str("GasInstrumentation"),
            Self::StackHeightInstrumentation => f.write_str("StackHeightInstrumentation"),
            Self::Instantiate => f.write_str("Instantiate"),
            Self::Memory => f.write_str("Memory"),
            Self::TooManyFunctions => f.write_str("TooManyFunctions"),
            Self::TooManyLocals => f.write_str("TooManyLocals"),
            Self::TooManyTables => f.write_str("TooManyTables"),
            Self::TooManyTableElements => f.write_str("TooManyTableElements"),
        }
    }
}
impl ::std::str::FromStr for PrepareError {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Serialization" => Ok(Self::Serialization),
            "Deserialization" => Ok(Self::Deserialization),
            "InternalMemoryDeclared" => Ok(Self::InternalMemoryDeclared),
            "GasInstrumentation" => Ok(Self::GasInstrumentation),
            "StackHeightInstrumentation" => Ok(Self::StackHeightInstrumentation),
            "Instantiate" => Ok(Self::Instantiate),
            "Memory" => Ok(Self::Memory),
            "TooManyFunctions" => Ok(Self::TooManyFunctions),
            "TooManyLocals" => Ok(Self::TooManyLocals),
            "TooManyTables" => Ok(Self::TooManyTables),
            "TooManyTableElements" => Ok(Self::TooManyTableElements),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PrepareError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PrepareError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PrepareError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Configures whether the node checks the next or the next next epoch for network version compatibility.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ProtocolVersionCheckConfig",
///  "description": "Configures whether the node checks the next or the next next epoch for network version compatibility.",
///  "type": "string",
///  "enum": [
///    "Next",
///    "NextNext"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ProtocolVersionCheckConfig {
    Next,
    NextNext,
}
impl ::std::fmt::Display for ProtocolVersionCheckConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Next => f.write_str("Next"),
            Self::NextNext => f.write_str("NextNext"),
        }
    }
}
impl ::std::str::FromStr for ProtocolVersionCheckConfig {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Next" => Ok(Self::Next),
            "NextNext" => Ok(Self::NextNext),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProtocolVersionCheckConfig {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ProtocolVersionCheckConfig {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProtocolVersionCheckConfig {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PublicKey`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "PublicKey",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct PublicKey(pub ::std::string::String);
impl ::std::ops::Deref for PublicKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicKey> for ::std::string::String {
    fn from(value: PublicKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for PublicKey {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PublicKey {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`QueryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "QueryRequest",
///  "oneOf": [
///    {
///      "title": "ViewAccount",
///      "type": "object",
///      "required": [
///        "account_id",
///        "request_type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "view_account"
///        }
///      }
///    },
///    {
///      "title": "ViewCode",
///      "type": "object",
///      "required": [
///        "account_id",
///        "request_type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "view_code"
///        }
///      }
///    },
///    {
///      "title": "ViewState",
///      "type": "object",
///      "required": [
///        "account_id",
///        "prefix_base64",
///        "request_type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "include_proof": {
///          "type": "boolean"
///        },
///        "prefix_base64": {
///          "$ref": "#/definitions/StoreKey"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "view_state"
///        }
///      }
///    },
///    {
///      "title": "ViewAccessKey",
///      "type": "object",
///      "required": [
///        "account_id",
///        "public_key",
///        "request_type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "public_key": {
///          "$ref": "#/definitions/PublicKey"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "view_access_key"
///        }
///      }
///    },
///    {
///      "title": "ViewAccessKeyList",
///      "type": "object",
///      "required": [
///        "account_id",
///        "request_type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "view_access_key_list"
///        }
///      }
///    },
///    {
///      "title": "ViewGasKeyNonces",
///      "type": "object",
///      "required": [
///        "account_id",
///        "public_key",
///        "request_type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "public_key": {
///          "$ref": "#/definitions/PublicKey"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "view_gas_key_nonces"
///        }
///      }
///    },
///    {
///      "title": "CallFunction",
///      "type": "object",
///      "required": [
///        "account_id",
///        "args_base64",
///        "method_name",
///        "request_type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "args_base64": {
///          "$ref": "#/definitions/FunctionArgs"
///        },
///        "method_name": {
///          "type": "string"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "call_function"
///        }
///      }
///    },
///    {
///      "title": "ViewGlobalContractCode",
///      "type": "object",
///      "required": [
///        "code_hash",
///        "request_type"
///      ],
///      "properties": {
///        "code_hash": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "view_global_contract_code"
///        }
///      }
///    },
///    {
///      "title": "ViewGlobalContractCodeByAccountId",
///      "type": "object",
///      "required": [
///        "account_id",
///        "request_type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "request_type": {
///          "type": "string",
///          "const": "view_global_contract_code_by_account_id"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "request_type")]
pub enum QueryRequest {
    ///ViewAccount
    #[serde(rename = "view_account")]
    ViewAccount { account_id: AccountId },
    ///ViewCode
    #[serde(rename = "view_code")]
    ViewCode { account_id: AccountId },
    ///ViewState
    #[serde(rename = "view_state")]
    ViewState {
        account_id: AccountId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        include_proof: ::std::option::Option<bool>,
        prefix_base64: StoreKey,
    },
    ///ViewAccessKey
    #[serde(rename = "view_access_key")]
    ViewAccessKey { account_id: AccountId, public_key: PublicKey },
    ///ViewAccessKeyList
    #[serde(rename = "view_access_key_list")]
    ViewAccessKeyList { account_id: AccountId },
    ///ViewGasKeyNonces
    #[serde(rename = "view_gas_key_nonces")]
    ViewGasKeyNonces { account_id: AccountId, public_key: PublicKey },
    ///CallFunction
    #[serde(rename = "call_function")]
    CallFunction {
        account_id: AccountId,
        args_base64: FunctionArgs,
        method_name: ::std::string::String,
    },
    ///ViewGlobalContractCode
    #[serde(rename = "view_global_contract_code")]
    ViewGlobalContractCode { code_hash: CryptoHash },
    ///ViewGlobalContractCodeByAccountId
    #[serde(rename = "view_global_contract_code_by_account_id")]
    ViewGlobalContractCodeByAccountId { account_id: AccountId },
}
///`ReceiptEnumView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ReceiptEnumView",
///  "oneOf": [
///    {
///      "title": "Action",
///      "type": "object",
///      "required": [
///        "Action"
///      ],
///      "properties": {
///        "Action": {
///          "type": "object",
///          "required": [
///            "actions",
///            "gas_price",
///            "input_data_ids",
///            "output_data_receivers",
///            "signer_id",
///            "signer_public_key"
///          ],
///          "properties": {
///            "actions": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/ActionView"
///              }
///            },
///            "gas_price": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "input_data_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/CryptoHash"
///              }
///            },
///            "is_promise_yield": {
///              "default": false,
///              "type": "boolean"
///            },
///            "output_data_receivers": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/DataReceiverView"
///              }
///            },
///            "refund_to": {
///              "oneOf": [
///                {
///                  "$ref": "#/definitions/AccountId"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "signer_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "signer_public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Data",
///      "type": "object",
///      "required": [
///        "Data"
///      ],
///      "properties": {
///        "Data": {
///          "type": "object",
///          "required": [
///            "data_id"
///          ],
///          "properties": {
///            "data": {
///              "default": null,
///              "type": [
///                "string",
///                "null"
///              ]
///            },
///            "data_id": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "is_promise_resume": {
///              "default": false,
///              "type": "boolean"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "GlobalContractDistribution",
///      "type": "object",
///      "required": [
///        "GlobalContractDistribution"
///      ],
///      "properties": {
///        "GlobalContractDistribution": {
///          "type": "object",
///          "required": [
///            "already_delivered_shards",
///            "code",
///            "id",
///            "target_shard"
///          ],
///          "properties": {
///            "already_delivered_shards": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/ShardId"
///              }
///            },
///            "code": {
///              "type": "string"
///            },
///            "id": {
///              "$ref": "#/definitions/GlobalContractIdentifier"
///            },
///            "target_shard": {
///              "$ref": "#/definitions/ShardId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ReceiptEnumView {
    Action {
        actions: ::std::vec::Vec<ActionView>,
        gas_price: NearToken,
        input_data_ids: ::std::vec::Vec<CryptoHash>,
        #[serde(default)]
        is_promise_yield: bool,
        output_data_receivers: ::std::vec::Vec<DataReceiverView>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        refund_to: ::std::option::Option<AccountId>,
        signer_id: AccountId,
        signer_public_key: PublicKey,
    },
    Data {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        data: ::std::option::Option<::std::string::String>,
        data_id: CryptoHash,
        #[serde(default)]
        is_promise_resume: bool,
    },
    GlobalContractDistribution {
        already_delivered_shards: ::std::vec::Vec<ShardId>,
        code: ::std::string::String,
        id: GlobalContractIdentifier,
        target_shard: ShardId,
    },
}
///Describes the error for validating a receipt.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ReceiptValidationError",
///  "description": "Describes the error for validating a receipt.",
///  "oneOf": [
///    {
///      "title": "InvalidPredecessorId",
///      "description": "The `predecessor_id` of a Receipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidPredecessorId"
///      ],
///      "properties": {
///        "InvalidPredecessorId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidReceiverId",
///      "description": "The `receiver_id` of a Receipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidReceiverId"
///      ],
///      "properties": {
///        "InvalidReceiverId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidSignerId",
///      "description": "The `signer_id` of an ActionReceipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidSignerId"
///      ],
///      "properties": {
///        "InvalidSignerId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidDataReceiverId",
///      "description": "The `receiver_id` of a DataReceiver within an ActionReceipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidDataReceiverId"
///      ],
///      "properties": {
///        "InvalidDataReceiverId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ReturnedValueLengthExceeded",
///      "description": "The length of the returned data exceeded the limit in a DataReceipt.",
///      "type": "object",
///      "required": [
///        "ReturnedValueLengthExceeded"
///      ],
///      "properties": {
///        "ReturnedValueLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "NumberInputDataDependenciesExceeded",
///      "description": "The number of input data dependencies exceeds the limit in an ActionReceipt.",
///      "type": "object",
///      "required": [
///        "NumberInputDataDependenciesExceeded"
///      ],
///      "properties": {
///        "NumberInputDataDependenciesExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "number_of_input_data_dependencies"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "number_of_input_data_dependencies": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ActionsValidation",
///      "description": "An error occurred while validating actions of an ActionReceipt.",
///      "type": "object",
///      "required": [
///        "ActionsValidation"
///      ],
///      "properties": {
///        "ActionsValidation": {
///          "$ref": "#/definitions/ActionsValidationError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ReceiptSizeExceeded",
///      "description": "Receipt is bigger than the limit.",
///      "type": "object",
///      "required": [
///        "ReceiptSizeExceeded"
///      ],
///      "properties": {
///        "ReceiptSizeExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "size"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "size": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidRefundTo",
///      "description": "The `refund_to` of an ActionReceipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidRefundTo"
///      ],
///      "properties": {
///        "InvalidRefundTo": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ReceiptValidationError {
    ///The `predecessor_id` of a Receipt is not valid.
    InvalidPredecessorId { account_id: ::std::string::String },
    ///The `receiver_id` of a Receipt is not valid.
    InvalidReceiverId { account_id: ::std::string::String },
    ///The `signer_id` of an ActionReceipt is not valid.
    InvalidSignerId { account_id: ::std::string::String },
    ///The `receiver_id` of a DataReceiver within an ActionReceipt is not valid.
    InvalidDataReceiverId { account_id: ::std::string::String },
    ///The length of the returned data exceeded the limit in a DataReceipt.
    ReturnedValueLengthExceeded { length: u64, limit: u64 },
    ///The number of input data dependencies exceeds the limit in an ActionReceipt.
    NumberInputDataDependenciesExceeded {
        limit: u64,
        number_of_input_data_dependencies: u64,
    },
    ///An error occurred while validating actions of an ActionReceipt.
    ActionsValidation(ActionsValidationError),
    ///Receipt is bigger than the limit.
    ReceiptSizeExceeded { limit: u64, size: u64 },
    ///The `refund_to` of an ActionReceipt is not valid.
    InvalidRefundTo { account_id: ::std::string::String },
}
impl ::std::convert::From<ActionsValidationError> for ReceiptValidationError {
    fn from(value: ActionsValidationError) -> Self {
        Self::ActionsValidation(value)
    }
}
///`ReceiptView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ReceiptView",
///  "type": "object",
///  "required": [
///    "predecessor_id",
///    "receipt",
///    "receipt_id",
///    "receiver_id"
///  ],
///  "properties": {
///    "predecessor_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "priority": {
///      "description": "Deprecated, retained for backward compatibility.",
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "receipt": {
///      "$ref": "#/definitions/ReceiptEnumView"
///    },
///    "receipt_id": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "receiver_id": {
///      "$ref": "#/definitions/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ReceiptView {
    pub predecessor_id: AccountId,
    ///Deprecated, retained for backward compatibility.
    #[serde(default)]
    pub priority: u64,
    pub receipt: ReceiptEnumView,
    pub receipt_id: CryptoHash,
    pub receiver_id: AccountId,
}
///`RpcBlockRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcBlockRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "BlockId",
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        }
///      }
///    },
///    {
///      "title": "Finality",
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/definitions/Finality"
///        }
///      }
///    },
///    {
///      "title": "SyncCheckpoint",
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/definitions/SyncCheckpoint"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum RpcBlockRequest {
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "finality")]
    Finality(Finality),
    #[serde(rename = "sync_checkpoint")]
    SyncCheckpoint(SyncCheckpoint),
}
impl ::std::convert::From<BlockId> for RpcBlockRequest {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
impl ::std::convert::From<Finality> for RpcBlockRequest {
    fn from(value: Finality) -> Self {
        Self::Finality(value)
    }
}
impl ::std::convert::From<SyncCheckpoint> for RpcBlockRequest {
    fn from(value: SyncCheckpoint) -> Self {
        Self::SyncCheckpoint(value)
    }
}
///`RpcBlockResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcBlockResponse",
///  "type": "object",
///  "required": [
///    "author",
///    "chunks",
///    "header"
///  ],
///  "properties": {
///    "author": {
///      "description": "The AccountId of the author of the Block",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        }
///      ]
///    },
///    "chunks": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChunkHeaderView"
///      }
///    },
///    "header": {
///      "$ref": "#/definitions/BlockHeaderView"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcBlockResponse {
    ///The AccountId of the author of the Block
    pub author: AccountId,
    pub chunks: ::std::vec::Vec<ChunkHeaderView>,
    pub header: BlockHeaderView,
}
///`RpcChunkRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcChunkRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "BlockShardId",
///      "type": "object",
///      "required": [
///        "block_id",
///        "shard_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        },
///        "shard_id": {
///          "$ref": "#/definitions/ShardId"
///        }
///      }
///    },
///    {
///      "title": "ChunkHash",
///      "type": "object",
///      "required": [
///        "chunk_id"
///      ],
///      "properties": {
///        "chunk_id": {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcChunkRequest {
    BlockShardId { block_id: BlockId, shard_id: ShardId },
    ChunkHash { chunk_id: CryptoHash },
}
///`RpcChunkResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcChunkResponse",
///  "type": "object",
///  "required": [
///    "author",
///    "header",
///    "receipts",
///    "transactions"
///  ],
///  "properties": {
///    "author": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "header": {
///      "$ref": "#/definitions/ChunkHeaderView"
///    },
///    "receipts": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ReceiptView"
///      }
///    },
///    "transactions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/SignedTransactionView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcChunkResponse {
    pub author: AccountId,
    pub header: ChunkHeaderView,
    pub receipts: ::std::vec::Vec<ReceiptView>,
    pub transactions: ::std::vec::Vec<SignedTransactionView>,
}
///`RpcClientConfigRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcClientConfigRequest",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcClientConfigRequest(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for RpcClientConfigRequest {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<RpcClientConfigRequest>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: RpcClientConfigRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for RpcClientConfigRequest {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///ClientConfig where some fields can be updated at runtime.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcClientConfigResponse",
///  "description": "ClientConfig where some fields can be updated at runtime.",
///  "type": "object",
///  "required": [
///    "archive",
///    "block_fetch_horizon",
///    "block_header_fetch_horizon",
///    "block_production_tracking_delay",
///    "catchup_step_period",
///    "chain_id",
///    "chunk_request_retry_period",
///    "chunk_validation_threads",
///    "chunk_wait_mult",
///    "chunks_cache_height_horizon",
///    "client_background_migration_threads",
///    "disable_tx_routing",
///    "doomslug_step_period",
///    "enable_early_prepare_transactions",
///    "enable_multiline_logging",
///    "enable_statistics_export",
///    "epoch_length",
///    "epoch_sync",
///    "expected_shutdown",
///    "gc",
///    "header_sync_expected_height_per_second",
///    "header_sync_initial_timeout",
///    "header_sync_progress_timeout",
///    "header_sync_stall_ban_timeout",
///    "log_summary_period",
///    "log_summary_style",
///    "max_block_production_delay",
///    "max_block_wait_delay",
///    "min_block_production_delay",
///    "min_num_peers",
///    "num_block_producer_seats",
///    "orphan_state_witness_max_size",
///    "orphan_state_witness_pool_size",
///    "produce_chunk_add_transactions_time_limit",
///    "produce_empty_blocks",
///    "protocol_version_check",
///    "resharding_config",
///    "save_invalid_witnesses",
///    "save_latest_witnesses",
///    "save_state_changes",
///    "save_trie_changes",
///    "save_tx_outcomes",
///    "save_untracked_partial_chunks_parts",
///    "skip_sync_wait",
///    "state_request_server_threads",
///    "state_request_throttle_period",
///    "state_requests_per_throttle_period",
///    "state_sync",
///    "state_sync_enabled",
///    "state_sync_external_backoff",
///    "state_sync_external_timeout",
///    "state_sync_p2p_timeout",
///    "state_sync_retry_backoff",
///    "sync_check_period",
///    "sync_height_threshold",
///    "sync_max_block_requests",
///    "sync_step_period",
///    "tracked_shards_config",
///    "transaction_request_handler_threads",
///    "ttl_account_id_router",
///    "tx_routing_height_horizon",
///    "version",
///    "view_client_threads"
///  ],
///  "properties": {
///    "archive": {
///      "description": "Not clear old data, set `true` for archive nodes.",
///      "type": "boolean"
///    },
///    "block_fetch_horizon": {
///      "description": "Horizon at which instead of fetching block, fetch full state.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "block_header_fetch_horizon": {
///      "description": "Behind this horizon header fetch kicks in.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "block_production_tracking_delay": {
///      "description": "Duration to check for producing / skipping block.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "catchup_step_period": {
///      "description": "Time between check to perform catchup.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "chain_id": {
///      "description": "Chain id for status.",
///      "type": "string"
///    },
///    "chunk_distribution_network": {
///      "description": "Optional config for the Chunk Distribution Network feature.\nIf set to `None` then this node does not participate in the Chunk Distribution Network.\nNodes not participating will still function fine, but possibly with higher\nlatency due to the need of requesting chunks over the peer-to-peer network.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/ChunkDistributionNetworkConfig"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "chunk_request_retry_period": {
///      "description": "Time between checking to re-request chunks.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "chunk_validation_threads": {
///      "description": "Number of threads for ChunkValidationActor pool.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "chunk_wait_mult": {
///      "description": "Multiplier for the wait time for all chunks to be received.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "chunks_cache_height_horizon": {
///      "description": "Height horizon for the chunk cache. A chunk is removed from the cache\nif its height + chunks_cache_height_horizon < largest_seen_height.\nThe default value is DEFAULT_CHUNKS_CACHE_HEIGHT_HORIZON.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "client_background_migration_threads": {
///      "description": "Number of threads to execute background migration work in client.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "cloud_archival_writer": {
///      "description": "Configuration for a cloud-based archival writer. If this config is present, the writer is enabled and\nwrites chunk-related data based on the tracked shards.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/CloudArchivalWriterConfig"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "disable_tx_routing": {
///      "description": "If true, the node won't forward transactions to next the chunk producers.",
///      "type": "boolean"
///    },
///    "doomslug_step_period": {
///      "description": "Time between running doomslug timer.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "enable_early_prepare_transactions": {
///      "description": "If true, transactions for the next chunk will be prepared early, right after the previous chunk's\npost-state is ready. This can help produce chunks faster, for high-throughput chains.\nThe current implementation increases latency on low-load chains, which will be fixed in the future.\nThe default is disabled.",
///      "type": "boolean"
///    },
///    "enable_multiline_logging": {
///      "type": "boolean"
///    },
///    "enable_statistics_export": {
///      "description": "Re-export storage layer statistics as prometheus metrics.",
///      "type": "boolean"
///    },
///    "epoch_length": {
///      "description": "Epoch length.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "epoch_sync": {
///      "description": "Options for epoch sync.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/EpochSyncConfig"
///        }
///      ]
///    },
///    "expected_shutdown": {
///      "description": "Graceful shutdown at expected block height.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/MutableConfigValue"
///        }
///      ]
///    },
///    "gc": {
///      "description": "Garbage collection configuration.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/GCConfig"
///        }
///      ]
///    },
///    "header_sync_expected_height_per_second": {
///      "description": "Expected increase of header head height per second during header sync",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "header_sync_initial_timeout": {
///      "description": "How much time to wait after initial header sync",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "header_sync_progress_timeout": {
///      "description": "How much time to wait after some progress is made in header sync",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "header_sync_stall_ban_timeout": {
///      "description": "How much time to wait before banning a peer in header sync if sync is too slow",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "log_summary_period": {
///      "description": "Period between logging summary information.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "log_summary_style": {
///      "description": "Enable coloring of the logs",
///      "allOf": [
///        {
///          "$ref": "#/definitions/LogSummaryStyle"
///        }
///      ]
///    },
///    "max_block_production_delay": {
///      "description": "Maximum wait for approvals before producing block.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "max_block_wait_delay": {
///      "description": "Maximum duration before skipping given height.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "max_gas_burnt_view": {
///      "description": "Max burnt gas per view method.  If present, overrides value stored in\ngenesis file.  The value only affects the RPCs without influencing the\nprotocol thus changing it per-node doesn’t affect the blockchain.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "min_block_production_delay": {
///      "description": "Minimum duration before producing block.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "min_num_peers": {
///      "description": "Minimum number of peers to start syncing.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats": {
///      "description": "Number of block producer seats",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "orphan_state_witness_max_size": {
///      "description": "Maximum size of state witnesses in the OrphanStateWitnessPool.\n\nWe keep only orphan witnesses which are smaller than this size.\nThis limits the maximum memory usage of OrphanStateWitnessPool.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "orphan_state_witness_pool_size": {
///      "description": "OrphanStateWitnessPool keeps instances of ChunkStateWitness which can't be processed\nbecause the previous block isn't available. The witnesses wait in the pool until the\nrequired block appears. This variable controls how many witnesses can be stored in the pool.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "produce_chunk_add_transactions_time_limit": {
///      "description": "Limit the time of adding transactions to a chunk.\nA node produces a chunk by adding transactions from the transaction pool until\nsome limit is reached. This time limit ensures that adding transactions won't take\nlonger than the specified duration, which helps to produce the chunk quickly.",
///      "type": "string"
///    },
///    "produce_empty_blocks": {
///      "description": "Produce empty blocks, use `false` for testing.",
///      "type": "boolean"
///    },
///    "protocol_version_check": {
///      "description": "Determines whether client should exit if the protocol version is not supported\nfor the next or next next epoch.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ProtocolVersionCheckConfig"
///        }
///      ]
///    },
///    "resharding_config": {
///      "$ref": "#/definitions/MutableConfigValue"
///    },
///    "rpc_addr": {
///      "description": "Listening rpc port for status.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "save_invalid_witnesses": {
///      "description": "Save observed instances of invalid ChunkStateWitness to the database in DBCol::InvalidChunkStateWitnesses.\nSaving invalid witnesses is useful for analysis and debugging.\nThis option can cause extra load on the database and is not recommended for production use.",
///      "type": "boolean"
///    },
///    "save_latest_witnesses": {
///      "description": "Save observed instances of ChunkStateWitness to the database in DBCol::LatestChunkStateWitnesses.\nSaving the latest witnesses is useful for analysis and debugging.\nThis option can cause extra load on the database and is not recommended for production use.",
///      "type": "boolean"
///    },
///    "save_state_changes": {
///      "description": "Whether to persist state changes on disk or not.",
///      "type": "boolean"
///    },
///    "save_trie_changes": {
///      "description": "save_trie_changes should be set to true iff\n- archive if false - non-archival nodes need trie changes to perform garbage collection\n- archive is true, cold_store is configured and migration to split_storage is finished - node\nworking in split storage mode needs trie changes in order to do garbage collection on hot.",
///      "type": "boolean"
///    },
///    "save_tx_outcomes": {
///      "description": "Whether to persist transaction outcomes to disk or not.",
///      "type": "boolean"
///    },
///    "save_untracked_partial_chunks_parts": {
///      "description": "Whether to persist partial chunk parts for untracked shards or not.",
///      "type": "boolean"
///    },
///    "skip_sync_wait": {
///      "description": "Skip waiting for sync (for testing or single node testnet).",
///      "type": "boolean"
///    },
///    "state_request_server_threads": {
///      "description": "Number of threads for StateRequestActor pool.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "state_request_throttle_period": {
///      "description": "Number of seconds between state requests for view client.\nThrottling window for state requests (headers and parts).",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "state_requests_per_throttle_period": {
///      "description": "Maximum number of state requests served per throttle period",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "state_sync": {
///      "description": "Options for syncing state.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/StateSyncConfig"
///        }
///      ]
///    },
///    "state_sync_enabled": {
///      "description": "Whether to use the State Sync mechanism.\nIf disabled, the node will do Block Sync instead of State Sync.",
///      "type": "boolean"
///    },
///    "state_sync_external_backoff": {
///      "description": "Additional waiting period after a failed request to external storage",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "state_sync_external_timeout": {
///      "description": "How long to wait for a response from centralized state sync",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "state_sync_p2p_timeout": {
///      "description": "How long to wait for a response from p2p state sync",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "state_sync_retry_backoff": {
///      "description": "How long to wait after a failed state sync request",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "sync_check_period": {
///      "description": "How often to check that we are not out of sync.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "sync_height_threshold": {
///      "description": "Sync height threshold: below this difference in height don't start syncing.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "sync_max_block_requests": {
///      "description": "Maximum number of block requests to send to peers to sync",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "sync_step_period": {
///      "description": "While syncing, how long to check for each step.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "tracked_shards_config": {
///      "$ref": "#/definitions/TrackedShardsConfig"
///    },
///    "transaction_pool_size_limit": {
///      "description": "Limit of the size of per-shard transaction pool measured in bytes. If not set, the size\nwill be unbounded.",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "transaction_request_handler_threads": {
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "trie_viewer_state_size_limit": {
///      "description": "Upper bound of the byte size of contract state that is still viewable. None is no limit",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "ttl_account_id_router": {
///      "description": "Time to persist Accounts Id in the router without removing them.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "tx_routing_height_horizon": {
///      "description": "If the node is not a chunk producer within that many blocks, then route\nto upcoming chunk producers.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "version": {
///      "description": "Version of the binary.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Version"
///        }
///      ]
///    },
///    "view_client_threads": {
///      "description": "Number of threads for ViewClientActor pool.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcClientConfigResponse {
    ///Not clear old data, set `true` for archive nodes.
    pub archive: bool,
    ///Horizon at which instead of fetching block, fetch full state.
    pub block_fetch_horizon: u64,
    ///Behind this horizon header fetch kicks in.
    pub block_header_fetch_horizon: u64,
    ///Duration to check for producing / skipping block.
    pub block_production_tracking_delay: [u64; 2usize],
    ///Time between check to perform catchup.
    pub catchup_step_period: [u64; 2usize],
    ///Chain id for status.
    pub chain_id: ::std::string::String,
    /**Optional config for the Chunk Distribution Network feature.
If set to `None` then this node does not participate in the Chunk Distribution Network.
Nodes not participating will still function fine, but possibly with higher
latency due to the need of requesting chunks over the peer-to-peer network.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_distribution_network: ::std::option::Option<
        ChunkDistributionNetworkConfig,
    >,
    ///Time between checking to re-request chunks.
    pub chunk_request_retry_period: [u64; 2usize],
    ///Number of threads for ChunkValidationActor pool.
    pub chunk_validation_threads: u32,
    ///Multiplier for the wait time for all chunks to be received.
    pub chunk_wait_mult: [i32; 2usize],
    /**Height horizon for the chunk cache. A chunk is removed from the cache
if its height + chunks_cache_height_horizon < largest_seen_height.
The default value is DEFAULT_CHUNKS_CACHE_HEIGHT_HORIZON.*/
    pub chunks_cache_height_horizon: u64,
    ///Number of threads to execute background migration work in client.
    pub client_background_migration_threads: u32,
    /**Configuration for a cloud-based archival writer. If this config is present, the writer is enabled and
writes chunk-related data based on the tracked shards.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cloud_archival_writer: ::std::option::Option<CloudArchivalWriterConfig>,
    ///If true, the node won't forward transactions to next the chunk producers.
    pub disable_tx_routing: bool,
    ///Time between running doomslug timer.
    pub doomslug_step_period: [u64; 2usize],
    /**If true, transactions for the next chunk will be prepared early, right after the previous chunk's
post-state is ready. This can help produce chunks faster, for high-throughput chains.
The current implementation increases latency on low-load chains, which will be fixed in the future.
The default is disabled.*/
    pub enable_early_prepare_transactions: bool,
    pub enable_multiline_logging: bool,
    ///Re-export storage layer statistics as prometheus metrics.
    pub enable_statistics_export: bool,
    ///Epoch length.
    pub epoch_length: u64,
    ///Options for epoch sync.
    pub epoch_sync: EpochSyncConfig,
    ///Graceful shutdown at expected block height.
    pub expected_shutdown: MutableConfigValue,
    ///Garbage collection configuration.
    pub gc: GcConfig,
    ///Expected increase of header head height per second during header sync
    pub header_sync_expected_height_per_second: u64,
    ///How much time to wait after initial header sync
    pub header_sync_initial_timeout: [u64; 2usize],
    ///How much time to wait after some progress is made in header sync
    pub header_sync_progress_timeout: [u64; 2usize],
    ///How much time to wait before banning a peer in header sync if sync is too slow
    pub header_sync_stall_ban_timeout: [u64; 2usize],
    ///Period between logging summary information.
    pub log_summary_period: [u64; 2usize],
    ///Enable coloring of the logs
    pub log_summary_style: LogSummaryStyle,
    ///Maximum wait for approvals before producing block.
    pub max_block_production_delay: [u64; 2usize],
    ///Maximum duration before skipping given height.
    pub max_block_wait_delay: [u64; 2usize],
    /**Max burnt gas per view method.  If present, overrides value stored in
genesis file.  The value only affects the RPCs without influencing the
protocol thus changing it per-node doesn’t affect the blockchain.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_gas_burnt_view: ::std::option::Option<NearGas>,
    ///Minimum duration before producing block.
    pub min_block_production_delay: [u64; 2usize],
    ///Minimum number of peers to start syncing.
    pub min_num_peers: u32,
    ///Number of block producer seats
    pub num_block_producer_seats: u64,
    /**Maximum size of state witnesses in the OrphanStateWitnessPool.

We keep only orphan witnesses which are smaller than this size.
This limits the maximum memory usage of OrphanStateWitnessPool.*/
    pub orphan_state_witness_max_size: u64,
    /**OrphanStateWitnessPool keeps instances of ChunkStateWitness which can't be processed
because the previous block isn't available. The witnesses wait in the pool until the
required block appears. This variable controls how many witnesses can be stored in the pool.*/
    pub orphan_state_witness_pool_size: u32,
    /**Limit the time of adding transactions to a chunk.
A node produces a chunk by adding transactions from the transaction pool until
some limit is reached. This time limit ensures that adding transactions won't take
longer than the specified duration, which helps to produce the chunk quickly.*/
    pub produce_chunk_add_transactions_time_limit: ::std::string::String,
    ///Produce empty blocks, use `false` for testing.
    pub produce_empty_blocks: bool,
    /**Determines whether client should exit if the protocol version is not supported
for the next or next next epoch.*/
    pub protocol_version_check: ProtocolVersionCheckConfig,
    pub resharding_config: MutableConfigValue,
    ///Listening rpc port for status.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rpc_addr: ::std::option::Option<::std::string::String>,
    /**Save observed instances of invalid ChunkStateWitness to the database in DBCol::InvalidChunkStateWitnesses.
Saving invalid witnesses is useful for analysis and debugging.
This option can cause extra load on the database and is not recommended for production use.*/
    pub save_invalid_witnesses: bool,
    /**Save observed instances of ChunkStateWitness to the database in DBCol::LatestChunkStateWitnesses.
Saving the latest witnesses is useful for analysis and debugging.
This option can cause extra load on the database and is not recommended for production use.*/
    pub save_latest_witnesses: bool,
    ///Whether to persist state changes on disk or not.
    pub save_state_changes: bool,
    /**save_trie_changes should be set to true iff
- archive if false - non-archival nodes need trie changes to perform garbage collection
- archive is true, cold_store is configured and migration to split_storage is finished - node
working in split storage mode needs trie changes in order to do garbage collection on hot.*/
    pub save_trie_changes: bool,
    ///Whether to persist transaction outcomes to disk or not.
    pub save_tx_outcomes: bool,
    ///Whether to persist partial chunk parts for untracked shards or not.
    pub save_untracked_partial_chunks_parts: bool,
    ///Skip waiting for sync (for testing or single node testnet).
    pub skip_sync_wait: bool,
    ///Number of threads for StateRequestActor pool.
    pub state_request_server_threads: u32,
    /**Number of seconds between state requests for view client.
Throttling window for state requests (headers and parts).*/
    pub state_request_throttle_period: [u64; 2usize],
    ///Maximum number of state requests served per throttle period
    pub state_requests_per_throttle_period: u32,
    ///Options for syncing state.
    pub state_sync: StateSyncConfig,
    /**Whether to use the State Sync mechanism.
If disabled, the node will do Block Sync instead of State Sync.*/
    pub state_sync_enabled: bool,
    ///Additional waiting period after a failed request to external storage
    pub state_sync_external_backoff: [u64; 2usize],
    ///How long to wait for a response from centralized state sync
    pub state_sync_external_timeout: [u64; 2usize],
    ///How long to wait for a response from p2p state sync
    pub state_sync_p2p_timeout: [u64; 2usize],
    ///How long to wait after a failed state sync request
    pub state_sync_retry_backoff: [u64; 2usize],
    ///How often to check that we are not out of sync.
    pub sync_check_period: [u64; 2usize],
    ///Sync height threshold: below this difference in height don't start syncing.
    pub sync_height_threshold: u64,
    ///Maximum number of block requests to send to peers to sync
    pub sync_max_block_requests: u32,
    ///While syncing, how long to check for each step.
    pub sync_step_period: [u64; 2usize],
    pub tracked_shards_config: TrackedShardsConfig,
    /**Limit of the size of per-shard transaction pool measured in bytes. If not set, the size
will be unbounded.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transaction_pool_size_limit: ::std::option::Option<u64>,
    pub transaction_request_handler_threads: u32,
    ///Upper bound of the byte size of contract state that is still viewable. None is no limit
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trie_viewer_state_size_limit: ::std::option::Option<u64>,
    ///Time to persist Accounts Id in the router without removing them.
    pub ttl_account_id_router: [u64; 2usize],
    /**If the node is not a chunk producer within that many blocks, then route
to upcoming chunk producers.*/
    pub tx_routing_height_horizon: u64,
    ///Version of the binary.
    pub version: Version,
    ///Number of threads for ViewClientActor pool.
    pub view_client_threads: u32,
}
///`RpcCongestionLevelRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcCongestionLevelRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "BlockShardId",
///      "type": "object",
///      "required": [
///        "block_id",
///        "shard_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        },
///        "shard_id": {
///          "$ref": "#/definitions/ShardId"
///        }
///      }
///    },
///    {
///      "title": "ChunkHash",
///      "type": "object",
///      "required": [
///        "chunk_id"
///      ],
///      "properties": {
///        "chunk_id": {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcCongestionLevelRequest {
    BlockShardId { block_id: BlockId, shard_id: ShardId },
    ChunkHash { chunk_id: CryptoHash },
}
///`RpcCongestionLevelResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcCongestionLevelResponse",
///  "type": "object",
///  "required": [
///    "congestion_level"
///  ],
///  "properties": {
///    "congestion_level": {
///      "type": "number",
///      "format": "double"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcCongestionLevelResponse {
    pub congestion_level: f64,
}
///`RpcGasPriceRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcGasPriceRequest",
///  "type": "object",
///  "properties": {
///    "block_id": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/BlockId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcGasPriceRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_id: ::std::option::Option<BlockId>,
}
impl ::std::default::Default for RpcGasPriceRequest {
    fn default() -> Self {
        Self {
            block_id: Default::default(),
        }
    }
}
///`RpcGasPriceResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcGasPriceResponse",
///  "type": "object",
///  "required": [
///    "gas_price"
///  ],
///  "properties": {
///    "gas_price": {
///      "$ref": "#/definitions/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcGasPriceResponse {
    pub gas_price: NearToken,
}
///`RpcHealthRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcHealthRequest",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcHealthRequest(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for RpcHealthRequest {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<RpcHealthRequest>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: RpcHealthRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for RpcHealthRequest {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`RpcHealthResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcHealthResponse",
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcHealthResponse(pub ());
impl ::std::ops::Deref for RpcHealthResponse {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<RpcHealthResponse> for () {
    fn from(value: RpcHealthResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for RpcHealthResponse {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///`RpcKnownProducer`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcKnownProducer",
///  "type": "object",
///  "required": [
///    "account_id",
///    "peer_id"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "addr": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "peer_id": {
///      "$ref": "#/definitions/PeerId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcKnownProducer {
    pub account_id: AccountId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub addr: ::std::option::Option<::std::string::String>,
    pub peer_id: PeerId,
}
///`RpcLightClientBlockProofRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcLightClientBlockProofRequest",
///  "type": "object",
///  "required": [
///    "block_hash",
///    "light_client_head"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "light_client_head": {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientBlockProofRequest {
    pub block_hash: CryptoHash,
    pub light_client_head: CryptoHash,
}
///`RpcLightClientBlockProofResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcLightClientBlockProofResponse",
///  "type": "object",
///  "required": [
///    "block_header_lite",
///    "block_proof"
///  ],
///  "properties": {
///    "block_header_lite": {
///      "$ref": "#/definitions/LightClientBlockLiteView"
///    },
///    "block_proof": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/MerklePathItem"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientBlockProofResponse {
    pub block_header_lite: LightClientBlockLiteView,
    pub block_proof: ::std::vec::Vec<MerklePathItem>,
}
///`RpcLightClientExecutionProofRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcLightClientExecutionProofRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "Transaction",
///      "type": "object",
///      "required": [
///        "light_client_head",
///        "sender_id",
///        "transaction_hash",
///        "type"
///      ],
///      "properties": {
///        "light_client_head": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "sender_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "transaction_hash": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "const": "transaction"
///        }
///      }
///    },
///    {
///      "title": "Receipt",
///      "type": "object",
///      "required": [
///        "light_client_head",
///        "receipt_id",
///        "receiver_id",
///        "type"
///      ],
///      "properties": {
///        "light_client_head": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "receipt_id": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "receiver_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "const": "receipt"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum RpcLightClientExecutionProofRequest {
    ///Transaction
    #[serde(rename = "transaction")]
    Transaction {
        light_client_head: CryptoHash,
        sender_id: AccountId,
        transaction_hash: CryptoHash,
    },
    ///Receipt
    #[serde(rename = "receipt")]
    Receipt {
        light_client_head: CryptoHash,
        receipt_id: CryptoHash,
        receiver_id: AccountId,
    },
}
///`RpcLightClientExecutionProofResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcLightClientExecutionProofResponse",
///  "type": "object",
///  "required": [
///    "block_header_lite",
///    "block_proof",
///    "outcome_proof",
///    "outcome_root_proof"
///  ],
///  "properties": {
///    "block_header_lite": {
///      "$ref": "#/definitions/LightClientBlockLiteView"
///    },
///    "block_proof": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/MerklePathItem"
///      }
///    },
///    "outcome_proof": {
///      "$ref": "#/definitions/ExecutionOutcomeWithIdView"
///    },
///    "outcome_root_proof": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/MerklePathItem"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientExecutionProofResponse {
    pub block_header_lite: LightClientBlockLiteView,
    pub block_proof: ::std::vec::Vec<MerklePathItem>,
    pub outcome_proof: ExecutionOutcomeWithIdView,
    pub outcome_root_proof: ::std::vec::Vec<MerklePathItem>,
}
///`RpcLightClientNextBlockRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcLightClientNextBlockRequest",
///  "type": "object",
///  "required": [
///    "last_block_hash"
///  ],
///  "properties": {
///    "last_block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientNextBlockRequest {
    pub last_block_hash: CryptoHash,
}
///A state for the current head of a light client. More info [here](https://nomicon.io/ChainSpec/LightClient).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcLightClientNextBlockResponse",
///  "description": "A state for the current head of a light client. More info [here](https://nomicon.io/ChainSpec/LightClient).",
///  "type": "object",
///  "properties": {
///    "approvals_after_next": {
///      "type": "array",
///      "items": {
///        "oneOf": [
///          {
///            "$ref": "#/definitions/Signature"
///          },
///          {
///            "type": "null"
///          }
///        ]
///      }
///    },
///    "inner_lite": {
///      "description": "Inner part of the block header that gets hashed, split into two parts, one that is sent\n   to light clients, and the rest",
///      "allOf": [
///        {
///          "$ref": "#/definitions/BlockHeaderInnerLiteView"
///        }
///      ]
///    },
///    "inner_rest_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "next_block_inner_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "next_bps": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/definitions/ValidatorStakeView"
///      }
///    },
///    "prev_block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientNextBlockResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub approvals_after_next: ::std::vec::Vec<::std::option::Option<Signature>>,
    /**Inner part of the block header that gets hashed, split into two parts, one that is sent
   to light clients, and the rest*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub inner_lite: ::std::option::Option<BlockHeaderInnerLiteView>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub inner_rest_hash: ::std::option::Option<CryptoHash>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next_block_inner_hash: ::std::option::Option<CryptoHash>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next_bps: ::std::option::Option<::std::vec::Vec<ValidatorStakeView>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prev_block_hash: ::std::option::Option<CryptoHash>,
}
impl ::std::default::Default for RpcLightClientNextBlockResponse {
    fn default() -> Self {
        Self {
            approvals_after_next: Default::default(),
            inner_lite: Default::default(),
            inner_rest_hash: Default::default(),
            next_block_inner_hash: Default::default(),
            next_bps: Default::default(),
            prev_block_hash: Default::default(),
        }
    }
}
///`RpcMaintenanceWindowsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcMaintenanceWindowsRequest",
///  "type": "object",
///  "required": [
///    "account_id"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcMaintenanceWindowsRequest {
    pub account_id: AccountId,
}
///`RpcNetworkInfoRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcNetworkInfoRequest",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcNetworkInfoRequest(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for RpcNetworkInfoRequest {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<RpcNetworkInfoRequest>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: RpcNetworkInfoRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for RpcNetworkInfoRequest {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`RpcNetworkInfoResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcNetworkInfoResponse",
///  "type": "object",
///  "required": [
///    "active_peers",
///    "known_producers",
///    "num_active_peers",
///    "peer_max_count",
///    "received_bytes_per_sec",
///    "sent_bytes_per_sec"
///  ],
///  "properties": {
///    "active_peers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RpcPeerInfo"
///      }
///    },
///    "known_producers": {
///      "description": "Accounts of known block and chunk producers from routing table.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RpcKnownProducer"
///      }
///    },
///    "num_active_peers": {
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "peer_max_count": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "received_bytes_per_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "sent_bytes_per_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcNetworkInfoResponse {
    pub active_peers: ::std::vec::Vec<RpcPeerInfo>,
    ///Accounts of known block and chunk producers from routing table.
    pub known_producers: ::std::vec::Vec<RpcKnownProducer>,
    pub num_active_peers: u32,
    pub peer_max_count: u32,
    pub received_bytes_per_sec: u64,
    pub sent_bytes_per_sec: u64,
}
///`RpcPeerInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcPeerInfo",
///  "type": "object",
///  "required": [
///    "id"
///  ],
///  "properties": {
///    "account_id": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "addr": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "id": {
///      "$ref": "#/definitions/PeerId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcPeerInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub account_id: ::std::option::Option<AccountId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub addr: ::std::option::Option<::std::string::String>,
    pub id: PeerId,
}
///`RpcProtocolConfigRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcProtocolConfigRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "BlockId",
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        }
///      }
///    },
///    {
///      "title": "Finality",
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/definitions/Finality"
///        }
///      }
///    },
///    {
///      "title": "SyncCheckpoint",
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/definitions/SyncCheckpoint"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum RpcProtocolConfigRequest {
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "finality")]
    Finality(Finality),
    #[serde(rename = "sync_checkpoint")]
    SyncCheckpoint(SyncCheckpoint),
}
impl ::std::convert::From<BlockId> for RpcProtocolConfigRequest {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
impl ::std::convert::From<Finality> for RpcProtocolConfigRequest {
    fn from(value: Finality) -> Self {
        Self::Finality(value)
    }
}
impl ::std::convert::From<SyncCheckpoint> for RpcProtocolConfigRequest {
    fn from(value: SyncCheckpoint) -> Self {
        Self::SyncCheckpoint(value)
    }
}
///`RpcProtocolConfigResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcProtocolConfigResponse",
///  "type": "object",
///  "required": [
///    "avg_hidden_validator_seats_per_shard",
///    "block_producer_kickout_threshold",
///    "chain_id",
///    "chunk_producer_kickout_threshold",
///    "chunk_validator_only_kickout_threshold",
///    "dynamic_resharding",
///    "epoch_length",
///    "fishermen_threshold",
///    "gas_limit",
///    "gas_price_adjustment_rate",
///    "genesis_height",
///    "genesis_time",
///    "max_gas_price",
///    "max_inflation_rate",
///    "max_kickout_stake_perc",
///    "min_gas_price",
///    "minimum_stake_divisor",
///    "minimum_stake_ratio",
///    "minimum_validators_per_shard",
///    "num_block_producer_seats",
///    "num_block_producer_seats_per_shard",
///    "num_blocks_per_year",
///    "online_max_threshold",
///    "online_min_threshold",
///    "protocol_reward_rate",
///    "protocol_treasury_account",
///    "protocol_upgrade_stake_threshold",
///    "protocol_version",
///    "runtime_config",
///    "shard_layout",
///    "shuffle_shard_assignment_for_chunk_producers",
///    "target_validator_mandates_per_shard",
///    "transaction_validity_period"
///  ],
///  "properties": {
///    "avg_hidden_validator_seats_per_shard": {
///      "description": "Expected number of hidden validators per shard.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "block_producer_kickout_threshold": {
///      "description": "Threshold for kicking out block producers, between 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "chain_id": {
///      "description": "ID of the blockchain. This must be unique for every blockchain.\nIf your testnet blockchains do not have unique chain IDs, you will have a bad time.",
///      "type": "string"
///    },
///    "chunk_producer_kickout_threshold": {
///      "description": "Threshold for kicking out chunk producers, between 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "chunk_validator_only_kickout_threshold": {
///      "description": "Threshold for kicking out nodes which are only chunk validators, between 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "dynamic_resharding": {
///      "description": "Enable dynamic re-sharding.",
///      "type": "boolean"
///    },
///    "epoch_length": {
///      "description": "Epoch length counted in block heights.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "fishermen_threshold": {
///      "description": "Fishermen stake threshold.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "gas_limit": {
///      "description": "Initial gas limit.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearGas"
///        }
///      ]
///    },
///    "gas_price_adjustment_rate": {
///      "description": "Gas price adjustment rate",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "genesis_height": {
///      "description": "Height of genesis block.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "genesis_time": {
///      "description": "Official time of blockchain start.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "max_gas_price": {
///      "description": "Maximum gas price.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "max_inflation_rate": {
///      "description": "Maximum inflation on the total supply every epoch.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "max_kickout_stake_perc": {
///      "description": "Max stake percentage of the validators we will kick out.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "min_gas_price": {
///      "description": "Minimum gas price. It is also the initial gas price.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "minimum_stake_divisor": {
///      "description": "The minimum stake required for staking is last seat price divided by this number.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "minimum_stake_ratio": {
///      "description": "The lowest ratio s/s_total any block producer can have.\nSee <https://github.com/near/NEPs/pull/167> for details",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "minimum_validators_per_shard": {
///      "description": "The minimum number of validators each shard must have",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats": {
///      "description": "Number of block producer seats at genesis.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats_per_shard": {
///      "description": "Defines number of shards and number of block producer seats per each shard at genesis.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_blocks_per_year": {
///      "description": "Expected number of blocks per year",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "online_max_threshold": {
///      "description": "Online maximum threshold above which validator gets full reward.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "online_min_threshold": {
///      "description": "Online minimum threshold below which validator doesn't receive reward.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_reward_rate": {
///      "description": "Protocol treasury rate",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_treasury_account": {
///      "description": "Protocol treasury account",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        }
///      ]
///    },
///    "protocol_upgrade_stake_threshold": {
///      "description": "Threshold of stake that needs to indicate that they ready for upgrade.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_version": {
///      "description": "Current Protocol Version",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "runtime_config": {
///      "description": "Runtime configuration (mostly economics constants).",
///      "allOf": [
///        {
///          "$ref": "#/definitions/RuntimeConfigView"
///        }
///      ]
///    },
///    "shard_layout": {
///      "description": "Layout information regarding how to split accounts to shards",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ShardLayout"
///        }
///      ]
///    },
///    "shuffle_shard_assignment_for_chunk_producers": {
///      "description": "If true, shuffle the chunk producers across shards. In other words, if\nthe shard assignments were `[S_0, S_1, S_2, S_3]` where `S_i` represents\nthe set of chunk producers for shard `i`, if this flag were true, the\nshard assignments might become, for example, `[S_2, S_0, S_3, S_1]`.",
///      "type": "boolean"
///    },
///    "target_validator_mandates_per_shard": {
///      "description": "Number of target chunk validator mandates for each shard.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "transaction_validity_period": {
///      "description": "Number of blocks for which a given transaction is valid",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcProtocolConfigResponse {
    ///Expected number of hidden validators per shard.
    pub avg_hidden_validator_seats_per_shard: ::std::vec::Vec<u64>,
    ///Threshold for kicking out block producers, between 0 and 100.
    pub block_producer_kickout_threshold: u8,
    /**ID of the blockchain. This must be unique for every blockchain.
If your testnet blockchains do not have unique chain IDs, you will have a bad time.*/
    pub chain_id: ::std::string::String,
    ///Threshold for kicking out chunk producers, between 0 and 100.
    pub chunk_producer_kickout_threshold: u8,
    ///Threshold for kicking out nodes which are only chunk validators, between 0 and 100.
    pub chunk_validator_only_kickout_threshold: u8,
    ///Enable dynamic re-sharding.
    pub dynamic_resharding: bool,
    ///Epoch length counted in block heights.
    pub epoch_length: u64,
    ///Fishermen stake threshold.
    pub fishermen_threshold: NearToken,
    ///Initial gas limit.
    pub gas_limit: NearGas,
    ///Gas price adjustment rate
    pub gas_price_adjustment_rate: [i32; 2usize],
    ///Height of genesis block.
    pub genesis_height: u64,
    ///Official time of blockchain start.
    pub genesis_time: ::chrono::DateTime<::chrono::offset::Utc>,
    ///Maximum gas price.
    pub max_gas_price: NearToken,
    ///Maximum inflation on the total supply every epoch.
    pub max_inflation_rate: [i32; 2usize],
    ///Max stake percentage of the validators we will kick out.
    pub max_kickout_stake_perc: u8,
    ///Minimum gas price. It is also the initial gas price.
    pub min_gas_price: NearToken,
    ///The minimum stake required for staking is last seat price divided by this number.
    pub minimum_stake_divisor: u64,
    /**The lowest ratio s/s_total any block producer can have.
See <https://github.com/near/NEPs/pull/167> for details*/
    pub minimum_stake_ratio: [i32; 2usize],
    ///The minimum number of validators each shard must have
    pub minimum_validators_per_shard: u64,
    ///Number of block producer seats at genesis.
    pub num_block_producer_seats: u64,
    ///Defines number of shards and number of block producer seats per each shard at genesis.
    pub num_block_producer_seats_per_shard: ::std::vec::Vec<u64>,
    ///Expected number of blocks per year
    pub num_blocks_per_year: u64,
    ///Online maximum threshold above which validator gets full reward.
    pub online_max_threshold: [i32; 2usize],
    ///Online minimum threshold below which validator doesn't receive reward.
    pub online_min_threshold: [i32; 2usize],
    ///Protocol treasury rate
    pub protocol_reward_rate: [i32; 2usize],
    ///Protocol treasury account
    pub protocol_treasury_account: AccountId,
    ///Threshold of stake that needs to indicate that they ready for upgrade.
    pub protocol_upgrade_stake_threshold: [i32; 2usize],
    ///Current Protocol Version
    pub protocol_version: u32,
    ///Runtime configuration (mostly economics constants).
    pub runtime_config: RuntimeConfigView,
    ///Layout information regarding how to split accounts to shards
    pub shard_layout: ShardLayout,
    /**If true, shuffle the chunk producers across shards. In other words, if
the shard assignments were `[S_0, S_1, S_2, S_3]` where `S_i` represents
the set of chunk producers for shard `i`, if this flag were true, the
shard assignments might become, for example, `[S_2, S_0, S_3, S_1]`.*/
    pub shuffle_shard_assignment_for_chunk_producers: bool,
    ///Number of target chunk validator mandates for each shard.
    pub target_validator_mandates_per_shard: u64,
    ///Number of blocks for which a given transaction is valid
    pub transaction_validity_period: u64,
}
///`RpcQueryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcQueryRequest",
///  "type": "object",
///  "allOf": [
///    {
///      "$ref": "#/definitions/BlockReference"
///    },
///    {
///      "$ref": "#/definitions/QueryRequest"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcQueryRequest {
    Variant0(RpcQueryRequestVariant0),
    Variant1(RpcQueryRequestVariant1),
    Variant2(RpcQueryRequestVariant2),
}
impl ::std::convert::From<RpcQueryRequestVariant0> for RpcQueryRequest {
    fn from(value: RpcQueryRequestVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<RpcQueryRequestVariant1> for RpcQueryRequest {
    fn from(value: RpcQueryRequestVariant1) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<RpcQueryRequestVariant2> for RpcQueryRequest {
    fn from(value: RpcQueryRequestVariant2) -> Self {
        Self::Variant2(value)
    }
}
///`RpcQueryRequestVariant0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "oneOf": [
///        {
///          "title": "ViewAccount",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_account"
///            }
///          }
///        },
///        {
///          "title": "ViewCode",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_code"
///            }
///          }
///        },
///        {
///          "title": "ViewState",
///          "type": "object",
///          "required": [
///            "account_id",
///            "prefix_base64",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "include_proof": {
///              "type": "boolean"
///            },
///            "prefix_base64": {
///              "$ref": "#/definitions/StoreKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_state"
///            }
///          }
///        },
///        {
///          "title": "ViewAccessKey",
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_access_key"
///            }
///          }
///        },
///        {
///          "title": "ViewAccessKeyList",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_access_key_list"
///            }
///          }
///        },
///        {
///          "title": "ViewGasKeyNonces",
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_gas_key_nonces"
///            }
///          }
///        },
///        {
///          "title": "CallFunction",
///          "type": "object",
///          "required": [
///            "account_id",
///            "args_base64",
///            "method_name",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "args_base64": {
///              "$ref": "#/definitions/FunctionArgs"
///            },
///            "method_name": {
///              "type": "string"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "call_function"
///            }
///          }
///        },
///        {
///          "title": "ViewGlobalContractCode",
///          "type": "object",
///          "required": [
///            "code_hash",
///            "request_type"
///          ],
///          "properties": {
///            "code_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_global_contract_code"
///            }
///          }
///        },
///        {
///          "title": "ViewGlobalContractCodeByAccountId",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_global_contract_code_by_account_id"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "BlockId",
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "Finality",
///        "type": "object",
///        "required": [
///          "finality"
///        ],
///        "properties": {
///          "finality": {
///            "$ref": "#/definitions/Finality"
///          }
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "SyncCheckpoint",
///        "type": "object",
///        "required": [
///          "sync_checkpoint"
///        ],
///        "properties": {
///          "sync_checkpoint": {
///            "$ref": "#/definitions/SyncCheckpoint"
///          }
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcQueryRequestVariant0 {
    Variant0 {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ::std::string::String,
    },
    Variant1 {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ::std::string::String,
    },
    Variant2 {
        account_id: AccountId,
        block_id: BlockId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        include_proof: ::std::option::Option<bool>,
        prefix_base64: StoreKey,
        request_type: ::std::string::String,
    },
    Variant3 {
        account_id: AccountId,
        block_id: BlockId,
        public_key: PublicKey,
        request_type: ::std::string::String,
    },
    Variant4 {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ::std::string::String,
    },
    Variant5 {
        account_id: AccountId,
        block_id: BlockId,
        public_key: PublicKey,
        request_type: ::std::string::String,
    },
    Variant6 {
        account_id: AccountId,
        args_base64: FunctionArgs,
        block_id: BlockId,
        method_name: ::std::string::String,
        request_type: ::std::string::String,
    },
    Variant7 {
        block_id: BlockId,
        code_hash: CryptoHash,
        request_type: ::std::string::String,
    },
    Variant8 {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ::std::string::String,
    },
}
///`RpcQueryRequestVariant1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "oneOf": [
///        {
///          "title": "ViewAccount",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_account"
///            }
///          }
///        },
///        {
///          "title": "ViewCode",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_code"
///            }
///          }
///        },
///        {
///          "title": "ViewState",
///          "type": "object",
///          "required": [
///            "account_id",
///            "prefix_base64",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "include_proof": {
///              "type": "boolean"
///            },
///            "prefix_base64": {
///              "$ref": "#/definitions/StoreKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_state"
///            }
///          }
///        },
///        {
///          "title": "ViewAccessKey",
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_access_key"
///            }
///          }
///        },
///        {
///          "title": "ViewAccessKeyList",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_access_key_list"
///            }
///          }
///        },
///        {
///          "title": "ViewGasKeyNonces",
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_gas_key_nonces"
///            }
///          }
///        },
///        {
///          "title": "CallFunction",
///          "type": "object",
///          "required": [
///            "account_id",
///            "args_base64",
///            "method_name",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "args_base64": {
///              "$ref": "#/definitions/FunctionArgs"
///            },
///            "method_name": {
///              "type": "string"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "call_function"
///            }
///          }
///        },
///        {
///          "title": "ViewGlobalContractCode",
///          "type": "object",
///          "required": [
///            "code_hash",
///            "request_type"
///          ],
///          "properties": {
///            "code_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_global_contract_code"
///            }
///          }
///        },
///        {
///          "title": "ViewGlobalContractCodeByAccountId",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_global_contract_code_by_account_id"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "Finality",
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/definitions/Finality"
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "BlockId",
///        "type": "object",
///        "required": [
///          "block_id"
///        ],
///        "properties": {
///          "block_id": {
///            "$ref": "#/definitions/BlockId"
///          }
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "SyncCheckpoint",
///        "type": "object",
///        "required": [
///          "sync_checkpoint"
///        ],
///        "properties": {
///          "sync_checkpoint": {
///            "$ref": "#/definitions/SyncCheckpoint"
///          }
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcQueryRequestVariant1 {
    Variant0 {
        account_id: AccountId,
        finality: Finality,
        request_type: ::std::string::String,
    },
    Variant1 {
        account_id: AccountId,
        finality: Finality,
        request_type: ::std::string::String,
    },
    Variant2 {
        account_id: AccountId,
        finality: Finality,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        include_proof: ::std::option::Option<bool>,
        prefix_base64: StoreKey,
        request_type: ::std::string::String,
    },
    Variant3 {
        account_id: AccountId,
        finality: Finality,
        public_key: PublicKey,
        request_type: ::std::string::String,
    },
    Variant4 {
        account_id: AccountId,
        finality: Finality,
        request_type: ::std::string::String,
    },
    Variant5 {
        account_id: AccountId,
        finality: Finality,
        public_key: PublicKey,
        request_type: ::std::string::String,
    },
    Variant6 {
        account_id: AccountId,
        args_base64: FunctionArgs,
        finality: Finality,
        method_name: ::std::string::String,
        request_type: ::std::string::String,
    },
    Variant7 {
        code_hash: CryptoHash,
        finality: Finality,
        request_type: ::std::string::String,
    },
    Variant8 {
        account_id: AccountId,
        finality: Finality,
        request_type: ::std::string::String,
    },
}
///`RpcQueryRequestVariant2`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "oneOf": [
///        {
///          "title": "ViewAccount",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_account"
///            }
///          }
///        },
///        {
///          "title": "ViewCode",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_code"
///            }
///          }
///        },
///        {
///          "title": "ViewState",
///          "type": "object",
///          "required": [
///            "account_id",
///            "prefix_base64",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "include_proof": {
///              "type": "boolean"
///            },
///            "prefix_base64": {
///              "$ref": "#/definitions/StoreKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_state"
///            }
///          }
///        },
///        {
///          "title": "ViewAccessKey",
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_access_key"
///            }
///          }
///        },
///        {
///          "title": "ViewAccessKeyList",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_access_key_list"
///            }
///          }
///        },
///        {
///          "title": "ViewGasKeyNonces",
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_gas_key_nonces"
///            }
///          }
///        },
///        {
///          "title": "CallFunction",
///          "type": "object",
///          "required": [
///            "account_id",
///            "args_base64",
///            "method_name",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "args_base64": {
///              "$ref": "#/definitions/FunctionArgs"
///            },
///            "method_name": {
///              "type": "string"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "call_function"
///            }
///          }
///        },
///        {
///          "title": "ViewGlobalContractCode",
///          "type": "object",
///          "required": [
///            "code_hash",
///            "request_type"
///          ],
///          "properties": {
///            "code_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_global_contract_code"
///            }
///          }
///        },
///        {
///          "title": "ViewGlobalContractCodeByAccountId",
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "const": "view_global_contract_code_by_account_id"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "SyncCheckpoint",
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/definitions/SyncCheckpoint"
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "BlockId",
///        "type": "object",
///        "required": [
///          "block_id"
///        ],
///        "properties": {
///          "block_id": {
///            "$ref": "#/definitions/BlockId"
///          }
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "Finality",
///        "type": "object",
///        "required": [
///          "finality"
///        ],
///        "properties": {
///          "finality": {
///            "$ref": "#/definitions/Finality"
///          }
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcQueryRequestVariant2 {
    Variant0 {
        account_id: AccountId,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant1 {
        account_id: AccountId,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant2 {
        account_id: AccountId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        include_proof: ::std::option::Option<bool>,
        prefix_base64: StoreKey,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant3 {
        account_id: AccountId,
        public_key: PublicKey,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant4 {
        account_id: AccountId,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant5 {
        account_id: AccountId,
        public_key: PublicKey,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant6 {
        account_id: AccountId,
        args_base64: FunctionArgs,
        method_name: ::std::string::String,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant7 {
        code_hash: CryptoHash,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant8 {
        account_id: AccountId,
        request_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
}
///`RpcQueryResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcQueryResponse",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "AccountView",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        {
///          "$ref": "#/definitions/AccountView"
///        }
///      ]
///    },
///    {
///      "title": "ContractCodeView",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        {
///          "$ref": "#/definitions/ContractCodeView"
///        }
///      ]
///    },
///    {
///      "title": "ViewStateResult",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        {
///          "$ref": "#/definitions/ViewStateResult"
///        }
///      ]
///    },
///    {
///      "title": "CallResult",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        {
///          "$ref": "#/definitions/CallResult"
///        }
///      ]
///    },
///    {
///      "title": "AccessKeyView",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        {
///          "$ref": "#/definitions/AccessKeyView"
///        }
///      ]
///    },
///    {
///      "title": "AccessKeyList",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        {
///          "$ref": "#/definitions/AccessKeyList"
///        }
///      ]
///    },
///    {
///      "title": "BlockHeightBlockHash",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        {
///          "type": "array",
///          "items": {
///            "type": "integer",
///            "format": "uint64",
///            "minimum": 0.0
///          }
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcQueryResponse {
    AccountView {
        amount: NearToken,
        block_hash: CryptoHash,
        block_height: u64,
        code_hash: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        global_contract_account_id: ::std::option::Option<AccountId>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        global_contract_hash: ::std::option::Option<CryptoHash>,
        locked: NearToken,
        ///TODO(2271): deprecated.
        #[serde(default)]
        storage_paid_at: u64,
        storage_usage: u64,
    },
    ContractCodeView {
        block_hash: CryptoHash,
        block_height: u64,
        code_base64: ::std::string::String,
        hash: CryptoHash,
    },
    ViewStateResult {
        block_hash: CryptoHash,
        block_height: u64,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        proof: ::std::vec::Vec<::std::string::String>,
        values: ::std::vec::Vec<StateItem>,
    },
    CallResult {
        block_hash: CryptoHash,
        block_height: u64,
        logs: ::std::vec::Vec<::std::string::String>,
        result: ::std::vec::Vec<u8>,
    },
    AccessKeyView {
        block_hash: CryptoHash,
        block_height: u64,
        nonce: u64,
        permission: AccessKeyPermissionView,
    },
    AccessKeyList {
        block_hash: CryptoHash,
        block_height: u64,
        keys: ::std::vec::Vec<AccessKeyInfoView>,
    },
    BlockHeightBlockHash(RpcQueryResponseBlockHeightBlockHash),
}
impl ::std::convert::From<RpcQueryResponseBlockHeightBlockHash> for RpcQueryResponse {
    fn from(value: RpcQueryResponseBlockHeightBlockHash) -> Self {
        Self::BlockHeightBlockHash(value)
    }
}
///`RpcQueryResponseBlockHeightBlockHash`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "BlockHeightBlockHash",
///  "allOf": [
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    },
///    {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(deny_unknown_fields)]
pub enum RpcQueryResponseBlockHeightBlockHash {}
///`RpcReceiptRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcReceiptRequest",
///  "type": "object",
///  "required": [
///    "receipt_id"
///  ],
///  "properties": {
///    "receipt_id": {
///      "$ref": "#/definitions/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcReceiptRequest {
    pub receipt_id: CryptoHash,
}
///`RpcReceiptResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcReceiptResponse",
///  "type": "object",
///  "required": [
///    "predecessor_id",
///    "receipt",
///    "receipt_id",
///    "receiver_id"
///  ],
///  "properties": {
///    "predecessor_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "priority": {
///      "description": "Deprecated, retained for backward compatibility.",
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "receipt": {
///      "$ref": "#/definitions/ReceiptEnumView"
///    },
///    "receipt_id": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "receiver_id": {
///      "$ref": "#/definitions/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcReceiptResponse {
    pub predecessor_id: AccountId,
    ///Deprecated, retained for backward compatibility.
    #[serde(default)]
    pub priority: u64,
    pub receipt: ReceiptEnumView,
    pub receipt_id: CryptoHash,
    pub receiver_id: AccountId,
}
///`RpcSendTransactionRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcSendTransactionRequest",
///  "type": "object",
///  "required": [
///    "signed_tx_base64"
///  ],
///  "properties": {
///    "signed_tx_base64": {
///      "$ref": "#/definitions/SignedTransaction"
///    },
///    "wait_until": {
///      "default": "EXECUTED_OPTIMISTIC",
///      "allOf": [
///        {
///          "$ref": "#/definitions/TxExecutionStatus"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcSendTransactionRequest {
    pub signed_tx_base64: SignedTransaction,
    #[serde(default = "defaults::rpc_send_transaction_request_wait_until")]
    pub wait_until: TxExecutionStatus,
}
///`RpcSplitStorageInfoRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcSplitStorageInfoRequest",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcSplitStorageInfoRequest(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for RpcSplitStorageInfoRequest {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<RpcSplitStorageInfoRequest>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: RpcSplitStorageInfoRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for RpcSplitStorageInfoRequest {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///Contains the split storage information.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcSplitStorageInfoResponse",
///  "description": "Contains the split storage information.",
///  "type": "object",
///  "properties": {
///    "cold_head_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "final_head_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "head_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "hot_db_kind": {
///      "type": [
///        "string",
///        "null"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcSplitStorageInfoResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cold_head_height: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub final_head_height: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub head_height: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hot_db_kind: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for RpcSplitStorageInfoResponse {
    fn default() -> Self {
        Self {
            cold_head_height: Default::default(),
            final_head_height: Default::default(),
            head_height: Default::default(),
            hot_db_kind: Default::default(),
        }
    }
}
/**It is a [serializable view] of [`StateChangesRequest`].

[serializable view]: ./index.html
[`StateChangesRequest`]: ../types/struct.StateChangesRequest.html*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStateChangesInBlockByTypeRequest",
///  "description": "It is a [serializable view] of [`StateChangesRequest`].\n\n[serializable view]: ./index.html\n[`StateChangesRequest`]: ../types/struct.StateChangesRequest.html",
///  "type": "object",
///  "allOf": [
///    {
///      "$ref": "#/definitions/BlockReference"
///    },
///    {
///      "$ref": "#/definitions/StateChangesRequestView"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcStateChangesInBlockByTypeRequest {
    Variant0(RpcStateChangesInBlockByTypeRequestVariant0),
    Variant1(RpcStateChangesInBlockByTypeRequestVariant1),
    Variant2(RpcStateChangesInBlockByTypeRequestVariant2),
}
impl ::std::convert::From<RpcStateChangesInBlockByTypeRequestVariant0>
for RpcStateChangesInBlockByTypeRequest {
    fn from(value: RpcStateChangesInBlockByTypeRequestVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<RpcStateChangesInBlockByTypeRequestVariant1>
for RpcStateChangesInBlockByTypeRequest {
    fn from(value: RpcStateChangesInBlockByTypeRequestVariant1) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<RpcStateChangesInBlockByTypeRequestVariant2>
for RpcStateChangesInBlockByTypeRequest {
    fn from(value: RpcStateChangesInBlockByTypeRequestVariant2) -> Self {
        Self::Variant2(value)
    }
}
///`RpcStateChangesInBlockByTypeRequestVariant0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "oneOf": [
///        {
///          "title": "AccountChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "account_changes"
///            }
///          }
///        },
///        {
///          "title": "SingleAccessKeyChanges",
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "const": "single_access_key_changes"
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountWithPublicKey"
///              }
///            }
///          }
///        },
///        {
///          "title": "AllAccessKeyChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "all_access_key_changes"
///            }
///          }
///        },
///        {
///          "title": "ContractCodeChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "contract_code_changes"
///            }
///          }
///        },
///        {
///          "title": "DataChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type",
///            "key_prefix_base64"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "data_changes"
///            },
///            "key_prefix_base64": {
///              "$ref": "#/definitions/StoreKey"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "BlockId",
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "Finality",
///        "type": "object",
///        "required": [
///          "finality"
///        ],
///        "properties": {
///          "finality": {
///            "$ref": "#/definitions/Finality"
///          }
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "SyncCheckpoint",
///        "type": "object",
///        "required": [
///          "sync_checkpoint"
///        ],
///        "properties": {
///          "sync_checkpoint": {
///            "$ref": "#/definitions/SyncCheckpoint"
///          }
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcStateChangesInBlockByTypeRequestVariant0 {
    Variant0 {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: ::std::string::String,
    },
    Variant1 {
        block_id: BlockId,
        changes_type: ::std::string::String,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
    },
    Variant2 {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: ::std::string::String,
    },
    Variant3 {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: ::std::string::String,
    },
    Variant4 {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: ::std::string::String,
        key_prefix_base64: StoreKey,
    },
}
///`RpcStateChangesInBlockByTypeRequestVariant1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "oneOf": [
///        {
///          "title": "AccountChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "account_changes"
///            }
///          }
///        },
///        {
///          "title": "SingleAccessKeyChanges",
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "const": "single_access_key_changes"
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountWithPublicKey"
///              }
///            }
///          }
///        },
///        {
///          "title": "AllAccessKeyChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "all_access_key_changes"
///            }
///          }
///        },
///        {
///          "title": "ContractCodeChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "contract_code_changes"
///            }
///          }
///        },
///        {
///          "title": "DataChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type",
///            "key_prefix_base64"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "data_changes"
///            },
///            "key_prefix_base64": {
///              "$ref": "#/definitions/StoreKey"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "Finality",
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/definitions/Finality"
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "BlockId",
///        "type": "object",
///        "required": [
///          "block_id"
///        ],
///        "properties": {
///          "block_id": {
///            "$ref": "#/definitions/BlockId"
///          }
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "SyncCheckpoint",
///        "type": "object",
///        "required": [
///          "sync_checkpoint"
///        ],
///        "properties": {
///          "sync_checkpoint": {
///            "$ref": "#/definitions/SyncCheckpoint"
///          }
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcStateChangesInBlockByTypeRequestVariant1 {
    Variant0 {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ::std::string::String,
        finality: Finality,
    },
    Variant1 {
        changes_type: ::std::string::String,
        finality: Finality,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
    },
    Variant2 {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ::std::string::String,
        finality: Finality,
    },
    Variant3 {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ::std::string::String,
        finality: Finality,
    },
    Variant4 {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ::std::string::String,
        finality: Finality,
        key_prefix_base64: StoreKey,
    },
}
///`RpcStateChangesInBlockByTypeRequestVariant2`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "oneOf": [
///        {
///          "title": "AccountChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "account_changes"
///            }
///          }
///        },
///        {
///          "title": "SingleAccessKeyChanges",
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "const": "single_access_key_changes"
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountWithPublicKey"
///              }
///            }
///          }
///        },
///        {
///          "title": "AllAccessKeyChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "all_access_key_changes"
///            }
///          }
///        },
///        {
///          "title": "ContractCodeChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "contract_code_changes"
///            }
///          }
///        },
///        {
///          "title": "DataChanges",
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type",
///            "key_prefix_base64"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/definitions/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "const": "data_changes"
///            },
///            "key_prefix_base64": {
///              "$ref": "#/definitions/StoreKey"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "SyncCheckpoint",
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/definitions/SyncCheckpoint"
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "BlockId",
///        "type": "object",
///        "required": [
///          "block_id"
///        ],
///        "properties": {
///          "block_id": {
///            "$ref": "#/definitions/BlockId"
///          }
///        }
///      }
///    },
///    {
///      "not": {
///        "title": "Finality",
///        "type": "object",
///        "required": [
///          "finality"
///        ],
///        "properties": {
///          "finality": {
///            "$ref": "#/definitions/Finality"
///          }
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcStateChangesInBlockByTypeRequestVariant2 {
    Variant0 {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant1 {
        changes_type: ::std::string::String,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant2 {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant3 {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ::std::string::String,
        sync_checkpoint: SyncCheckpoint,
    },
    Variant4 {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ::std::string::String,
        key_prefix_base64: StoreKey,
        sync_checkpoint: SyncCheckpoint,
    },
}
///`RpcStateChangesInBlockByTypeResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStateChangesInBlockByTypeResponse",
///  "type": "object",
///  "required": [
///    "block_hash",
///    "changes"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "changes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/StateChangeKindView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcStateChangesInBlockByTypeResponse {
    pub block_hash: CryptoHash,
    pub changes: ::std::vec::Vec<StateChangeKindView>,
}
///`RpcStateChangesInBlockRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStateChangesInBlockRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "BlockId",
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        }
///      }
///    },
///    {
///      "title": "Finality",
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/definitions/Finality"
///        }
///      }
///    },
///    {
///      "title": "SyncCheckpoint",
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/definitions/SyncCheckpoint"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum RpcStateChangesInBlockRequest {
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "finality")]
    Finality(Finality),
    #[serde(rename = "sync_checkpoint")]
    SyncCheckpoint(SyncCheckpoint),
}
impl ::std::convert::From<BlockId> for RpcStateChangesInBlockRequest {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
impl ::std::convert::From<Finality> for RpcStateChangesInBlockRequest {
    fn from(value: Finality) -> Self {
        Self::Finality(value)
    }
}
impl ::std::convert::From<SyncCheckpoint> for RpcStateChangesInBlockRequest {
    fn from(value: SyncCheckpoint) -> Self {
        Self::SyncCheckpoint(value)
    }
}
///`RpcStateChangesInBlockResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStateChangesInBlockResponse",
///  "type": "object",
///  "required": [
///    "block_hash",
///    "changes"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "changes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/StateChangeWithCauseView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcStateChangesInBlockResponse {
    pub block_hash: CryptoHash,
    pub changes: ::std::vec::Vec<StateChangeWithCauseView>,
}
///`RpcStatusRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStatusRequest",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcStatusRequest(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for RpcStatusRequest {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<RpcStatusRequest>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: RpcStatusRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for RpcStatusRequest {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`RpcStatusResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStatusResponse",
///  "type": "object",
///  "required": [
///    "chain_id",
///    "genesis_hash",
///    "latest_protocol_version",
///    "node_public_key",
///    "protocol_version",
///    "sync_info",
///    "uptime_sec",
///    "validators",
///    "version"
///  ],
///  "properties": {
///    "chain_id": {
///      "description": "Unique chain id.",
///      "type": "string"
///    },
///    "detailed_debug_status": {
///      "description": "Information about last blocks, network, epoch and chain & chunk info.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/DetailedDebugStatus"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "genesis_hash": {
///      "description": "Genesis hash of the chain.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        }
///      ]
///    },
///    "latest_protocol_version": {
///      "description": "Latest protocol version that this client supports.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "node_key": {
///      "description": "Deprecated; same as `validator_public_key` which you should use instead.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "node_public_key": {
///      "description": "Public key of the node.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    },
///    "protocol_version": {
///      "description": "Currently active protocol version.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "rpc_addr": {
///      "description": "Address for RPC server.  None if node doesn't have RPC endpoint enabled.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "sync_info": {
///      "description": "Sync status of the node.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/StatusSyncInfo"
///        }
///      ]
///    },
///    "uptime_sec": {
///      "description": "Uptime of the node.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "validator_account_id": {
///      "description": "Validator id of the node",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "validator_public_key": {
///      "description": "Public key of the validator.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "validators": {
///      "description": "Current epoch validators.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ValidatorInfo"
///      }
///    },
///    "version": {
///      "description": "Binary version.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Version"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcStatusResponse {
    ///Unique chain id.
    pub chain_id: ::std::string::String,
    ///Information about last blocks, network, epoch and chain & chunk info.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub detailed_debug_status: ::std::option::Option<DetailedDebugStatus>,
    ///Genesis hash of the chain.
    pub genesis_hash: CryptoHash,
    ///Latest protocol version that this client supports.
    pub latest_protocol_version: u32,
    ///Deprecated; same as `validator_public_key` which you should use instead.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub node_key: ::std::option::Option<PublicKey>,
    ///Public key of the node.
    pub node_public_key: PublicKey,
    ///Currently active protocol version.
    pub protocol_version: u32,
    ///Address for RPC server.  None if node doesn't have RPC endpoint enabled.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rpc_addr: ::std::option::Option<::std::string::String>,
    ///Sync status of the node.
    pub sync_info: StatusSyncInfo,
    ///Uptime of the node.
    pub uptime_sec: i64,
    ///Validator id of the node
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub validator_account_id: ::std::option::Option<AccountId>,
    ///Public key of the validator.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub validator_public_key: ::std::option::Option<PublicKey>,
    ///Current epoch validators.
    pub validators: ::std::vec::Vec<ValidatorInfo>,
    ///Binary version.
    pub version: Version,
}
///`RpcTransactionResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcTransactionResponse",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "FinalExecutionOutcomeWithReceiptView",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "final_execution_status"
///          ],
///          "properties": {
///            "final_execution_status": {
///              "$ref": "#/definitions/TxExecutionStatus"
///            }
///          }
///        },
///        {
///          "$ref": "#/definitions/FinalExecutionOutcomeWithReceiptView"
///        }
///      ]
///    },
///    {
///      "title": "FinalExecutionOutcomeView",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "final_execution_status"
///          ],
///          "properties": {
///            "final_execution_status": {
///              "$ref": "#/definitions/TxExecutionStatus"
///            }
///          }
///        },
///        {
///          "$ref": "#/definitions/FinalExecutionOutcomeView"
///        }
///      ]
///    },
///    {
///      "title": "Empty",
///      "type": "object",
///      "required": [
///        "final_execution_status"
///      ],
///      "properties": {
///        "final_execution_status": {
///          "$ref": "#/definitions/TxExecutionStatus"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcTransactionResponse {
    FinalExecutionOutcomeWithReceiptView {
        final_execution_status: TxExecutionStatus,
        ///Receipts generated from the transaction
        receipts: ::std::vec::Vec<ReceiptView>,
        ///The execution outcome of receipts.
        receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
        /**Execution status defined by chain.rs:get_final_transaction_result
FinalExecutionStatus::NotStarted - the tx is not converted to the receipt yet
FinalExecutionStatus::Started - we have at least 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished the execution
FinalExecutionStatus::Failure - the result of the first leaf receipt_id
FinalExecutionStatus::SuccessValue - the result of the first leaf receipt_id*/
        status: FinalExecutionStatus,
        ///Signed Transaction
        transaction: SignedTransactionView,
        ///The execution outcome of the signed transaction.
        transaction_outcome: ExecutionOutcomeWithIdView,
    },
    FinalExecutionOutcomeView {
        final_execution_status: TxExecutionStatus,
        ///The execution outcome of receipts.
        receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
        /**Execution status defined by chain.rs:get_final_transaction_result
FinalExecutionStatus::NotStarted - the tx is not converted to the receipt yet
FinalExecutionStatus::Started - we have at least 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished the execution
FinalExecutionStatus::Failure - the result of the first leaf receipt_id
FinalExecutionStatus::SuccessValue - the result of the first leaf receipt_id*/
        status: FinalExecutionStatus,
        ///Signed Transaction
        transaction: SignedTransactionView,
        ///The execution outcome of the signed transaction.
        transaction_outcome: ExecutionOutcomeWithIdView,
    },
    Empty { final_execution_status: TxExecutionStatus },
}
///`RpcTransactionStatusRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcTransactionStatusRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "SignedTxBase64",
///      "allOf": [
///        {
///          "type": "object",
///          "properties": {
///            "wait_until": {
///              "default": "EXECUTED_OPTIMISTIC",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/TxExecutionStatus"
///                }
///              ]
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "signed_tx_base64"
///          ],
///          "properties": {
///            "signed_tx_base64": {
///              "$ref": "#/definitions/SignedTransaction"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "TxHashSenderAccountId",
///      "allOf": [
///        {
///          "type": "object",
///          "properties": {
///            "wait_until": {
///              "default": "EXECUTED_OPTIMISTIC",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/TxExecutionStatus"
///                }
///              ]
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "sender_account_id",
///            "tx_hash"
///          ],
///          "properties": {
///            "sender_account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "tx_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            }
///          }
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcTransactionStatusRequest {
    SignedTxBase64 {
        signed_tx_base64: SignedTransaction,
        #[serde(
            default = "defaults::rpc_transaction_status_request_signed_tx_base64_wait_until"
        )]
        wait_until: TxExecutionStatus,
    },
    TxHashSenderAccountId {
        sender_account_id: AccountId,
        tx_hash: CryptoHash,
        #[serde(
            default = "defaults::rpc_transaction_status_request_tx_hash_sender_account_id_wait_until"
        )]
        wait_until: TxExecutionStatus,
    },
}
///`RpcValidatorRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcValidatorRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "EpochId",
///      "type": "object",
///      "required": [
///        "epoch_id"
///      ],
///      "properties": {
///        "epoch_id": {
///          "$ref": "#/definitions/EpochId"
///        }
///      }
///    },
///    {
///      "title": "BlockId",
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/definitions/BlockId"
///        }
///      }
///    },
///    {
///      "title": "Latest",
///      "type": "object",
///      "required": [
///        "latest"
///      ],
///      "properties": {
///        "latest": {
///          "type": "null"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum RpcValidatorRequest {
    #[serde(rename = "epoch_id")]
    EpochId(EpochId),
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "latest")]
    Latest,
}
impl ::std::convert::From<EpochId> for RpcValidatorRequest {
    fn from(value: EpochId) -> Self {
        Self::EpochId(value)
    }
}
impl ::std::convert::From<BlockId> for RpcValidatorRequest {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
///Information about this epoch validators and next epoch validators
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcValidatorResponse",
///  "description": "Information about this epoch validators and next epoch validators",
///  "type": "object",
///  "required": [
///    "current_fishermen",
///    "current_proposals",
///    "current_validators",
///    "epoch_height",
///    "epoch_start_height",
///    "next_fishermen",
///    "next_validators",
///    "prev_epoch_kickout"
///  ],
///  "properties": {
///    "current_fishermen": {
///      "description": "Fishermen for the current epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ValidatorStakeView"
///      }
///    },
///    "current_proposals": {
///      "description": "Proposals in the current epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ValidatorStakeView"
///      }
///    },
///    "current_validators": {
///      "description": "Validators for the current epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/CurrentEpochValidatorInfo"
///      }
///    },
///    "epoch_height": {
///      "description": "Epoch height",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "epoch_start_height": {
///      "description": "Epoch start block height",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "next_fishermen": {
///      "description": "Fishermen for the next epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ValidatorStakeView"
///      }
///    },
///    "next_validators": {
///      "description": "Validators for the next epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/NextEpochValidatorInfo"
///      }
///    },
///    "prev_epoch_kickout": {
///      "description": "Kickout in the previous epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ValidatorKickoutView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcValidatorResponse {
    ///Fishermen for the current epoch
    pub current_fishermen: ::std::vec::Vec<ValidatorStakeView>,
    ///Proposals in the current epoch
    pub current_proposals: ::std::vec::Vec<ValidatorStakeView>,
    ///Validators for the current epoch
    pub current_validators: ::std::vec::Vec<CurrentEpochValidatorInfo>,
    ///Epoch height
    pub epoch_height: u64,
    ///Epoch start block height
    pub epoch_start_height: u64,
    ///Fishermen for the next epoch
    pub next_fishermen: ::std::vec::Vec<ValidatorStakeView>,
    ///Validators for the next epoch
    pub next_validators: ::std::vec::Vec<NextEpochValidatorInfo>,
    ///Kickout in the previous epoch
    pub prev_epoch_kickout: ::std::vec::Vec<ValidatorKickoutView>,
}
///`RpcValidatorsOrderedRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcValidatorsOrderedRequest",
///  "type": "object",
///  "properties": {
///    "block_id": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/BlockId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcValidatorsOrderedRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_id: ::std::option::Option<BlockId>,
}
impl ::std::default::Default for RpcValidatorsOrderedRequest {
    fn default() -> Self {
        Self {
            block_id: Default::default(),
        }
    }
}
///View that preserves JSON format of the runtime config.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeConfigView",
///  "description": "View that preserves JSON format of the runtime config.",
///  "type": "object",
///  "required": [
///    "account_creation_config",
///    "congestion_control_config",
///    "storage_amount_per_byte",
///    "transaction_costs",
///    "wasm_config",
///    "witness_config"
///  ],
///  "properties": {
///    "account_creation_config": {
///      "description": "Config that defines rules for account creation.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/AccountCreationConfigView"
///        }
///      ]
///    },
///    "congestion_control_config": {
///      "description": "The configuration for congestion control.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/CongestionControlConfigView"
///        }
///      ]
///    },
///    "storage_amount_per_byte": {
///      "description": "Amount of yN per byte required to have on the account.  See\n<https://nomicon.io/Economics/Economics.html#state-stake> for details.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "transaction_costs": {
///      "description": "Costs of different actions that need to be performed when sending and\nprocessing transaction and receipts.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/RuntimeFeesConfigView"
///        }
///      ]
///    },
///    "wasm_config": {
///      "description": "Config of wasm operations.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/VMConfigView"
///        }
///      ]
///    },
///    "witness_config": {
///      "description": "Configuration specific to ChunkStateWitness.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/WitnessConfigView"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RuntimeConfigView {
    ///Config that defines rules for account creation.
    pub account_creation_config: AccountCreationConfigView,
    ///The configuration for congestion control.
    pub congestion_control_config: CongestionControlConfigView,
    /**Amount of yN per byte required to have on the account.  See
<https://nomicon.io/Economics/Economics.html#state-stake> for details.*/
    pub storage_amount_per_byte: NearToken,
    /**Costs of different actions that need to be performed when sending and
processing transaction and receipts.*/
    pub transaction_costs: RuntimeFeesConfigView,
    ///Config of wasm operations.
    pub wasm_config: VmConfigView,
    ///Configuration specific to ChunkStateWitness.
    pub witness_config: WitnessConfigView,
}
///Describes different fees for the runtime
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeFeesConfigView",
///  "description": "Describes different fees for the runtime",
///  "type": "object",
///  "required": [
///    "action_creation_config",
///    "action_receipt_creation_config",
///    "burnt_gas_reward",
///    "data_receipt_creation_config",
///    "pessimistic_gas_price_inflation_ratio",
///    "storage_usage_config"
///  ],
///  "properties": {
///    "action_creation_config": {
///      "description": "Describes the cost of creating a certain action, `Action`. Includes all variants.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ActionCreationConfigView"
///        }
///      ]
///    },
///    "action_receipt_creation_config": {
///      "description": "Describes the cost of creating an action receipt, `ActionReceipt`, excluding the actual cost\nof actions.\n- `send` cost is burned when a receipt is created using `promise_create` or\n    `promise_batch_create`\n- `exec` cost is burned when the receipt is being executed.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Fee"
///        }
///      ]
///    },
///    "burnt_gas_reward": {
///      "description": "Fraction of the burnt gas to reward to the contract account for execution.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "data_receipt_creation_config": {
///      "description": "Describes the cost of creating a data receipt, `DataReceipt`.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/DataReceiptCreationConfigView"
///        }
///      ]
///    },
///    "pessimistic_gas_price_inflation_ratio": {
///      "description": "Pessimistic gas price inflation ratio.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "storage_usage_config": {
///      "description": "Describes fees for storage.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/StorageUsageConfigView"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RuntimeFeesConfigView {
    ///Describes the cost of creating a certain action, `Action`. Includes all variants.
    pub action_creation_config: ActionCreationConfigView,
    /**Describes the cost of creating an action receipt, `ActionReceipt`, excluding the actual cost
of actions.
- `send` cost is burned when a receipt is created using `promise_create` or
    `promise_batch_create`
- `exec` cost is burned when the receipt is being executed.*/
    pub action_receipt_creation_config: Fee,
    ///Fraction of the burnt gas to reward to the contract account for execution.
    pub burnt_gas_reward: [i32; 2usize],
    ///Describes the cost of creating a data receipt, `DataReceipt`.
    pub data_receipt_creation_config: DataReceiptCreationConfigView,
    ///Pessimistic gas price inflation ratio.
    pub pessimistic_gas_price_inflation_ratio: [i32; 2usize],
    ///Describes fees for storage.
    pub storage_usage_config: StorageUsageConfigView,
}
/**The shard identifier. It may be an arbitrary number - it does not need to be
a number in the range 0..NUM_SHARDS. The shard ids do not need to be
sequential or contiguous.

The shard id is wrapped in a new type to prevent the old pattern of using
indices in range 0..NUM_SHARDS and casting to ShardId. Once the transition
if fully complete it potentially may be simplified to a regular type alias.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ShardId",
///  "description": "The shard identifier. It may be an arbitrary number - it does not need to be\na number in the range 0..NUM_SHARDS. The shard ids do not need to be\nsequential or contiguous.\n\nThe shard id is wrapped in a new type to prevent the old pattern of using\nindices in range 0..NUM_SHARDS and casting to ShardId. Once the transition\nif fully complete it potentially may be simplified to a regular type alias.",
///  "type": "integer",
///  "format": "uint64",
///  "minimum": 0.0
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ShardId(pub u64);
impl ::std::ops::Deref for ShardId {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl ::std::convert::From<ShardId> for u64 {
    fn from(value: ShardId) -> Self {
        value.0
    }
}
impl ::std::convert::From<u64> for ShardId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ShardId {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ShardId {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ShardId {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for ShardId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
/**A versioned struct that contains all information needed to assign accounts to shards.

Because of re-sharding, the chain may use different shard layout to split shards at different
times. Currently, `ShardLayout` is stored as part of `EpochConfig`, which is generated each
epoch given the epoch protocol version. In mainnet/testnet, we use two shard layouts since
re-sharding has only happened once. It is stored as part of genesis config, see
default_simple_nightshade_shard_layout() Below is an overview for some important
functionalities of ShardLayout interface.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ShardLayout",
///  "description": "A versioned struct that contains all information needed to assign accounts to shards.\n\nBecause of re-sharding, the chain may use different shard layout to split shards at different\ntimes. Currently, `ShardLayout` is stored as part of `EpochConfig`, which is generated each\nepoch given the epoch protocol version. In mainnet/testnet, we use two shard layouts since\nre-sharding has only happened once. It is stored as part of genesis config, see\ndefault_simple_nightshade_shard_layout() Below is an overview for some important\nfunctionalities of ShardLayout interface.",
///  "oneOf": [
///    {
///      "title": "V0",
///      "type": "object",
///      "required": [
///        "V0"
///      ],
///      "properties": {
///        "V0": {
///          "$ref": "#/definitions/ShardLayoutV0"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "V1",
///      "type": "object",
///      "required": [
///        "V1"
///      ],
///      "properties": {
///        "V1": {
///          "$ref": "#/definitions/ShardLayoutV1"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "V2",
///      "type": "object",
///      "required": [
///        "V2"
///      ],
///      "properties": {
///        "V2": {
///          "$ref": "#/definitions/ShardLayoutV2"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "V3",
///      "type": "object",
///      "required": [
///        "V3"
///      ],
///      "properties": {
///        "V3": {
///          "$ref": "#/definitions/ShardLayoutV3"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ShardLayout {
    V0(ShardLayoutV0),
    V1(ShardLayoutV1),
    V2(ShardLayoutV2),
    V3(ShardLayoutV3),
}
impl ::std::convert::From<ShardLayoutV0> for ShardLayout {
    fn from(value: ShardLayoutV0) -> Self {
        Self::V0(value)
    }
}
impl ::std::convert::From<ShardLayoutV1> for ShardLayout {
    fn from(value: ShardLayoutV1) -> Self {
        Self::V1(value)
    }
}
impl ::std::convert::From<ShardLayoutV2> for ShardLayout {
    fn from(value: ShardLayoutV2) -> Self {
        Self::V2(value)
    }
}
impl ::std::convert::From<ShardLayoutV3> for ShardLayout {
    fn from(value: ShardLayoutV3) -> Self {
        Self::V3(value)
    }
}
/**A shard layout that maps accounts evenly across all shards -- by calculate the hash of account
id and mod number of shards. This is added to capture the old `account_id_to_shard_id` algorithm,
to keep backward compatibility for some existing tests.
`parent_shards` for `ShardLayoutV1` is always `None`, meaning it can only be the first shard layout
a chain uses.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ShardLayoutV0",
///  "description": "A shard layout that maps accounts evenly across all shards -- by calculate the hash of account\nid and mod number of shards. This is added to capture the old `account_id_to_shard_id` algorithm,\nto keep backward compatibility for some existing tests.\n`parent_shards` for `ShardLayoutV1` is always `None`, meaning it can only be the first shard layout\na chain uses.",
///  "type": "object",
///  "required": [
///    "num_shards",
///    "version"
///  ],
///  "properties": {
///    "num_shards": {
///      "description": "Map accounts evenly across all shards",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "version": {
///      "description": "Version of the shard layout, this is useful for uniquely identify the shard layout",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardLayoutV0 {
    ///Map accounts evenly across all shards
    pub num_shards: u64,
    ///Version of the shard layout, this is useful for uniquely identify the shard layout
    pub version: u32,
}
///`ShardLayoutV1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ShardLayoutV1",
///  "type": "object",
///  "required": [
///    "boundary_accounts",
///    "version"
///  ],
///  "properties": {
///    "boundary_accounts": {
///      "description": "The boundary accounts are the accounts on boundaries between shards.\nEach shard contains a range of accounts from one boundary account to\nanother - or the smallest or largest account possible. The total\nnumber of shards is equal to the number of boundary accounts plus 1.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AccountId"
///      }
///    },
///    "shards_split_map": {
///      "description": "Maps shards from the last shard layout to shards that it splits to in this shard layout,\nUseful for constructing states for the shards.\nNone for the genesis shard layout",
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "array",
///        "items": {
///          "$ref": "#/definitions/ShardId"
///        }
///      }
///    },
///    "to_parent_shard_map": {
///      "description": "Maps shard in this shard layout to their parent shard\nSince shard_ids always range from 0 to num_shards - 1, we use vec instead of a hashmap",
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/definitions/ShardId"
///      }
///    },
///    "version": {
///      "description": "Version of the shard layout, this is useful for uniquely identify the shard layout",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardLayoutV1 {
    /**The boundary accounts are the accounts on boundaries between shards.
Each shard contains a range of accounts from one boundary account to
another - or the smallest or largest account possible. The total
number of shards is equal to the number of boundary accounts plus 1.*/
    pub boundary_accounts: ::std::vec::Vec<AccountId>,
    /**Maps shards from the last shard layout to shards that it splits to in this shard layout,
Useful for constructing states for the shards.
None for the genesis shard layout*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shards_split_map: ::std::option::Option<
        ::std::vec::Vec<::std::vec::Vec<ShardId>>,
    >,
    /**Maps shard in this shard layout to their parent shard
Since shard_ids always range from 0 to num_shards - 1, we use vec instead of a hashmap*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub to_parent_shard_map: ::std::option::Option<::std::vec::Vec<ShardId>>,
    ///Version of the shard layout, this is useful for uniquely identify the shard layout
    pub version: u32,
}
/**Counterpart to `ShardLayoutV2` composed of maps with string keys to aid
serde serialization.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ShardLayoutV2",
///  "description": "Counterpart to `ShardLayoutV2` composed of maps with string keys to aid\nserde serialization.",
///  "type": "object",
///  "required": [
///    "boundary_accounts",
///    "id_to_index_map",
///    "index_to_id_map",
///    "shard_ids",
///    "version"
///  ],
///  "properties": {
///    "boundary_accounts": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AccountId"
///      }
///    },
///    "id_to_index_map": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "uint",
///        "minimum": 0.0
///      }
///    },
///    "index_to_id_map": {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/definitions/ShardId"
///      }
///    },
///    "shard_ids": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ShardId"
///      }
///    },
///    "shards_parent_map": {
///      "type": [
///        "object",
///        "null"
///      ],
///      "additionalProperties": {
///        "$ref": "#/definitions/ShardId"
///      }
///    },
///    "shards_split_map": {
///      "type": [
///        "object",
///        "null"
///      ],
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/definitions/ShardId"
///        }
///      }
///    },
///    "version": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardLayoutV2 {
    pub boundary_accounts: ::std::vec::Vec<AccountId>,
    pub id_to_index_map: ::std::collections::HashMap<::std::string::String, u32>,
    pub index_to_id_map: ::std::collections::HashMap<::std::string::String, ShardId>,
    pub shard_ids: ::std::vec::Vec<ShardId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shards_parent_map: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ShardId>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shards_split_map: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<ShardId>>,
    >,
    pub version: u32,
}
/**Counterpart to `ShardLayoutV3` composed of maps with string keys to aid
serde serialization.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ShardLayoutV3",
///  "description": "Counterpart to `ShardLayoutV3` composed of maps with string keys to aid\nserde serialization.",
///  "type": "object",
///  "required": [
///    "boundary_accounts",
///    "id_to_index_map",
///    "last_split",
///    "shard_ids",
///    "shards_split_map"
///  ],
///  "properties": {
///    "boundary_accounts": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AccountId"
///      }
///    },
///    "id_to_index_map": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "uint",
///        "minimum": 0.0
///      }
///    },
///    "last_split": {
///      "$ref": "#/definitions/ShardId"
///    },
///    "shard_ids": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ShardId"
///      }
///    },
///    "shards_split_map": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/definitions/ShardId"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardLayoutV3 {
    pub boundary_accounts: ::std::vec::Vec<AccountId>,
    pub id_to_index_map: ::std::collections::HashMap<::std::string::String, u32>,
    pub last_split: ShardId,
    pub shard_ids: ::std::vec::Vec<ShardId>,
    pub shards_split_map: ::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<ShardId>,
    >,
}
/**`ShardUId` is a unique representation for shards from different shard layouts.

Comparing to `ShardId`, which is just an ordinal number ranging from 0 to NUM_SHARDS-1,
`ShardUId` provides a way to unique identify shards when shard layouts may change across epochs.
This is important because we store states indexed by shards in our database, so we need a
way to unique identify shard even when shards change across epochs.
Another difference between `ShardUId` and `ShardId` is that `ShardUId` should only exist in
a node's internal state while `ShardId` can be exposed to outside APIs and used in protocol
level information (for example, `ShardChunkHeader` contains `ShardId` instead of `ShardUId`)*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ShardUId",
///  "description": "`ShardUId` is a unique representation for shards from different shard layouts.\n\nComparing to `ShardId`, which is just an ordinal number ranging from 0 to NUM_SHARDS-1,\n`ShardUId` provides a way to unique identify shards when shard layouts may change across epochs.\nThis is important because we store states indexed by shards in our database, so we need a\nway to unique identify shard even when shards change across epochs.\nAnother difference between `ShardUId` and `ShardId` is that `ShardUId` should only exist in\na node's internal state while `ShardId` can be exposed to outside APIs and used in protocol\nlevel information (for example, `ShardChunkHeader` contains `ShardId` instead of `ShardUId`)",
///  "type": "object",
///  "required": [
///    "shard_id",
///    "version"
///  ],
///  "properties": {
///    "shard_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "version": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardUId {
    pub shard_id: u32,
    pub version: u32,
}
///`Signature`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Signature",
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct Signature(pub ::std::string::String);
impl ::std::ops::Deref for Signature {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Signature> for ::std::string::String {
    fn from(value: Signature) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for Signature {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Signature {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`SignedDelegateAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "SignedDelegateAction",
///  "type": "object",
///  "required": [
///    "delegate_action",
///    "signature"
///  ],
///  "properties": {
///    "delegate_action": {
///      "$ref": "#/definitions/DelegateAction"
///    },
///    "signature": {
///      "$ref": "#/definitions/Signature"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SignedDelegateAction {
    pub delegate_action: DelegateAction,
    pub signature: Signature,
}
///`SignedTransaction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "SignedTransaction",
///  "type": "string",
///  "format": "byte"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct SignedTransaction(pub ::std::string::String);
impl ::std::ops::Deref for SignedTransaction {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SignedTransaction> for ::std::string::String {
    fn from(value: SignedTransaction) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SignedTransaction {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SignedTransaction {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`SignedTransactionView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "SignedTransactionView",
///  "type": "object",
///  "required": [
///    "actions",
///    "hash",
///    "nonce",
///    "public_key",
///    "receiver_id",
///    "signature",
///    "signer_id"
///  ],
///  "properties": {
///    "actions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ActionView"
///      }
///    },
///    "hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "nonce_index": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint16",
///      "maximum": 65535.0,
///      "minimum": 0.0
///    },
///    "priority_fee": {
///      "description": "Deprecated, retained for backward compatibility.",
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "public_key": {
///      "$ref": "#/definitions/PublicKey"
///    },
///    "receiver_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "signature": {
///      "$ref": "#/definitions/Signature"
///    },
///    "signer_id": {
///      "$ref": "#/definitions/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SignedTransactionView {
    pub actions: ::std::vec::Vec<ActionView>,
    pub hash: CryptoHash,
    pub nonce: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nonce_index: ::std::option::Option<u16>,
    ///Deprecated, retained for backward compatibility.
    #[serde(default)]
    pub priority_fee: u64,
    pub public_key: PublicKey,
    pub receiver_id: AccountId,
    pub signature: Signature,
    pub signer_id: AccountId,
}
///`SlashedValidator`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "SlashedValidator",
///  "type": "object",
///  "required": [
///    "account_id",
///    "is_double_sign"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "is_double_sign": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SlashedValidator {
    pub account_id: AccountId,
    pub is_double_sign: bool,
}
///An action which stakes signer_id tokens and setup's validator public key
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StakeAction",
///  "description": "An action which stakes signer_id tokens and setup's validator public key",
///  "type": "object",
///  "required": [
///    "public_key",
///    "stake"
///  ],
///  "properties": {
///    "public_key": {
///      "description": "Validator key which will be used to sign transactions on behalf of signer_id",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    },
///    "stake": {
///      "description": "Amount of tokens to stake.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StakeAction {
    ///Validator key which will be used to sign transactions on behalf of signer_id
    pub public_key: PublicKey,
    ///Amount of tokens to stake.
    pub stake: NearToken,
}
///See crate::types::StateChangeCause for details.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StateChangeCauseView",
///  "description": "See crate::types::StateChangeCause for details.",
///  "oneOf": [
///    {
///      "title": "NotWritableToDisk",
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "not_writable_to_disk"
///        }
///      }
///    },
///    {
///      "title": "InitialState",
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "initial_state"
///        }
///      }
///    },
///    {
///      "title": "TransactionProcessing",
///      "type": "object",
///      "required": [
///        "tx_hash",
///        "type"
///      ],
///      "properties": {
///        "tx_hash": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "const": "transaction_processing"
///        }
///      }
///    },
///    {
///      "title": "ActionReceiptProcessingStarted",
///      "type": "object",
///      "required": [
///        "receipt_hash",
///        "type"
///      ],
///      "properties": {
///        "receipt_hash": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "const": "action_receipt_processing_started"
///        }
///      }
///    },
///    {
///      "title": "ActionReceiptGasReward",
///      "type": "object",
///      "required": [
///        "receipt_hash",
///        "type"
///      ],
///      "properties": {
///        "receipt_hash": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "const": "action_receipt_gas_reward"
///        }
///      }
///    },
///    {
///      "title": "ReceiptProcessing",
///      "type": "object",
///      "required": [
///        "receipt_hash",
///        "type"
///      ],
///      "properties": {
///        "receipt_hash": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "const": "receipt_processing"
///        }
///      }
///    },
///    {
///      "title": "PostponedReceipt",
///      "type": "object",
///      "required": [
///        "receipt_hash",
///        "type"
///      ],
///      "properties": {
///        "receipt_hash": {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "const": "postponed_receipt"
///        }
///      }
///    },
///    {
///      "title": "UpdatedDelayedReceipts",
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "updated_delayed_receipts"
///        }
///      }
///    },
///    {
///      "title": "ValidatorAccountsUpdate",
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "validator_accounts_update"
///        }
///      }
///    },
///    {
///      "title": "Migration",
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "migration"
///        }
///      }
///    },
///    {
///      "title": "BandwidthSchedulerStateUpdate",
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "bandwidth_scheduler_state_update"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum StateChangeCauseView {
    #[serde(rename = "not_writable_to_disk")]
    NotWritableToDisk,
    #[serde(rename = "initial_state")]
    InitialState,
    ///TransactionProcessing
    #[serde(rename = "transaction_processing")]
    TransactionProcessing { tx_hash: CryptoHash },
    ///ActionReceiptProcessingStarted
    #[serde(rename = "action_receipt_processing_started")]
    ActionReceiptProcessingStarted { receipt_hash: CryptoHash },
    ///ActionReceiptGasReward
    #[serde(rename = "action_receipt_gas_reward")]
    ActionReceiptGasReward { receipt_hash: CryptoHash },
    ///ReceiptProcessing
    #[serde(rename = "receipt_processing")]
    ReceiptProcessing { receipt_hash: CryptoHash },
    ///PostponedReceipt
    #[serde(rename = "postponed_receipt")]
    PostponedReceipt { receipt_hash: CryptoHash },
    #[serde(rename = "updated_delayed_receipts")]
    UpdatedDelayedReceipts,
    #[serde(rename = "validator_accounts_update")]
    ValidatorAccountsUpdate,
    #[serde(rename = "migration")]
    Migration,
    #[serde(rename = "bandwidth_scheduler_state_update")]
    BandwidthSchedulerStateUpdate,
}
/**It is a [serializable view] of [`StateChangeKind`].

[serializable view]: ./index.html
[`StateChangeKind`]: ../types/struct.StateChangeKind.html*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StateChangeKindView",
///  "description": "It is a [serializable view] of [`StateChangeKind`].\n\n[serializable view]: ./index.html\n[`StateChangeKind`]: ../types/struct.StateChangeKind.html",
///  "oneOf": [
///    {
///      "title": "AccountTouched",
///      "type": "object",
///      "required": [
///        "account_id",
///        "type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "const": "account_touched"
///        }
///      }
///    },
///    {
///      "title": "AccessKeyTouched",
///      "type": "object",
///      "required": [
///        "account_id",
///        "type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "const": "access_key_touched"
///        }
///      }
///    },
///    {
///      "title": "DataTouched",
///      "type": "object",
///      "required": [
///        "account_id",
///        "type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "const": "data_touched"
///        }
///      }
///    },
///    {
///      "title": "ContractCodeTouched",
///      "type": "object",
///      "required": [
///        "account_id",
///        "type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/definitions/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "const": "contract_code_touched"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "account_id")]
pub enum StateChangeKindView {
    ///AccountTouched
    #[serde(rename = "account_touched")]
    AccountTouched(AccountId),
    ///AccessKeyTouched
    #[serde(rename = "access_key_touched")]
    AccessKeyTouched(AccountId),
    ///DataTouched
    #[serde(rename = "data_touched")]
    DataTouched(AccountId),
    ///ContractCodeTouched
    #[serde(rename = "contract_code_touched")]
    ContractCodeTouched(AccountId),
}
///`StateChangeWithCauseView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StateChangeWithCauseView",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "AccountUpdate",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "description": "A view of the account",
///          "type": "object",
///          "required": [
///            "account_id",
///            "amount",
///            "code_hash",
///            "locked",
///            "storage_usage"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "amount": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "code_hash": {
///              "$ref": "#/definitions/CryptoHash"
///            },
///            "global_contract_account_id": {
///              "oneOf": [
///                {
///                  "$ref": "#/definitions/AccountId"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "global_contract_hash": {
///              "oneOf": [
///                {
///                  "$ref": "#/definitions/CryptoHash"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "locked": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "storage_paid_at": {
///              "description": "TODO(2271): deprecated.",
///              "default": 0,
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "storage_usage": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "account_update"
///        }
///      }
///    },
///    {
///      "title": "AccountDeletion",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "account_deletion"
///        }
///      }
///    },
///    {
///      "title": "AccessKeyUpdate",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "access_key",
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "access_key": {
///              "$ref": "#/definitions/AccessKeyView"
///            },
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "access_key_update"
///        }
///      }
///    },
///    {
///      "title": "AccessKeyDeletion",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "access_key_deletion"
///        }
///      }
///    },
///    {
///      "title": "GasKeyNonceUpdate",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "index",
///            "nonce",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "index": {
///              "type": "integer",
///              "format": "uint16",
///              "maximum": 65535.0,
///              "minimum": 0.0
///            },
///            "nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "public_key": {
///              "$ref": "#/definitions/PublicKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "gas_key_nonce_update"
///        }
///      }
///    },
///    {
///      "title": "DataUpdate",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "key_base64",
///            "value_base64"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "key_base64": {
///              "$ref": "#/definitions/StoreKey"
///            },
///            "value_base64": {
///              "$ref": "#/definitions/StoreValue"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "data_update"
///        }
///      }
///    },
///    {
///      "title": "DataDeletion",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "key_base64"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "key_base64": {
///              "$ref": "#/definitions/StoreKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "data_deletion"
///        }
///      }
///    },
///    {
///      "title": "ContractCodeUpdate",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "code_base64"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            },
///            "code_base64": {
///              "type": "string"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "contract_code_update"
///        }
///      }
///    },
///    {
///      "title": "ContractCodeDeletion",
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/definitions/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/definitions/AccountId"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "const": "contract_code_deletion"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum StateChangeWithCauseView {
    ///AccountUpdate
    #[serde(rename = "account_update")]
    AccountUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    ///AccountDeletion
    #[serde(rename = "account_deletion")]
    AccountDeletion {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    ///AccessKeyUpdate
    #[serde(rename = "access_key_update")]
    AccessKeyUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    ///AccessKeyDeletion
    #[serde(rename = "access_key_deletion")]
    AccessKeyDeletion {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    ///GasKeyNonceUpdate
    #[serde(rename = "gas_key_nonce_update")]
    GasKeyNonceUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    ///DataUpdate
    #[serde(rename = "data_update")]
    DataUpdate { cause: StateChangeCauseView, change: StateChangeWithCauseViewChange },
    ///DataDeletion
    #[serde(rename = "data_deletion")]
    DataDeletion { cause: StateChangeCauseView, change: StateChangeWithCauseViewChange },
    ///ContractCodeUpdate
    #[serde(rename = "contract_code_update")]
    ContractCodeUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    ///ContractCodeDeletion
    #[serde(rename = "contract_code_deletion")]
    ContractCodeDeletion {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
}
///A view of the account
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A view of the account",
///  "type": "object",
///  "required": [
///    "account_id",
///    "amount",
///    "code_hash",
///    "locked",
///    "storage_usage"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "amount": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "code_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "global_contract_account_id": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "global_contract_hash": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "locked": {
///      "$ref": "#/definitions/NearToken"
///    },
///    "storage_paid_at": {
///      "description": "TODO(2271): deprecated.",
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "storage_usage": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StateChangeWithCauseViewChange {
    pub account_id: AccountId,
    pub amount: NearToken,
    pub code_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_account_id: ::std::option::Option<AccountId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_hash: ::std::option::Option<CryptoHash>,
    pub locked: NearToken,
    ///TODO(2271): deprecated.
    #[serde(default)]
    pub storage_paid_at: u64,
    pub storage_usage: u64,
}
///`StateChangesRequestView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StateChangesRequestView",
///  "oneOf": [
///    {
///      "title": "AccountChanges",
///      "type": "object",
///      "required": [
///        "account_ids",
///        "changes_type"
///      ],
///      "properties": {
///        "account_ids": {
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/AccountId"
///          }
///        },
///        "changes_type": {
///          "type": "string",
///          "const": "account_changes"
///        }
///      }
///    },
///    {
///      "title": "SingleAccessKeyChanges",
///      "type": "object",
///      "required": [
///        "changes_type",
///        "keys"
///      ],
///      "properties": {
///        "changes_type": {
///          "type": "string",
///          "const": "single_access_key_changes"
///        },
///        "keys": {
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/AccountWithPublicKey"
///          }
///        }
///      }
///    },
///    {
///      "title": "AllAccessKeyChanges",
///      "type": "object",
///      "required": [
///        "account_ids",
///        "changes_type"
///      ],
///      "properties": {
///        "account_ids": {
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/AccountId"
///          }
///        },
///        "changes_type": {
///          "type": "string",
///          "const": "all_access_key_changes"
///        }
///      }
///    },
///    {
///      "title": "ContractCodeChanges",
///      "type": "object",
///      "required": [
///        "account_ids",
///        "changes_type"
///      ],
///      "properties": {
///        "account_ids": {
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/AccountId"
///          }
///        },
///        "changes_type": {
///          "type": "string",
///          "const": "contract_code_changes"
///        }
///      }
///    },
///    {
///      "title": "DataChanges",
///      "type": "object",
///      "required": [
///        "account_ids",
///        "changes_type",
///        "key_prefix_base64"
///      ],
///      "properties": {
///        "account_ids": {
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/AccountId"
///          }
///        },
///        "changes_type": {
///          "type": "string",
///          "const": "data_changes"
///        },
///        "key_prefix_base64": {
///          "$ref": "#/definitions/StoreKey"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "changes_type")]
pub enum StateChangesRequestView {
    ///AccountChanges
    #[serde(rename = "account_changes")]
    AccountChanges { account_ids: ::std::vec::Vec<AccountId> },
    ///SingleAccessKeyChanges
    #[serde(rename = "single_access_key_changes")]
    SingleAccessKeyChanges { keys: ::std::vec::Vec<AccountWithPublicKey> },
    ///AllAccessKeyChanges
    #[serde(rename = "all_access_key_changes")]
    AllAccessKeyChanges { account_ids: ::std::vec::Vec<AccountId> },
    ///ContractCodeChanges
    #[serde(rename = "contract_code_changes")]
    ContractCodeChanges { account_ids: ::std::vec::Vec<AccountId> },
    ///DataChanges
    #[serde(rename = "data_changes")]
    DataChanges { account_ids: ::std::vec::Vec<AccountId>, key_prefix_base64: StoreKey },
}
///Item of the state, key and value are serialized in base64 and proof for inclusion of given state item.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StateItem",
///  "description": "Item of the state, key and value are serialized in base64 and proof for inclusion of given state item.",
///  "type": "object",
///  "required": [
///    "key",
///    "value"
///  ],
///  "properties": {
///    "key": {
///      "$ref": "#/definitions/StoreKey"
///    },
///    "value": {
///      "$ref": "#/definitions/StoreValue"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StateItem {
    pub key: StoreKey,
    pub value: StoreValue,
}
///`StateSyncConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StateSyncConfig",
///  "type": "object",
///  "properties": {
///    "concurrency": {
///      "$ref": "#/definitions/SyncConcurrency"
///    },
///    "dump": {
///      "description": "`none` value disables state dump to external storage.",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/DumpConfig"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "parts_compression_lvl": {
///      "description": "Zstd compression level for state parts.",
///      "default": 1,
///      "type": "integer",
///      "format": "int32"
///    },
///    "sync": {
///      "$ref": "#/definitions/SyncConfig"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StateSyncConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub concurrency: ::std::option::Option<SyncConcurrency>,
    ///`none` value disables state dump to external storage.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dump: ::std::option::Option<DumpConfig>,
    ///Zstd compression level for state parts.
    #[serde(default = "defaults::default_u64::<i32, 1>")]
    pub parts_compression_lvl: i32,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sync: ::std::option::Option<SyncConfig>,
}
impl ::std::default::Default for StateSyncConfig {
    fn default() -> Self {
        Self {
            concurrency: Default::default(),
            dump: Default::default(),
            parts_compression_lvl: defaults::default_u64::<i32, 1>(),
            sync: Default::default(),
        }
    }
}
///`StatusSyncInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StatusSyncInfo",
///  "type": "object",
///  "required": [
///    "latest_block_hash",
///    "latest_block_height",
///    "latest_block_time",
///    "latest_state_root",
///    "syncing"
///  ],
///  "properties": {
///    "earliest_block_hash": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "earliest_block_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "earliest_block_time": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "epoch_id": {
///      "oneOf": [
///        {
///          "$ref": "#/definitions/EpochId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "epoch_start_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "latest_block_hash": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "latest_block_height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "latest_block_time": {
///      "type": "string"
///    },
///    "latest_state_root": {
///      "$ref": "#/definitions/CryptoHash"
///    },
///    "syncing": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StatusSyncInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub earliest_block_hash: ::std::option::Option<CryptoHash>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub earliest_block_height: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub earliest_block_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_id: ::std::option::Option<EpochId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_start_height: ::std::option::Option<u64>,
    pub latest_block_hash: CryptoHash,
    pub latest_block_height: u64,
    pub latest_block_time: ::std::string::String,
    pub latest_state_root: CryptoHash,
    pub syncing: bool,
}
/**Errors which may occur during working with trie storages, storing
trie values (trie nodes and state values) by their hashes.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StorageError",
///  "description": "Errors which may occur during working with trie storages, storing\ntrie values (trie nodes and state values) by their hashes.",
///  "oneOf": [
///    {
///      "description": "Key-value db internal failure",
///      "type": "string",
///      "const": "StorageInternalError"
///    },
///    {
///      "title": "MissingTrieValue",
///      "description": "Requested trie value by its hash which is missing in storage.",
///      "type": "object",
///      "required": [
///        "MissingTrieValue"
///      ],
///      "properties": {
///        "MissingTrieValue": {
///          "$ref": "#/definitions/MissingTrieValue"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Found trie node which shouldn't be part of state. Raised during\nvalidation of state sync parts where incorrect node was passed.\nTODO (#8997): consider including hash of trie node.",
///      "type": "string",
///      "const": "UnexpectedTrieValue"
///    },
///    {
///      "title": "StorageInconsistentState",
///      "description": "Either invalid state or key-value db is corrupted.\nFor PartialStorage it cannot be corrupted.\nError message is unreliable and for debugging purposes only. It's also probably ok to\npanic in every place that produces this error.\nWe can check if db is corrupted by verifying everything in the state trie.",
///      "type": "object",
///      "required": [
///        "StorageInconsistentState"
///      ],
///      "properties": {
///        "StorageInconsistentState": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "FlatStorageBlockNotSupported",
///      "description": "Flat storage error, meaning that it doesn't support some block anymore.\nWe guarantee that such block cannot become final, thus block processing\nmust resume normally.",
///      "type": "object",
///      "required": [
///        "FlatStorageBlockNotSupported"
///      ],
///      "properties": {
///        "FlatStorageBlockNotSupported": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "MemTrieLoadingError",
///      "description": "In-memory trie could not be loaded for some reason.",
///      "type": "object",
///      "required": [
///        "MemTrieLoadingError"
///      ],
///      "properties": {
///        "MemTrieLoadingError": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum StorageError {
    ///Key-value db internal failure
    StorageInternalError,
    ///Requested trie value by its hash which is missing in storage.
    MissingTrieValue(MissingTrieValue),
    /**Found trie node which shouldn't be part of state. Raised during
validation of state sync parts where incorrect node was passed.
TODO (#8997): consider including hash of trie node.*/
    UnexpectedTrieValue,
    /**Either invalid state or key-value db is corrupted.
For PartialStorage it cannot be corrupted.
Error message is unreliable and for debugging purposes only. It's also probably ok to
panic in every place that produces this error.
We can check if db is corrupted by verifying everything in the state trie.*/
    StorageInconsistentState(::std::string::String),
    /**Flat storage error, meaning that it doesn't support some block anymore.
We guarantee that such block cannot become final, thus block processing
must resume normally.*/
    FlatStorageBlockNotSupported(::std::string::String),
    ///In-memory trie could not be loaded for some reason.
    MemTrieLoadingError(::std::string::String),
}
impl ::std::convert::From<MissingTrieValue> for StorageError {
    fn from(value: MissingTrieValue) -> Self {
        Self::MissingTrieValue(value)
    }
}
///This enum represents if a storage_get call will be performed through flat storage or trie
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StorageGetMode",
///  "description": "This enum represents if a storage_get call will be performed through flat storage or trie",
///  "type": "string",
///  "enum": [
///    "FlatStorage",
///    "Trie"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum StorageGetMode {
    FlatStorage,
    Trie,
}
impl ::std::fmt::Display for StorageGetMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FlatStorage => f.write_str("FlatStorage"),
            Self::Trie => f.write_str("Trie"),
        }
    }
}
impl ::std::str::FromStr for StorageGetMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "FlatStorage" => Ok(Self::FlatStorage),
            "Trie" => Ok(Self::Trie),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StorageGetMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StorageGetMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StorageGetMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Describes cost of storage per block
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StorageUsageConfigView",
///  "description": "Describes cost of storage per block",
///  "type": "object",
///  "required": [
///    "num_bytes_account",
///    "num_extra_bytes_record"
///  ],
///  "properties": {
///    "num_bytes_account": {
///      "description": "Number of bytes for an account record, including rounding up for account id.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_extra_bytes_record": {
///      "description": "Additional number of bytes for a k/v record",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StorageUsageConfigView {
    ///Number of bytes for an account record, including rounding up for account id.
    pub num_bytes_account: u64,
    ///Additional number of bytes for a k/v record
    pub num_extra_bytes_record: u64,
}
/**This type is used to mark keys (arrays of bytes) that are queried from store.

NOTE: Currently, this type is only used in the view_client and RPC to be able to transparently
pretty-serialize the bytes arrays as base64-encoded strings (see `serialize.rs`).*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StoreKey",
///  "description": "This type is used to mark keys (arrays of bytes) that are queried from store.\n\nNOTE: Currently, this type is only used in the view_client and RPC to be able to transparently\npretty-serialize the bytes arrays as base64-encoded strings (see `serialize.rs`).",
///  "type": "string",
///  "format": "bytes"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct StoreKey(pub ::std::string::String);
impl ::std::ops::Deref for StoreKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StoreKey> for ::std::string::String {
    fn from(value: StoreKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for StoreKey {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StoreKey {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StoreKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
/**This type is used to mark values returned from store (arrays of bytes).

NOTE: Currently, this type is only used in the view_client and RPC to be able to transparently
pretty-serialize the bytes arrays as base64-encoded strings (see `serialize.rs`).*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "StoreValue",
///  "description": "This type is used to mark values returned from store (arrays of bytes).\n\nNOTE: Currently, this type is only used in the view_client and RPC to be able to transparently\npretty-serialize the bytes arrays as base64-encoded strings (see `serialize.rs`).",
///  "type": "string",
///  "format": "bytes"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct StoreValue(pub ::std::string::String);
impl ::std::ops::Deref for StoreValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StoreValue> for ::std::string::String {
    fn from(value: StoreValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for StoreValue {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StoreValue {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StoreValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`SyncCheckpoint`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "SyncCheckpoint",
///  "type": "string",
///  "enum": [
///    "genesis",
///    "earliest_available"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SyncCheckpoint {
    #[serde(rename = "genesis")]
    Genesis,
    #[serde(rename = "earliest_available")]
    EarliestAvailable,
}
impl ::std::fmt::Display for SyncCheckpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Genesis => f.write_str("genesis"),
            Self::EarliestAvailable => f.write_str("earliest_available"),
        }
    }
}
impl ::std::str::FromStr for SyncCheckpoint {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "genesis" => Ok(Self::Genesis),
            "earliest_available" => Ok(Self::EarliestAvailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SyncCheckpoint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncCheckpoint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncCheckpoint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SyncConcurrency`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "SyncConcurrency",
///  "type": "object",
///  "required": [
///    "apply",
///    "apply_during_catchup",
///    "peer_downloads",
///    "per_shard"
///  ],
///  "properties": {
///    "apply": {
///      "description": "Maximum number of \"apply parts\" tasks that can be performed in parallel.\nThis is a very disk-heavy task and therefore we set this to a low limit,\nor else the rocksdb contention makes the whole server freeze up.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "apply_during_catchup": {
///      "description": "Maximum number of \"apply parts\" tasks that can be performed in parallel\nduring catchup. We set this to a very low value to avoid overloading the\nnode while it is still performing normal tasks.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "peer_downloads": {
///      "description": "Maximum number of outstanding requests for decentralized state sync.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "per_shard": {
///      "description": "The maximum parallelism to use per shard. This is mostly for fairness, because\nthe actual rate limiting is done by the TaskTrackers, but this is useful for\nbalancing the shards a little.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SyncConcurrency {
    /**Maximum number of "apply parts" tasks that can be performed in parallel.
This is a very disk-heavy task and therefore we set this to a low limit,
or else the rocksdb contention makes the whole server freeze up.*/
    pub apply: u8,
    /**Maximum number of "apply parts" tasks that can be performed in parallel
during catchup. We set this to a very low value to avoid overloading the
node while it is still performing normal tasks.*/
    pub apply_during_catchup: u8,
    ///Maximum number of outstanding requests for decentralized state sync.
    pub peer_downloads: u8,
    /**The maximum parallelism to use per shard. This is mostly for fairness, because
the actual rate limiting is done by the TaskTrackers, but this is useful for
balancing the shards a little.*/
    pub per_shard: u8,
}
///Configures how to fetch state parts during state sync.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "SyncConfig",
///  "description": "Configures how to fetch state parts during state sync.",
///  "oneOf": [
///    {
///      "description": "Syncs state from the peers without reading anything from external storage.",
///      "type": "string",
///      "const": "Peers"
///    },
///    {
///      "title": "ExternalStorage",
///      "description": "Expects parts to be available in external storage.\n\nUsually as a fallback after some number of attempts to use peers.",
///      "type": "object",
///      "required": [
///        "ExternalStorage"
///      ],
///      "properties": {
///        "ExternalStorage": {
///          "$ref": "#/definitions/ExternalStorageConfig"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum SyncConfig {
    ///Syncs state from the peers without reading anything from external storage.
    Peers,
    /**Expects parts to be available in external storage.

Usually as a fallback after some number of attempts to use peers.*/
    ExternalStorage(ExternalStorageConfig),
}
impl ::std::convert::From<ExternalStorageConfig> for SyncConfig {
    fn from(value: ExternalStorageConfig) -> Self {
        Self::ExternalStorage(value)
    }
}
///`Tier1ProxyView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Tier1ProxyView",
///  "type": "object",
///  "required": [
///    "addr",
///    "peer_id"
///  ],
///  "properties": {
///    "addr": {
///      "type": "string"
///    },
///    "peer_id": {
///      "$ref": "#/definitions/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Tier1ProxyView {
    pub addr: ::std::string::String,
    pub peer_id: PublicKey,
}
/**Describes the expected behavior of the node regarding shard tracking.
If the node is an active validator, it will also track the shards it is responsible for as a validator.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "TrackedShardsConfig",
///  "description": "Describes the expected behavior of the node regarding shard tracking.\nIf the node is an active validator, it will also track the shards it is responsible for as a validator.",
///  "oneOf": [
///    {
///      "description": "Tracks no shards (light client).",
///      "type": "string",
///      "const": "NoShards"
///    },
///    {
///      "title": "Shards",
///      "description": "Tracks arbitrary shards.",
///      "type": "object",
///      "required": [
///        "Shards"
///      ],
///      "properties": {
///        "Shards": {
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/ShardUId"
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Tracks all shards.",
///      "type": "string",
///      "const": "AllShards"
///    },
///    {
///      "title": "ShadowValidator",
///      "description": "Tracks shards that are assigned to given validator account.",
///      "type": "object",
///      "required": [
///        "ShadowValidator"
///      ],
///      "properties": {
///        "ShadowValidator": {
///          "$ref": "#/definitions/AccountId"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Schedule",
///      "description": "Rotate between these sets of tracked shards.\nUsed to simulate the behavior of chunk only producers without staking tokens.",
///      "type": "object",
///      "required": [
///        "Schedule"
///      ],
///      "properties": {
///        "Schedule": {
///          "type": "array",
///          "items": {
///            "type": "array",
///            "items": {
///              "$ref": "#/definitions/ShardId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "Accounts",
///      "description": "Tracks shards that contain one of the given account.",
///      "type": "object",
///      "required": [
///        "Accounts"
///      ],
///      "properties": {
///        "Accounts": {
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/AccountId"
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum TrackedShardsConfig {
    ///Tracks no shards (light client).
    NoShards,
    ///Tracks arbitrary shards.
    Shards(::std::vec::Vec<ShardUId>),
    ///Tracks all shards.
    AllShards,
    ///Tracks shards that are assigned to given validator account.
    ShadowValidator(AccountId),
    /**Rotate between these sets of tracked shards.
Used to simulate the behavior of chunk only producers without staking tokens.*/
    Schedule(::std::vec::Vec<::std::vec::Vec<ShardId>>),
    ///Tracks shards that contain one of the given account.
    Accounts(::std::vec::Vec<AccountId>),
}
impl ::std::convert::From<::std::vec::Vec<ShardUId>> for TrackedShardsConfig {
    fn from(value: ::std::vec::Vec<ShardUId>) -> Self {
        Self::Shards(value)
    }
}
impl ::std::convert::From<AccountId> for TrackedShardsConfig {
    fn from(value: AccountId) -> Self {
        Self::ShadowValidator(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::vec::Vec<ShardId>>>
for TrackedShardsConfig {
    fn from(value: ::std::vec::Vec<::std::vec::Vec<ShardId>>) -> Self {
        Self::Schedule(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<AccountId>> for TrackedShardsConfig {
    fn from(value: ::std::vec::Vec<AccountId>) -> Self {
        Self::Accounts(value)
    }
}
///`TransferAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "TransferAction",
///  "type": "object",
///  "required": [
///    "deposit"
///  ],
///  "properties": {
///    "deposit": {
///      "$ref": "#/definitions/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TransferAction {
    pub deposit: NearToken,
}
///Transfer NEAR to a gas key's balance
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "TransferToGasKeyAction",
///  "description": "Transfer NEAR to a gas key's balance",
///  "type": "object",
///  "required": [
///    "deposit",
///    "public_key"
///  ],
///  "properties": {
///    "deposit": {
///      "description": "Amount of NEAR to transfer to the gas key",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "public_key": {
///      "description": "The public key of the gas key to fund",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TransferToGasKeyAction {
    ///Amount of NEAR to transfer to the gas key
    pub deposit: NearToken,
    ///The public key of the gas key to fund
    pub public_key: PublicKey,
}
///Error returned in the ExecutionOutcome in case of failure
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "TxExecutionError",
///  "description": "Error returned in the ExecutionOutcome in case of failure",
///  "oneOf": [
///    {
///      "title": "ActionError",
///      "description": "An error happened during Action execution",
///      "type": "object",
///      "required": [
///        "ActionError"
///      ],
///      "properties": {
///        "ActionError": {
///          "$ref": "#/definitions/ActionError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "InvalidTxError",
///      "description": "An error happened during Transaction execution",
///      "type": "object",
///      "required": [
///        "InvalidTxError"
///      ],
///      "properties": {
///        "InvalidTxError": {
///          "$ref": "#/definitions/InvalidTxError"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum TxExecutionError {
    ///An error happened during Action execution
    ActionError(ActionError),
    ///An error happened during Transaction execution
    InvalidTxError(InvalidTxError),
}
impl ::std::convert::From<ActionError> for TxExecutionError {
    fn from(value: ActionError) -> Self {
        Self::ActionError(value)
    }
}
impl ::std::convert::From<InvalidTxError> for TxExecutionError {
    fn from(value: InvalidTxError) -> Self {
        Self::InvalidTxError(value)
    }
}
///`TxExecutionStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "TxExecutionStatus",
///  "oneOf": [
///    {
///      "description": "Transaction is waiting to be included into the block",
///      "type": "string",
///      "const": "NONE"
///    },
///    {
///      "description": "Transaction is included into the block. The block may be not finalized yet",
///      "type": "string",
///      "const": "INCLUDED"
///    },
///    {
///      "description": "Transaction is included into the block +\nAll non-refund transaction receipts finished their execution.\nThe corresponding blocks for tx and each receipt may be not finalized yet",
///      "type": "string",
///      "const": "EXECUTED_OPTIMISTIC"
///    },
///    {
///      "description": "Transaction is included into finalized block",
///      "type": "string",
///      "const": "INCLUDED_FINAL"
///    },
///    {
///      "description": "Transaction is included into finalized block +\nAll non-refund transaction receipts finished their execution.\nThe corresponding blocks for each receipt may be not finalized yet",
///      "type": "string",
///      "const": "EXECUTED"
///    },
///    {
///      "description": "Transaction is included into finalized block +\nExecution of all transaction receipts is finalized, including refund receipts",
///      "type": "string",
///      "const": "FINAL"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TxExecutionStatus {
    ///Transaction is waiting to be included into the block
    #[serde(rename = "NONE")]
    None,
    ///Transaction is included into the block. The block may be not finalized yet
    #[serde(rename = "INCLUDED")]
    Included,
    /**Transaction is included into the block +
All non-refund transaction receipts finished their execution.
The corresponding blocks for tx and each receipt may be not finalized yet*/
    #[serde(rename = "EXECUTED_OPTIMISTIC")]
    ExecutedOptimistic,
    ///Transaction is included into finalized block
    #[serde(rename = "INCLUDED_FINAL")]
    IncludedFinal,
    /**Transaction is included into finalized block +
All non-refund transaction receipts finished their execution.
The corresponding blocks for each receipt may be not finalized yet*/
    #[serde(rename = "EXECUTED")]
    Executed,
    /**Transaction is included into finalized block +
Execution of all transaction receipts is finalized, including refund receipts*/
    #[serde(rename = "FINAL")]
    Final,
}
impl ::std::fmt::Display for TxExecutionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("NONE"),
            Self::Included => f.write_str("INCLUDED"),
            Self::ExecutedOptimistic => f.write_str("EXECUTED_OPTIMISTIC"),
            Self::IncludedFinal => f.write_str("INCLUDED_FINAL"),
            Self::Executed => f.write_str("EXECUTED"),
            Self::Final => f.write_str("FINAL"),
        }
    }
}
impl ::std::str::FromStr for TxExecutionStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NONE" => Ok(Self::None),
            "INCLUDED" => Ok(Self::Included),
            "EXECUTED_OPTIMISTIC" => Ok(Self::ExecutedOptimistic),
            "INCLUDED_FINAL" => Ok(Self::IncludedFinal),
            "EXECUTED" => Ok(Self::Executed),
            "FINAL" => Ok(Self::Final),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TxExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TxExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TxExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Use global contract action
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "UseGlobalContractAction",
///  "description": "Use global contract action",
///  "type": "object",
///  "required": [
///    "contract_identifier"
///  ],
///  "properties": {
///    "contract_identifier": {
///      "$ref": "#/definitions/GlobalContractIdentifier"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UseGlobalContractAction {
    pub contract_identifier: GlobalContractIdentifier,
}
///`ValidatorInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ValidatorInfo",
///  "type": "object",
///  "required": [
///    "account_id"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValidatorInfo {
    pub account_id: AccountId,
}
///Reasons for removing a validator from the validator set.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ValidatorKickoutReason",
///  "description": "Reasons for removing a validator from the validator set.",
///  "oneOf": [
///    {
///      "description": "Deprecated",
///      "type": "string",
///      "const": "_UnusedSlashed"
///    },
///    {
///      "title": "NotEnoughBlocks",
///      "description": "Validator didn't produce enough blocks.",
///      "type": "object",
///      "required": [
///        "NotEnoughBlocks"
///      ],
///      "properties": {
///        "NotEnoughBlocks": {
///          "type": "object",
///          "required": [
///            "expected",
///            "produced"
///          ],
///          "properties": {
///            "expected": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "produced": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "NotEnoughChunks",
///      "description": "Validator didn't produce enough chunks.",
///      "type": "object",
///      "required": [
///        "NotEnoughChunks"
///      ],
///      "properties": {
///        "NotEnoughChunks": {
///          "type": "object",
///          "required": [
///            "expected",
///            "produced"
///          ],
///          "properties": {
///            "expected": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "produced": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Validator unstaked themselves.",
///      "type": "string",
///      "const": "Unstaked"
///    },
///    {
///      "title": "NotEnoughStake",
///      "description": "Validator stake is now below threshold",
///      "type": "object",
///      "required": [
///        "NotEnoughStake"
///      ],
///      "properties": {
///        "NotEnoughStake": {
///          "type": "object",
///          "required": [
///            "stake_u128",
///            "threshold_u128"
///          ],
///          "properties": {
///            "stake_u128": {
///              "$ref": "#/definitions/NearToken"
///            },
///            "threshold_u128": {
///              "$ref": "#/definitions/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Enough stake but is not chosen because of seat limits.",
///      "type": "string",
///      "const": "DidNotGetASeat"
///    },
///    {
///      "title": "NotEnoughChunkEndorsements",
///      "description": "Validator didn't produce enough chunk endorsements.",
///      "type": "object",
///      "required": [
///        "NotEnoughChunkEndorsements"
///      ],
///      "properties": {
///        "NotEnoughChunkEndorsements": {
///          "type": "object",
///          "required": [
///            "expected",
///            "produced"
///          ],
///          "properties": {
///            "expected": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "produced": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "title": "ProtocolVersionTooOld",
///      "description": "Validator's last block proposal was for a protocol version older than\nthe network's voted protocol version.",
///      "type": "object",
///      "required": [
///        "ProtocolVersionTooOld"
///      ],
///      "properties": {
///        "ProtocolVersionTooOld": {
///          "type": "object",
///          "required": [
///            "network_version",
///            "version"
///          ],
///          "properties": {
///            "network_version": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            },
///            "version": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ValidatorKickoutReason {
    ///Deprecated
    #[serde(rename = "_UnusedSlashed")]
    UnusedSlashed,
    ///Validator didn't produce enough blocks.
    NotEnoughBlocks { expected: u64, produced: u64 },
    ///Validator didn't produce enough chunks.
    NotEnoughChunks { expected: u64, produced: u64 },
    ///Validator unstaked themselves.
    Unstaked,
    ///Validator stake is now below threshold
    NotEnoughStake { stake_u128: NearToken, threshold_u128: NearToken },
    ///Enough stake but is not chosen because of seat limits.
    DidNotGetASeat,
    ///Validator didn't produce enough chunk endorsements.
    NotEnoughChunkEndorsements { expected: u64, produced: u64 },
    /**Validator's last block proposal was for a protocol version older than
the network's voted protocol version.*/
    ProtocolVersionTooOld { network_version: u32, version: u32 },
}
///`ValidatorKickoutView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ValidatorKickoutView",
///  "type": "object",
///  "required": [
///    "account_id",
///    "reason"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "reason": {
///      "$ref": "#/definitions/ValidatorKickoutReason"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValidatorKickoutView {
    pub account_id: AccountId,
    pub reason: ValidatorKickoutReason,
}
///`ValidatorStakeView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "V1",
///  "type": "object",
///  "allOf": [
///    {
///      "$ref": "#/definitions/ValidatorStakeViewV1"
///    }
///  ],
///  "required": [
///    "validator_stake_struct_version"
///  ],
///  "properties": {
///    "validator_stake_struct_version": {
///      "type": "string",
///      "const": "V1"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValidatorStakeView {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub stake: NearToken,
    pub validator_stake_struct_version: ::std::string::String,
}
///`ValidatorStakeViewV1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ValidatorStakeViewV1",
///  "type": "object",
///  "required": [
///    "account_id",
///    "public_key",
///    "stake"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/definitions/AccountId"
///    },
///    "public_key": {
///      "$ref": "#/definitions/PublicKey"
///    },
///    "stake": {
///      "$ref": "#/definitions/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValidatorStakeViewV1 {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub stake: NearToken,
}
///`ValidatorStakeViews`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ValidatorStakeViews",
///  "type": "array",
///  "items": {
///    "$ref": "#/definitions/ValidatorStakeView"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ValidatorStakeViews(pub ::std::vec::Vec<ValidatorStakeView>);
impl ::std::ops::Deref for ValidatorStakeViews {
    type Target = ::std::vec::Vec<ValidatorStakeView>;
    fn deref(&self) -> &::std::vec::Vec<ValidatorStakeView> {
        &self.0
    }
}
impl ::std::convert::From<ValidatorStakeViews> for ::std::vec::Vec<ValidatorStakeView> {
    fn from(value: ValidatorStakeViews) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ValidatorStakeView>> for ValidatorStakeViews {
    fn from(value: ::std::vec::Vec<ValidatorStakeView>) -> Self {
        Self(value)
    }
}
///Data structure for semver version and github tag or commit.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Version",
///  "description": "Data structure for semver version and github tag or commit.",
///  "type": "object",
///  "required": [
///    "build",
///    "commit",
///    "version"
///  ],
///  "properties": {
///    "build": {
///      "type": "string"
///    },
///    "commit": {
///      "type": "string"
///    },
///    "rustc_version": {
///      "default": "",
///      "type": "string"
///    },
///    "version": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Version {
    pub build: ::std::string::String,
    pub commit: ::std::string::String,
    #[serde(default)]
    pub rustc_version: ::std::string::String,
    pub version: ::std::string::String,
}
///Resulting state values for a view state query request
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ViewStateResult",
///  "description": "Resulting state values for a view state query request",
///  "type": "object",
///  "required": [
///    "values"
///  ],
///  "properties": {
///    "proof": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "values": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/StateItem"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ViewStateResult {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub proof: ::std::vec::Vec<::std::string::String>,
    pub values: ::std::vec::Vec<StateItem>,
}
///`VmConfigView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "VMConfigView",
///  "type": "object",
///  "required": [
///    "deterministic_account_ids",
///    "discard_custom_sections",
///    "eth_implicit_accounts",
///    "ext_costs",
///    "fix_contract_loading_cost",
///    "global_contract_host_fns",
///    "grow_mem_cost",
///    "implicit_account_creation",
///    "limit_config",
///    "linear_op_base_cost",
///    "linear_op_unit_cost",
///    "reftypes_bulk_memory",
///    "regular_op_cost",
///    "storage_get_mode",
///    "vm_kind"
///  ],
///  "properties": {
///    "deterministic_account_ids": {
///      "description": "See [VMConfig::deterministic_account_ids](crate::vm::Config::deterministic_account_ids).",
///      "type": "boolean"
///    },
///    "discard_custom_sections": {
///      "description": "See [VMConfig::discard_custom_sections](crate::vm::Config::discard_custom_sections).",
///      "type": "boolean"
///    },
///    "eth_implicit_accounts": {
///      "description": "See [VMConfig::eth_implicit_accounts](crate::vm::Config::eth_implicit_accounts).",
///      "type": "boolean"
///    },
///    "ext_costs": {
///      "description": "Costs for runtime externals",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ExtCostsConfigView"
///        }
///      ]
///    },
///    "fix_contract_loading_cost": {
///      "description": "See [VMConfig::fix_contract_loading_cost](crate::vm::Config::fix_contract_loading_cost).",
///      "type": "boolean"
///    },
///    "global_contract_host_fns": {
///      "description": "See [VMConfig::global_contract_host_fns](crate::vm::Config::global_contract_host_fns).",
///      "type": "boolean"
///    },
///    "grow_mem_cost": {
///      "description": "Gas cost of a growing memory by single page.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "implicit_account_creation": {
///      "description": "Deprecated",
///      "type": "boolean"
///    },
///    "limit_config": {
///      "description": "Describes limits for VM and Runtime.\n\nTODO: Consider changing this to `VMLimitConfigView` to avoid dependency\non runtime.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/LimitConfig"
///        }
///      ]
///    },
///    "linear_op_base_cost": {
///      "description": "Base gas cost of a linear operation",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "linear_op_unit_cost": {
///      "description": "Unit gas cost of a linear operation",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "reftypes_bulk_memory": {
///      "description": "See [VMConfig::reftypes_bulk_memory](crate::vm::Config::reftypes_bulk_memory).",
///      "type": "boolean"
///    },
///    "regular_op_cost": {
///      "description": "Gas cost of a regular operation.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "storage_get_mode": {
///      "description": "See [VMConfig::storage_get_mode](crate::vm::Config::storage_get_mode).",
///      "allOf": [
///        {
///          "$ref": "#/definitions/StorageGetMode"
///        }
///      ]
///    },
///    "vm_kind": {
///      "description": "See [VMConfig::vm_kind](crate::vm::Config::vm_kind).",
///      "allOf": [
///        {
///          "$ref": "#/definitions/VMKind"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VmConfigView {
    ///See [VMConfig::deterministic_account_ids](crate::vm::Config::deterministic_account_ids).
    pub deterministic_account_ids: bool,
    ///See [VMConfig::discard_custom_sections](crate::vm::Config::discard_custom_sections).
    pub discard_custom_sections: bool,
    ///See [VMConfig::eth_implicit_accounts](crate::vm::Config::eth_implicit_accounts).
    pub eth_implicit_accounts: bool,
    ///Costs for runtime externals
    pub ext_costs: ExtCostsConfigView,
    ///See [VMConfig::fix_contract_loading_cost](crate::vm::Config::fix_contract_loading_cost).
    pub fix_contract_loading_cost: bool,
    ///See [VMConfig::global_contract_host_fns](crate::vm::Config::global_contract_host_fns).
    pub global_contract_host_fns: bool,
    ///Gas cost of a growing memory by single page.
    pub grow_mem_cost: u32,
    ///Deprecated
    pub implicit_account_creation: bool,
    /**Describes limits for VM and Runtime.

TODO: Consider changing this to `VMLimitConfigView` to avoid dependency
on runtime.*/
    pub limit_config: LimitConfig,
    ///Base gas cost of a linear operation
    pub linear_op_base_cost: u64,
    ///Unit gas cost of a linear operation
    pub linear_op_unit_cost: u64,
    ///See [VMConfig::reftypes_bulk_memory](crate::vm::Config::reftypes_bulk_memory).
    pub reftypes_bulk_memory: bool,
    ///Gas cost of a regular operation.
    pub regular_op_cost: u32,
    ///See [VMConfig::storage_get_mode](crate::vm::Config::storage_get_mode).
    pub storage_get_mode: StorageGetMode,
    ///See [VMConfig::vm_kind](crate::vm::Config::vm_kind).
    pub vm_kind: VmKind,
}
///`VmKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "VMKind",
///  "oneOf": [
///    {
///      "description": "Wasmer 0.17.x VM. Gone now.",
///      "type": "string",
///      "const": "Wasmer0"
///    },
///    {
///      "description": "Wasmtime VM.",
///      "type": "string",
///      "const": "Wasmtime"
///    },
///    {
///      "description": "Wasmer 2.x VM.",
///      "type": "string",
///      "const": "Wasmer2"
///    },
///    {
///      "description": "NearVM.",
///      "type": "string",
///      "const": "NearVm"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum VmKind {
    ///Wasmer 0.17.x VM. Gone now.
    Wasmer0,
    ///Wasmtime VM.
    Wasmtime,
    ///Wasmer 2.x VM.
    Wasmer2,
    ///NearVM.
    NearVm,
}
impl ::std::fmt::Display for VmKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wasmer0 => f.write_str("Wasmer0"),
            Self::Wasmtime => f.write_str("Wasmtime"),
            Self::Wasmer2 => f.write_str("Wasmer2"),
            Self::NearVm => f.write_str("NearVm"),
        }
    }
}
impl ::std::str::FromStr for VmKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Wasmer0" => Ok(Self::Wasmer0),
            "Wasmtime" => Ok(Self::Wasmtime),
            "Wasmer2" => Ok(Self::Wasmer2),
            "NearVm" => Ok(Self::NearVm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VmKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VmKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VmKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A kind of a trap happened during execution of a binary
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "WasmTrap",
///  "description": "A kind of a trap happened during execution of a binary",
///  "oneOf": [
///    {
///      "description": "An `unreachable` opcode was executed.",
///      "type": "string",
///      "const": "Unreachable"
///    },
///    {
///      "description": "Call indirect incorrect signature trap.",
///      "type": "string",
///      "const": "IncorrectCallIndirectSignature"
///    },
///    {
///      "description": "Memory out of bounds trap.",
///      "type": "string",
///      "const": "MemoryOutOfBounds"
///    },
///    {
///      "description": "Call indirect out of bounds trap.",
///      "type": "string",
///      "const": "CallIndirectOOB"
///    },
///    {
///      "description": "An arithmetic exception, e.g. divided by zero.",
///      "type": "string",
///      "const": "IllegalArithmetic"
///    },
///    {
///      "description": "Misaligned atomic access trap.",
///      "type": "string",
///      "const": "MisalignedAtomicAccess"
///    },
///    {
///      "description": "Indirect call to null.",
///      "type": "string",
///      "const": "IndirectCallToNull"
///    },
///    {
///      "description": "Stack overflow.",
///      "type": "string",
///      "const": "StackOverflow"
///    },
///    {
///      "description": "Generic trap.",
///      "type": "string",
///      "const": "GenericTrap"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WasmTrap {
    ///An `unreachable` opcode was executed.
    Unreachable,
    ///Call indirect incorrect signature trap.
    IncorrectCallIndirectSignature,
    ///Memory out of bounds trap.
    MemoryOutOfBounds,
    ///Call indirect out of bounds trap.
    #[serde(rename = "CallIndirectOOB")]
    CallIndirectOob,
    ///An arithmetic exception, e.g. divided by zero.
    IllegalArithmetic,
    ///Misaligned atomic access trap.
    MisalignedAtomicAccess,
    ///Indirect call to null.
    IndirectCallToNull,
    ///Stack overflow.
    StackOverflow,
    ///Generic trap.
    GenericTrap,
}
impl ::std::fmt::Display for WasmTrap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unreachable => f.write_str("Unreachable"),
            Self::IncorrectCallIndirectSignature => {
                f.write_str("IncorrectCallIndirectSignature")
            }
            Self::MemoryOutOfBounds => f.write_str("MemoryOutOfBounds"),
            Self::CallIndirectOob => f.write_str("CallIndirectOOB"),
            Self::IllegalArithmetic => f.write_str("IllegalArithmetic"),
            Self::MisalignedAtomicAccess => f.write_str("MisalignedAtomicAccess"),
            Self::IndirectCallToNull => f.write_str("IndirectCallToNull"),
            Self::StackOverflow => f.write_str("StackOverflow"),
            Self::GenericTrap => f.write_str("GenericTrap"),
        }
    }
}
impl ::std::str::FromStr for WasmTrap {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Unreachable" => Ok(Self::Unreachable),
            "IncorrectCallIndirectSignature" => Ok(Self::IncorrectCallIndirectSignature),
            "MemoryOutOfBounds" => Ok(Self::MemoryOutOfBounds),
            "CallIndirectOOB" => Ok(Self::CallIndirectOob),
            "IllegalArithmetic" => Ok(Self::IllegalArithmetic),
            "MisalignedAtomicAccess" => Ok(Self::MisalignedAtomicAccess),
            "IndirectCallToNull" => Ok(Self::IndirectCallToNull),
            "StackOverflow" => Ok(Self::StackOverflow),
            "GenericTrap" => Ok(Self::GenericTrap),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WasmTrap {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WasmTrap {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WasmTrap {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Withdraw NEAR from a gas key's balance to the account
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "WithdrawFromGasKeyAction",
///  "description": "Withdraw NEAR from a gas key's balance to the account",
///  "type": "object",
///  "required": [
///    "amount",
///    "public_key"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Amount of NEAR to transfer from the gas key",
///      "allOf": [
///        {
///          "$ref": "#/definitions/NearToken"
///        }
///      ]
///    },
///    "public_key": {
///      "description": "The public key of the gas key to withdraw from",
///      "allOf": [
///        {
///          "$ref": "#/definitions/PublicKey"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WithdrawFromGasKeyAction {
    ///Amount of NEAR to transfer from the gas key
    pub amount: NearToken,
    ///The public key of the gas key to withdraw from
    pub public_key: PublicKey,
}
///Configuration specific to ChunkStateWitness.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "WitnessConfigView",
///  "description": "Configuration specific to ChunkStateWitness.",
///  "type": "object",
///  "required": [
///    "combined_transactions_size_limit",
///    "main_storage_proof_size_soft_limit",
///    "new_transactions_validation_state_size_soft_limit"
///  ],
///  "properties": {
///    "combined_transactions_size_limit": {
///      "description": "Maximum size of transactions contained inside ChunkStateWitness.\n\nA witness contains transactions from both the previous chunk and the current one.\nThis parameter limits the sum of sizes of transactions from both of those chunks.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "main_storage_proof_size_soft_limit": {
///      "description": "Size limit for storage proof generated while executing receipts in a chunk.\nAfter this limit is reached we defer execution of any new receipts.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "new_transactions_validation_state_size_soft_limit": {
///      "description": "Soft size limit of storage proof used to validate new transactions in ChunkStateWitness.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WitnessConfigView {
    /**Maximum size of transactions contained inside ChunkStateWitness.

A witness contains transactions from both the previous chunk and the current one.
This parameter limits the sum of sizes of transactions from both of those chunks.*/
    pub combined_transactions_size_limit: u32,
    /**Size limit for storage proof generated while executing receipts in a chunk.
After this limit is reached we defer execution of any new receipts.*/
    pub main_storage_proof_size_soft_limit: u64,
    ///Soft size limit of storage proof used to validate new transactions in ChunkStateWitness.
    pub new_transactions_validation_state_size_soft_limit: u64,
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<u64>,
        <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn block_header_view_rent_paid() -> super::NearToken {
        super::NearToken("0".to_string())
    }
    pub(super) fn block_header_view_validator_reward() -> super::NearToken {
        super::NearToken("0".to_string())
    }
    pub(super) fn chunk_header_view_rent_paid() -> super::NearToken {
        super::NearToken("0".to_string())
    }
    pub(super) fn chunk_header_view_validator_reward() -> super::NearToken {
        super::NearToken("0".to_string())
    }
    pub(super) fn cloud_archival_writer_config_polling_interval() -> super::DurationAsStdSchemaProvider {
        super::DurationAsStdSchemaProvider {
            nanos: 0_i32,
            secs: 1_i64,
        }
    }
    pub(super) fn execution_outcome_view_metadata() -> super::ExecutionMetadataView {
        super::ExecutionMetadataView {
            gas_profile: ::std::option::Option::None,
            version: 1_u32,
        }
    }
    pub(super) fn gc_config_gc_step_period() -> super::DurationAsStdSchemaProvider {
        super::DurationAsStdSchemaProvider {
            nanos: 500000000_i32,
            secs: 0_i64,
        }
    }
    pub(super) fn genesis_config_minimum_stake_ratio() -> [i32; 2usize] {
        [1_i32, 6250_i32]
    }
    pub(super) fn genesis_config_online_max_threshold() -> [i32; 2usize] {
        [99_i32, 100_i32]
    }
    pub(super) fn genesis_config_online_min_threshold() -> [i32; 2usize] {
        [9_i32, 10_i32]
    }
    pub(super) fn genesis_config_protocol_upgrade_stake_threshold() -> [i32; 2usize] {
        [4_i32, 5_i32]
    }
    pub(super) fn genesis_config_shard_layout() -> super::ShardLayout {
        super::ShardLayout::V2(super::ShardLayoutV2 {
            boundary_accounts: vec![],
            id_to_index_map: [("0".to_string(), 0_u32)].into_iter().collect(),
            index_to_id_map: [("0".to_string(), super::ShardId(0_u64))]
                .into_iter()
                .collect(),
            shard_ids: vec![super::ShardId(0_u64)],
            shards_parent_map: ::std::option::Option::None,
            shards_split_map: ::std::option::Option::None,
            version: 0_u32,
        })
    }
    pub(super) fn limit_config_account_id_validity_rules_version() -> super::AccountIdValidityRulesVersion {
        super::AccountIdValidityRulesVersion(0_u8)
    }
    pub(super) fn rpc_send_transaction_request_wait_until() -> super::TxExecutionStatus {
        super::TxExecutionStatus::ExecutedOptimistic
    }
    pub(super) fn rpc_transaction_status_request_signed_tx_base64_wait_until() -> super::TxExecutionStatus {
        super::TxExecutionStatus::ExecutedOptimistic
    }
    pub(super) fn rpc_transaction_status_request_tx_hash_sender_account_id_wait_until() -> super::TxExecutionStatus {
        super::TxExecutionStatus::ExecutedOptimistic
    }
}
