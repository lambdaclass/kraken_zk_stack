// To change the code generated, modify the codegen tool instead:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen

// Code generated with version:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen#91e3a82e65d6a47f05418b6d9acc326e3d0b19c5

// Code generation requested but not implemented for these types:
// - `BLOCK_ID`
// - `BROADCASTED_DECLARE_TXN`
// - `BROADCASTED_INVOKE_TXN`
// - `BROADCASTED_TXN`
// - `CONTRACT_ABI_ENTRY`
// - `CONTRACT_CLASS`
// - `DECLARE_TXN`
// - `INVOKE_TXN`
// - `PENDING_TXN_RECEIPT`
// - `TXN`
// - `TXN_RECEIPT`

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use cairo_felt::Felt252;

use mempool::TransactionType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;

use super::*;
use serializable_types::base64;
use serializable_types::NumAsHex;

/// Block status.
///
/// The status of the block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
    #[serde(rename = "REJECTED")]
    Rejected,
}

/// Block tag.
///
/// A tag specifying a dynamic reference to a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pending")]
    Pending,
}

/// Block with transaction hashes.
///
/// The block object.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockWithTxHashes {
    /// Status
    pub status: BlockStatus,
    /// Block hash
    #[serde_as(as = "FeltHex")]
    pub block_hash: Felt252,
    /// The hash of this block's parent
    #[serde_as(as = "FeltHex")]
    pub parent_hash: Felt252,
    /// The block number (its height)
    pub block_number: u64,
    /// The new global state root
    #[serde_as(as = "FeltHex")]
    pub new_root: Felt252,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "FeltHex")]
    pub sequencer_address: Felt252,
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<FeltHex>")]
    pub transactions: Vec<Felt252>,
}

/// NOTE: Not from RPC spec standard
/// Block with transaction hashes.
///
/// The block object.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockWithInternalTransactions {
    /// Block hash
    #[serde_as(as = "FeltHex")]
    pub block_hash: Felt252,
    /// The hash of this block's parent
    #[serde_as(as = "FeltHex")]
    pub parent_hash: Felt252,
    /// The block number (its height)
    pub block_number: u64,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "FeltHex")]
    pub sequencer_address: Felt252,
    /// The hashes of the transactions included in this block
    pub transactions: Vec<TransactionType>,
}

/// Block with transactions.
///
/// The block object.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockWithTxs {
    /// Status
    pub status: BlockStatus,
    /// Block hash
    #[serde_as(as = "FeltHex")]
    pub block_hash: Felt252,
    /// The hash of this block's parent
    #[serde_as(as = "FeltHex")]
    pub parent_hash: Felt252,
    /// The block number (its height)
    pub block_number: u64,
    /// The new global state root
    #[serde_as(as = "FeltHex")]
    pub new_root: Felt252,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "FeltHex")]
    pub sequencer_address: Felt252,
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
}

/// Broadcasted declare transaction v1.
///
/// Mempool representation of a declare transaction.
#[derive(Debug, Clone)]
pub struct BroadcastedDeclareTransactionV1 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// The class to be declared
    pub contract_class: Arc<CompressedLegacyContractClass>,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt252,
}

/// Broadcasted declare transaction v2.
///
/// Mempool representation of a declare transaction v2.
#[derive(Debug, Clone)]
pub struct BroadcastedDeclareTransactionV2 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// The class to be declared
    pub contract_class: Arc<FlattenedSierraClass>,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt252,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: Felt252,
}

/// Broadcasted deploy account transaction.
///
/// Mempool representation of a deploy account transaction.
#[derive(Debug, Clone)]
pub struct BroadcastedDeployAccountTransaction {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt252,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt252>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt252,
}

/// Version 0 invoke transaction.
///
/// Invokes a specific function in the desired contract (not necessarily an account).
#[derive(Debug, Clone)]
pub struct BroadcastedInvokeTransactionV0 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// Contract address
    pub contract_address: Felt252,
    /// Entry point selector
    pub entry_point_selector: Felt252,
    /// The parameters passed to the function
    pub calldata: Vec<Felt252>,
}

/// Version 1 invoke transaction.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone)]
pub struct BroadcastedInvokeTransactionV1 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    pub sender_address: Felt252,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<Felt252>,
}

/// Deprecated contract class.
///
/// The definition of a legacy (cairo 0) Starknet contract class.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompressedLegacyContractClass {
    /// A base64 representation of the compressed program code
    #[serde(with = "base64")]
    pub program: Vec<u8>,
    /// Deprecated entry points by type
    pub entry_points_by_type: LegacyEntryPointsByType,
    /// Contract abi
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<LegacyContractAbiEntry>>,
}

/// Contract storage diff item.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractStorageDiffItem {
    /// The contract address for which the storage changed
    #[serde_as(as = "FeltHex")]
    pub address: Felt252,
    /// The changes in the storage of the contract
    pub storage_entries: Vec<StorageEntry>,
}

/// Declare transaction receipt.
#[derive(Debug, Clone)]
pub struct DeclareTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Status
    pub status: TransactionStatus,
    /// Block hash
    pub block_hash: Felt252,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Declare contract transaction v1.
///
/// Declare contract transaction v1.
#[derive(Debug, Clone)]
pub struct DeclareTransactionV1 {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// The hash of the declared class
    pub class_hash: Felt252,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt252,
}

/// Declare transaction v2.
///
/// Declare contract transaction v2.
#[derive(Debug, Clone)]
pub struct DeclareTransactionV2 {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// The hash of the declared class
    pub class_hash: Felt252,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt252,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: Felt252,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclaredClassItem {
    /// The hash of the declared class
    #[serde_as(as = "FeltHex")]
    pub class_hash: Felt252,
    /// The cairo assembly hash corresponding to the declared class
    #[serde_as(as = "FeltHex")]
    pub compiled_class_hash: Felt252,
}

/// Deploy account transaction.
///
/// Deploys an account contract, charges fee from the pre-funded account addresses.
#[derive(Debug, Clone)]
pub struct DeployAccountTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt252,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt252>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt252,
}

/// Deploy account transaction receipt.
#[derive(Debug, Clone)]
pub struct DeployAccountTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Status
    pub status: TransactionStatus,
    /// Block hash
    pub block_hash: Felt252,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The address of the deployed contract
    pub contract_address: Felt252,
}

/// Deploy contract transaction.
///
/// The structure of a deploy transaction. Note that this transaction type is deprecated and will no
/// longer be supported in future versions.
#[derive(Debug, Clone)]
pub struct DeployTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The hash of the deployed contract's class
    pub class_hash: Felt252,
    /// Version of the transaction scheme
    pub version: u64,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt252,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt252>,
}

/// Deploy transaction receipt.
#[derive(Debug, Clone)]
pub struct DeployTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Status
    pub status: TransactionStatus,
    /// Block hash
    pub block_hash: Felt252,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The address of the deployed contract
    pub contract_address: Felt252,
}

/// Deployed contract item.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployedContractItem {
    /// The address of the contract
    #[serde_as(as = "FeltHex")]
    pub address: Felt252,
    /// The hash of the contract code
    #[serde_as(as = "FeltHex")]
    pub class_hash: Felt252,
}

/// Emitted event.
///
/// Event information decorated with metadata on where it was emitted / an event emitted as a result
/// of transaction execution.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EmittedEvent {
    /// From address
    #[serde_as(as = "FeltHex")]
    pub from_address: Felt252,
    /// Keys
    #[serde_as(as = "Vec<FeltHex>")]
    pub keys: Vec<Felt252>,
    /// Data
    #[serde_as(as = "Vec<FeltHex>")]
    pub data: Vec<Felt252>,
    /// The hash of the block in which the event was emitted
    #[serde_as(as = "FeltHex")]
    pub block_hash: Felt252,
    /// The number of the block in which the event was emitted
    pub block_number: u64,
    /// The transaction that emitted the event
    #[serde_as(as = "FeltHex")]
    pub transaction_hash: Felt252,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EntryPointsByType {
    /// Constructor
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Vec<SierraEntryPoint>,
    /// External
    #[serde(rename = "EXTERNAL")]
    pub external: Vec<SierraEntryPoint>,
    /// L1 handler
    #[serde(rename = "L1_HANDLER")]
    pub l1_handler: Vec<SierraEntryPoint>,
}

/// Event.
///
/// A Starknet event.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct Event {
    /// From address
    #[serde_as(as = "FeltHex")]
    pub from_address: Felt252,
    /// Keys
    #[serde_as(as = "Vec<FeltHex>")]
    pub keys: Vec<Felt252>,
    /// Data
    #[serde_as(as = "Vec<FeltHex>")]
    pub data: Vec<Felt252>,
}

/// Event filter.
///
/// An event filter/query.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventFilter {
    /// From block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockId>,
    /// To block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockId>,
    /// From contract
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<FeltHex>")]
    pub address: Option<Felt252>,
    /// The values used to filter the events
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Vec<Vec<FeltHex>>>")]
    pub keys: Option<Vec<Vec<Felt252>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventFilterWithPage {
    #[serde(flatten)]
    pub event_filter: EventFilter,
    #[serde(flatten)]
    pub result_page_request: ResultPageRequest,
}

/// Events chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventsChunk {
    /// Matching events
    pub events: Vec<EmittedEvent>,
    /// Use this token in a subsequent query to obtain the next page. Should not appear if there are
    /// no more pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

/// Fee estimation.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FeeEstimate {
    /// The Ethereum gas cost of the transaction (see
    /// https://docs.starknet.io/docs/fees/fee-mechanism for more info)
    #[serde_as(as = "NumAsHex")]
    pub gas_consumed: u64,
    /// The gas price (in gwei) that was used in the cost estimation
    #[serde_as(as = "NumAsHex")]
    pub gas_price: u64,
    /// The estimated fee for the transaction (in gwei), product of gas_consumed and gas_price
    #[serde_as(as = "NumAsHex")]
    pub overall_fee: u64,
}

/// The definition of a sierra Starknet contract class.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FlattenedSierraClass {
    /// The list of sierra instructions of which the program consists
    #[serde_as(as = "Vec<FeltHex>")]
    pub sierra_program: Vec<Felt252>,
    /// The version of the contract class object. Currently, the Starknet os supports version 0.1.0
    pub contract_class_version: String,
    /// Entry points by type
    pub entry_points_by_type: EntryPointsByType,
    /// The class abi, as supplied by the user declaring the class
    pub abi: String,
}

/// Function call.
///
/// Function call information.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FunctionCall {
    /// Contract address
    #[serde_as(as = "FeltHex")]
    pub contract_address: Felt252,
    /// Entry point selector
    #[serde_as(as = "FeltHex")]
    pub entry_point_selector: Felt252,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<FeltHex>")]
    pub calldata: Vec<Felt252>,
}

/// Function state mutability type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionStateMutability {
    #[serde(rename = "view")]
    View,
}

/// Invoke transaction receipt.
#[derive(Debug, Clone)]
pub struct InvokeTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Status
    pub status: TransactionStatus,
    /// Block hash
    pub block_hash: Felt252,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Invoke transaction v0.
///
/// Invokes a specific function in the desired contract (not necessarily an account).
#[derive(Debug, Clone)]
pub struct InvokeTransactionV0 {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// Contract address
    pub contract_address: Felt252,
    /// Entry point selector
    pub entry_point_selector: Felt252,
    /// The parameters passed to the function
    pub calldata: Vec<Felt252>,
}

/// Invoke transaction v1.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone)]
pub struct InvokeTransactionV1 {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt252,
    /// Signature
    pub signature: Vec<Felt252>,
    /// Nonce
    pub nonce: Felt252,
    /// Sender address
    pub sender_address: Felt252,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<Felt252>,
}

impl InvokeTransactionV1 {
    /// Creates a transaction and returns it as a vector of bytes.
    /// The transaction is a InvokeTransactionV1 transaction.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the transaction.
    pub fn new_as_bytes(nonce: u64, calldata: u64) -> Vec<u8> {
        // let transaction_type = TransactionType::ExecuteFibonacci(200 + counter);
        // TODO: these are default values, need to be changed
        let mut starknet_transaction = InvokeTransactionV1 {
            transaction_hash: Felt252::new(0), //Temporary hash
            max_fee: Felt252::new(89853483),
            signature: vec![Felt252::new(183728913)],
            nonce: Felt252::new(nonce),
            sender_address: Felt252::new(91232018),
            calldata: vec![Felt252::new(calldata)],
        };
        starknet_transaction.transaction_hash = Felt252::new(starknet_transaction.calculate_hash());
        let starknet_transaction_str = serde_json::to_string(&starknet_transaction).unwrap();
        starknet_transaction_str.as_bytes().to_owned()
    }

    pub fn from_bytes(bytes: &[u8]) -> InvokeTransactionV1 {
        let tx_string = String::from_utf8(
            bytes
                .iter()
                .skip(9)
                .take_while(|v| *v != &0)
                .map(|v| *v)
                .collect(),
        )
        .unwrap();

        serde_json::from_str::<InvokeTransactionV1>(&tx_string).unwrap()
    }

    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Hash for InvokeTransactionV1 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.max_fee.hash(state);
        self.signature.hash(state);
        self.nonce.hash(state);
        self.sender_address.hash(state);
        self.calldata.hash(state);
    }
}

/// L1 handler transaction.
#[derive(Debug, Clone)]
pub struct L1HandlerTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// Version of the transaction scheme
    pub version: u64,
    /// The L1->L2 message nonce field of the sn core L1 contract at the time the transaction was
    /// sent
    pub nonce: u64,
    /// Contract address
    pub contract_address: Felt252,
    /// Entry point selector
    pub entry_point_selector: Felt252,
    /// The parameters passed to the function
    pub calldata: Vec<Felt252>,
}

/// L1 handler transaction receipt.
///
/// Receipt for L1 handler transaction.
#[derive(Debug, Clone)]
pub struct L1HandlerTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Status
    pub status: TransactionStatus,
    /// Block hash
    pub block_hash: Felt252,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Deprecated cairo entry point.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyContractEntryPoint {
    /// The offset of the entry point in the program
    #[serde_as(as = "NumAsHex")]
    pub offset: u64,
    /// A unique identifier of the entry point (function) in the program
    #[serde_as(as = "FeltHex")]
    pub selector: Felt252,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyEntryPointsByType {
    /// Deprecated constructor
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Vec<LegacyContractEntryPoint>,
    /// Deprecated external
    #[serde(rename = "EXTERNAL")]
    pub external: Vec<LegacyContractEntryPoint>,
    /// Deprecated L1 handler
    #[serde(rename = "L1_HANDLER")]
    pub l1_handler: Vec<LegacyContractEntryPoint>,
}

/// Event abi entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyEventAbiEntry {
    /// Event abi type
    pub r#type: LegacyEventAbiType,
    /// The event name
    pub name: String,
    /// Typed parameter
    pub keys: Vec<LegacyTypedParameter>,
    /// Typed parameter
    pub data: Vec<LegacyTypedParameter>,
}

/// Event abi type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegacyEventAbiType {
    #[serde(rename = "event")]
    Event,
}

/// Function abi entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyFunctionAbiEntry {
    /// Function abi type
    pub r#type: LegacyFunctionAbiType,
    /// The function name
    pub name: String,
    /// Typed parameter
    pub inputs: Vec<LegacyTypedParameter>,
    /// Typed parameter
    pub outputs: Vec<LegacyTypedParameter>,
    /// Function state mutability
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stateMutability")]
    pub state_mutability: Option<FunctionStateMutability>,
}

/// Function abi type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegacyFunctionAbiType {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "l1_handler")]
    L1Handler,
    #[serde(rename = "constructor")]
    Constructor,
}

/// Struct abi entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyStructAbiEntry {
    /// Struct abi type
    pub r#type: LegacyStructAbiType,
    /// The struct name
    pub name: String,
    /// Size
    pub size: u64,
    /// Members
    pub members: Vec<LegacyStructMember>,
}

/// Struct abi type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegacyStructAbiType {
    #[serde(rename = "struct")]
    Struct,
}

/// Struct member.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyStructMember {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    pub r#type: String,
    /// Offset of this property within the struct
    pub offset: u64,
}

/// Typed parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyTypedParameter {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    pub r#type: String,
}

/// Message to L1.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct MsgToL1 {
    /// The address of the L2 contract sending the message
    #[serde_as(as = "FeltHex")]
    pub from_address: Felt252,
    /// The target L1 address the message is sent to
    #[serde_as(as = "FeltHex")]
    pub to_address: Felt252,
    /// The payload of the message
    #[serde_as(as = "Vec<FeltHex>")]
    pub payload: Vec<Felt252>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct NonceUpdate {
    /// The address of the contract
    #[serde_as(as = "FeltHex")]
    pub contract_address: Felt252,
    /// The nonce for the given address at the end of the block
    #[serde_as(as = "FeltHex")]
    pub nonce: Felt252,
}

/// Pending block with transaction hashes.
///
/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PendingBlockWithTxHashes {
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<FeltHex>")]
    pub transactions: Vec<Felt252>,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "FeltHex")]
    pub sequencer_address: Felt252,
    /// The hash of this block's parent
    #[serde_as(as = "FeltHex")]
    pub parent_hash: Felt252,
}

/// Pending block with transactions.
///
/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PendingBlockWithTxs {
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "FeltHex")]
    pub sequencer_address: Felt252,
    /// The hash of this block's parent
    #[serde_as(as = "FeltHex")]
    pub parent_hash: Felt252,
}

/// Pending declare transaction receipt.
#[derive(Debug, Clone)]
pub struct PendingDeclareTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Pending deploy account transaction receipt.
#[derive(Debug, Clone)]
pub struct PendingDeployAccountTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Pending deploy transaction receipt.
#[derive(Debug, Clone)]
pub struct PendingDeployTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The address of the deployed contract
    pub contract_address: Felt252,
}

/// Pending invoke transaction receipt.
#[derive(Debug, Clone)]
pub struct PendingInvokeTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Pending receipt for L1 handler transaction.
#[derive(Debug, Clone)]
pub struct PendingL1HandlerTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt252,
    /// The fee that was charged by the sequencer
    pub actual_fee: Felt252,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Pending state update.
///
/// Pending state update.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PendingStateUpdate {
    /// The previous global state root
    #[serde_as(as = "FeltHex")]
    pub old_root: Felt252,
    /// The change in state applied in this block, given as a mapping of addresses to the new values
    /// and/or new contracts
    pub state_diff: StateDiff,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ReplacedClassItem {
    /// The address of the contract whose class was replaced
    #[serde_as(as = "FeltHex")]
    pub contract_address: Felt252,
    /// The new class hash
    #[serde_as(as = "FeltHex")]
    pub class_hash: Felt252,
}

/// Result page request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ResultPageRequest {
    /// The token returned from the previous query. If no token is provided the first page is
    /// returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// Chunk size
    pub chunk_size: u64,
}

/// Sierra entry point.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraEntryPoint {
    /// A unique identifier of the entry point (function) in the program
    #[serde_as(as = "FeltHex")]
    pub selector: Felt252,
    /// The index of the function in the program
    pub function_idx: u64,
}

/// JSON-RPC error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum StarknetError {
    /// Failed to write transaction
    #[error("Failed to write transaction")]
    FailedToReceiveTransaction,
    /// Contract not found
    #[error("Contract not found")]
    ContractNotFound,
    /// Block not found
    #[error("Block not found")]
    BlockNotFound,
    /// Transaction hash not found
    #[error("Transaction hash not found")]
    TransactionHashNotFound,
    /// Invalid transaction index in a block
    #[error("Invalid transaction index in a block")]
    InvalidTransactionIndex,
    /// Class hash not found
    #[error("Class hash not found")]
    ClassHashNotFound,
    /// Requested page size is too big
    #[error("Requested page size is too big")]
    PageSizeTooBig,
    /// There are no blocks
    #[error("There are no blocks")]
    NoBlocks,
    /// The supplied continuation token is invalid or unknown
    #[error("The supplied continuation token is invalid or unknown")]
    InvalidContinuationToken,
    /// Too many keys provided in a filter
    #[error("Too many keys provided in a filter")]
    TooManyKeysInFilter,
    /// Contract error
    #[error("Contract error")]
    ContractError,
    /// Invalid contract class
    #[error("Invalid contract class")]
    InvalidContractClass,
    /// Class already declared
    #[error("Class already declared")]
    ClassAlreadyDeclared,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateDiff {
    /// Storage diffs
    pub storage_diffs: Vec<ContractStorageDiffItem>,
    /// Deprecated declared classes
    #[serde_as(as = "Vec<FeltHex>")]
    pub deprecated_declared_classes: Vec<Felt252>,
    /// Declared classes
    pub declared_classes: Vec<DeclaredClassItem>,
    /// Deployed contracts
    pub deployed_contracts: Vec<DeployedContractItem>,
    /// Replaced classes
    pub replaced_classes: Vec<ReplacedClassItem>,
    /// Nonces
    pub nonces: Vec<NonceUpdate>,
}

/// State update.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateUpdate {
    /// Block hash
    #[serde_as(as = "FeltHex")]
    pub block_hash: Felt252,
    /// The new global state root
    #[serde_as(as = "FeltHex")]
    pub new_root: Felt252,
    /// The previous global state root
    #[serde_as(as = "FeltHex")]
    pub old_root: Felt252,
    /// The change in state applied in this block, given as a mapping of addresses to the new values
    /// and/or new contracts
    pub state_diff: StateDiff,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StorageEntry {
    /// The key of the changed value
    #[serde_as(as = "FeltHex")]
    pub key: Felt252,
    /// The new value applied to the given address
    #[serde_as(as = "FeltHex")]
    pub value: Felt252,
}

/// Sync status.
///
/// An object describing the node synchronization status.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SyncStatus {
    /// The hash of the block from which the sync started
    #[serde_as(as = "FeltHex")]
    pub starting_block_hash: Felt252,
    /// The number (height) of the block from which the sync started
    #[serde_as(as = "NumAsHex")]
    pub starting_block_num: u64,
    /// The hash of the current block being synchronized
    #[serde_as(as = "FeltHex")]
    pub current_block_hash: Felt252,
    /// The number (height) of the current block being synchronized
    #[serde_as(as = "NumAsHex")]
    pub current_block_num: u64,
    /// The hash of the estimated highest block to be synchronized
    #[serde_as(as = "FeltHex")]
    pub highest_block_hash: Felt252,
    /// The number (height) of the estimated highest block to be synchronized
    #[serde_as(as = "NumAsHex")]
    pub highest_block_num: u64,
}

/// Transaction status.
///
/// The status of the transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
    #[serde(rename = "REJECTED")]
    Rejected,
}

/// Request for method starknet_addDeclareTransaction
#[derive(Debug, Clone)]
pub struct AddDeclareTransactionRequest {
    pub declare_transaction: BroadcastedDeclareTransaction,
}

/// Reference version of [AddDeclareTransactionRequest].
#[derive(Debug, Clone)]
pub struct AddDeclareTransactionRequestRef<'a> {
    pub declare_transaction: &'a BroadcastedDeclareTransaction,
}

/// Request for method starknet_addDeployAccountTransaction
#[derive(Debug, Clone)]
pub struct AddDeployAccountTransactionRequest {
    /// The deploy account transaction
    pub deploy_account_transaction: BroadcastedDeployAccountTransaction,
}

/// Reference version of [AddDeployAccountTransactionRequest].
#[derive(Debug, Clone)]
pub struct AddDeployAccountTransactionRequestRef<'a> {
    pub deploy_account_transaction: &'a BroadcastedDeployAccountTransaction,
}

/// Request for method starknet_addInvokeTransaction
#[derive(Debug, Clone)]
pub struct AddInvokeTransactionRequest {
    /// The information needed to invoke the function (or account, for version 1 transactions)
    pub invoke_transaction: BroadcastedInvokeTransaction,
}

/// Reference version of [AddInvokeTransactionRequest].
#[derive(Debug, Clone)]
pub struct AddInvokeTransactionRequestRef<'a> {
    pub invoke_transaction: &'a BroadcastedInvokeTransaction,
}

/// Request for method starknet_blockHashAndNumber
#[derive(Debug, Clone)]
pub struct BlockHashAndNumberRequest;

/// Request for method starknet_blockNumber
#[derive(Debug, Clone)]
pub struct BlockNumberRequest;

/// Request for method starknet_call
#[derive(Debug, Clone)]
pub struct CallRequest {
    pub request: FunctionCall,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
}

/// Reference version of [CallRequest].
#[derive(Debug, Clone)]
pub struct CallRequestRef<'a> {
    pub request: &'a FunctionCall,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_chainId
#[derive(Debug, Clone)]
pub struct ChainIdRequest;

/// Request for method starknet_estimateFee
#[derive(Debug, Clone)]
pub struct EstimateFeeRequest {
    pub request: Vec<BroadcastedTransaction>,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
}

/// Reference version of [EstimateFeeRequest].
#[derive(Debug, Clone)]
pub struct EstimateFeeRequestRef<'a> {
    pub request: &'a [BroadcastedTransaction],
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockTransactionCount
#[derive(Debug, Clone)]
pub struct GetBlockTransactionCountRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockTransactionCountRequest].
#[derive(Debug, Clone)]
pub struct GetBlockTransactionCountRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockWithTxHashes
#[derive(Debug, Clone)]
pub struct GetBlockWithTxHashesRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockWithTxHashesRequest].
#[derive(Debug, Clone)]
pub struct GetBlockWithTxHashesRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockWithTxs
#[derive(Debug, Clone)]
pub struct GetBlockWithTxsRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockWithTxsRequest].
#[derive(Debug, Clone)]
pub struct GetBlockWithTxsRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getClassAt
#[derive(Debug, Clone)]
pub struct GetClassAtRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose class definition will be returned
    pub contract_address: Felt252,
}

/// Reference version of [GetClassAtRequest].
#[derive(Debug, Clone)]
pub struct GetClassAtRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a Felt252,
}

/// Request for method starknet_getClassHashAt
#[derive(Debug, Clone)]
pub struct GetClassHashAtRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose class hash will be returned
    pub contract_address: Felt252,
}

/// Reference version of [GetClassHashAtRequest].
#[derive(Debug, Clone)]
pub struct GetClassHashAtRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a Felt252,
}

/// Request for method starknet_getClass
#[derive(Debug, Clone)]
pub struct GetClassRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The hash of the requested contract class
    pub class_hash: Felt252,
}

/// Reference version of [GetClassRequest].
#[derive(Debug, Clone)]
pub struct GetClassRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub class_hash: &'a Felt252,
}

/// Request for method starknet_getEvents
#[derive(Debug, Clone)]
pub struct GetEventsRequest {
    pub filter: EventFilterWithPage,
}

/// Reference version of [GetEventsRequest].
#[derive(Debug, Clone)]
pub struct GetEventsRequestRef<'a> {
    pub filter: &'a EventFilterWithPage,
}

/// Request for method starknet_getNonce
#[derive(Debug, Clone)]
pub struct GetNonceRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose nonce we're seeking
    pub contract_address: Felt252,
}

/// Reference version of [GetNonceRequest].
#[derive(Debug, Clone)]
pub struct GetNonceRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a Felt252,
}

/// Request for method starknet_getStateUpdate
#[derive(Debug, Clone)]
pub struct GetStateUpdateRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetStateUpdateRequest].
#[derive(Debug, Clone)]
pub struct GetStateUpdateRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getStorageAt
#[derive(Debug, Clone)]
pub struct GetStorageAtRequest {
    /// The address of the contract to read from
    pub contract_address: Felt252,
    /// The key to the storage value for the given contract
    pub key: Felt252,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetStorageAtRequest].
#[derive(Debug, Clone)]
pub struct GetStorageAtRequestRef<'a> {
    pub contract_address: &'a Felt252,
    pub key: &'a Felt252,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getTransactionByBlockIdAndIndex
#[derive(Debug, Clone)]
pub struct GetTransactionByBlockIdAndIndexRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    pub index: u64,
}

/// Reference version of [GetTransactionByBlockIdAndIndexRequest].
#[derive(Debug, Clone)]
pub struct GetTransactionByBlockIdAndIndexRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub index: &'a u64,
}

/// Request for method starknet_getTransactionByHash
#[derive(Debug, Clone)]
pub struct GetTransactionByHashRequest {
    pub transaction_hash: Felt252,
}

/// Reference version of [GetTransactionByHashRequest].
#[derive(Debug, Clone)]
pub struct GetTransactionByHashRequestRef<'a> {
    pub transaction_hash: &'a Felt252,
}

/// Request for method starknet_getTransactionReceipt
#[derive(Debug, Clone)]
pub struct GetTransactionReceiptRequest {
    pub transaction_hash: Felt252,
}

/// Reference version of [GetTransactionReceiptRequest].
#[derive(Debug, Clone)]
pub struct GetTransactionReceiptRequestRef<'a> {
    pub transaction_hash: &'a Felt252,
}

/// Request for method starknet_pendingTransactions
#[derive(Debug, Clone)]
pub struct PendingTransactionsRequest;

/// Request for method starknet_syncing
#[derive(Debug, Clone)]
pub struct SyncingRequest;

impl Serialize for BroadcastedDeclareTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            pub contract_class: &'a CompressedLegacyContractClass,
            #[serde_as(as = "FeltHex")]
            pub sender_address: &'a Felt252,
        }

        let tagged = Tagged {
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "DECLARE",
            contract_class: &self.contract_class,
            sender_address: &self.sender_address,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeclareTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub contract_class: CompressedLegacyContractClass,
            #[serde_as(as = "FeltHex")]
            pub sender_address: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_class: Arc::new(tagged.contract_class),
            sender_address: tagged.sender_address,
        })
    }
}

impl Serialize for BroadcastedDeclareTransactionV2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            pub contract_class: &'a FlattenedSierraClass,
            #[serde_as(as = "FeltHex")]
            pub sender_address: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub compiled_class_hash: &'a Felt252,
        }

        let tagged = Tagged {
            max_fee: &self.max_fee,
            version: &2,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "DECLARE",
            contract_class: &self.contract_class,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeclareTransactionV2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub contract_class: FlattenedSierraClass,
            #[serde_as(as = "FeltHex")]
            pub sender_address: Felt252,
            #[serde_as(as = "FeltHex")]
            pub compiled_class_hash: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &2 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_class: Arc::new(tagged.contract_class),
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
        })
    }
}

impl Serialize for BroadcastedDeployAccountTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub contract_address_salt: &'a Felt252,
            #[serde_as(as = "[FeltHex]")]
            pub constructor_calldata: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub class_hash: &'a Felt252,
        }

        let tagged = Tagged {
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "DEPLOY_ACCOUNT",
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeployAccountTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub contract_address_salt: Felt252,
            #[serde_as(as = "Vec<FeltHex>")]
            pub constructor_calldata: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub class_hash: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for BroadcastedInvokeTransactionV0 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub entry_point_selector: &'a Felt252,
            #[serde_as(as = "[FeltHex]")]
            pub calldata: &'a [Felt252],
        }

        let tagged = Tagged {
            max_fee: &self.max_fee,
            version: &0,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "INVOKE",
            contract_address: &self.contract_address,
            entry_point_selector: &self.entry_point_selector,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedInvokeTransactionV0 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
            #[serde_as(as = "FeltHex")]
            pub entry_point_selector: Felt252,
            #[serde_as(as = "Vec<FeltHex>")]
            pub calldata: Vec<Felt252>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &0 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address: tagged.contract_address,
            entry_point_selector: tagged.entry_point_selector,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for BroadcastedInvokeTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub sender_address: &'a Felt252,
            #[serde_as(as = "[FeltHex]")]
            pub calldata: &'a [Felt252],
        }

        let tagged = Tagged {
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "INVOKE",
            sender_address: &self.sender_address,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedInvokeTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub sender_address: Felt252,
            #[serde_as(as = "Vec<FeltHex>")]
            pub calldata: Vec<Felt252>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for DeclareTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub status: &'a TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: &'a Felt252,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            r#type: "DECLARE",
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            pub status: TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: Felt252,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
        })
    }
}

impl Serialize for DeclareTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub class_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub sender_address: &'a Felt252,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "DECLARE",
            class_hash: &self.class_hash,
            sender_address: &self.sender_address,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub class_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub sender_address: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
            sender_address: tagged.sender_address,
        })
    }
}

impl Serialize for DeclareTransactionV2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub class_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub sender_address: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub compiled_class_hash: &'a Felt252,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            max_fee: &self.max_fee,
            version: &2,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "DECLARE",
            class_hash: &self.class_hash,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub class_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub sender_address: Felt252,
            #[serde_as(as = "FeltHex")]
            pub compiled_class_hash: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &2 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
        })
    }
}

impl Serialize for DeployAccountTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub contract_address_salt: &'a Felt252,
            #[serde_as(as = "[FeltHex]")]
            pub constructor_calldata: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub class_hash: &'a Felt252,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "DEPLOY_ACCOUNT",
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployAccountTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub contract_address_salt: Felt252,
            #[serde_as(as = "Vec<FeltHex>")]
            pub constructor_calldata: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub class_hash: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeployAccountTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub status: &'a TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: &'a Felt252,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
            r#type: "DEPLOY_ACCOUNT",
            contract_address: &self.contract_address,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployAccountTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            pub status: TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: Felt252,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for DeployTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub class_hash: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub contract_address_salt: &'a Felt252,
            #[serde_as(as = "[FeltHex]")]
            pub constructor_calldata: &'a [Felt252],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            class_hash: &self.class_hash,
            version: &self.version,
            r#type: "DEPLOY",
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub class_hash: Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub contract_address_salt: Felt252,
            #[serde_as(as = "Vec<FeltHex>")]
            pub constructor_calldata: Vec<Felt252>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            class_hash: tagged.class_hash,
            version: tagged.version,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
        })
    }
}

impl Serialize for DeployTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub status: &'a TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: &'a Felt252,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
            r#type: "DEPLOY",
            contract_address: &self.contract_address,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            pub status: TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: Felt252,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for InvokeTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub status: &'a TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: &'a Felt252,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            r#type: "INVOKE",
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            pub status: TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: Felt252,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
        })
    }
}

impl Serialize for InvokeTransactionV0 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub entry_point_selector: &'a Felt252,
            #[serde_as(as = "[FeltHex]")]
            pub calldata: &'a [Felt252],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            max_fee: &self.max_fee,
            version: &0,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "INVOKE",
            contract_address: &self.contract_address,
            entry_point_selector: &self.entry_point_selector,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionV0 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
            #[serde_as(as = "FeltHex")]
            pub entry_point_selector: Felt252,
            #[serde_as(as = "Vec<FeltHex>")]
            pub calldata: Vec<Felt252>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &0 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address: tagged.contract_address,
            entry_point_selector: tagged.entry_point_selector,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for InvokeTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[FeltHex]")]
            pub signature: &'a [Felt252],
            #[serde_as(as = "FeltHex")]
            pub nonce: &'a Felt252,
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub sender_address: &'a Felt252,
            #[serde_as(as = "[FeltHex]")]
            pub calldata: &'a [Felt252],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            r#type: "INVOKE",
            sender_address: &self.sender_address,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub max_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<FeltHex>")]
            pub signature: Vec<Felt252>,
            #[serde_as(as = "FeltHex")]
            pub nonce: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub sender_address: Felt252,
            #[serde_as(as = "Vec<FeltHex>")]
            pub calldata: Vec<Felt252>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for L1HandlerTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            pub r#type: &'a str,
            #[serde_as(as = "NumAsHex")]
            pub nonce: &'a u64,
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub entry_point_selector: &'a Felt252,
            #[serde_as(as = "[FeltHex]")]
            pub calldata: &'a [Felt252],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            version: &self.version,
            r#type: "L1_HANDLER",
            nonce: &self.nonce,
            contract_address: &self.contract_address,
            entry_point_selector: &self.entry_point_selector,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for L1HandlerTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "NumAsHex")]
            pub version: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "NumAsHex")]
            pub nonce: u64,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
            #[serde_as(as = "FeltHex")]
            pub entry_point_selector: Felt252,
            #[serde_as(as = "Vec<FeltHex>")]
            pub calldata: Vec<Felt252>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "L1_HANDLER" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            version: tagged.version,
            nonce: tagged.nonce,
            contract_address: tagged.contract_address,
            entry_point_selector: tagged.entry_point_selector,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for L1HandlerTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub status: &'a TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: &'a Felt252,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            r#type: "L1_HANDLER",
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for L1HandlerTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            pub status: TransactionStatus,
            #[serde_as(as = "FeltHex")]
            pub block_hash: Felt252,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "L1_HANDLER" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
        })
    }
}

impl Serialize for PendingDeclareTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "DECLARE",
            messages_sent: &self.messages_sent,
            events: &self.events,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingDeclareTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
        })
    }
}

impl Serialize for PendingDeployAccountTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "DEPLOY_ACCOUNT",
            messages_sent: &self.messages_sent,
            events: &self.events,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingDeployAccountTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
        })
    }
}

impl Serialize for PendingDeployTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "DEPLOY",
            messages_sent: &self.messages_sent,
            events: &self.events,
            contract_address: &self.contract_address,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingDeployTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for PendingInvokeTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "INVOKE",
            messages_sent: &self.messages_sent,
            events: &self.events,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingInvokeTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
        })
    }
}

impl Serialize for PendingL1HandlerTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: &'a Felt252,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "L1_HANDLER",
            messages_sent: &self.messages_sent,
            events: &self.events,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingL1HandlerTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
            #[serde_as(as = "FeltHex")]
            pub actual_fee: Felt252,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "L1_HANDLER" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
        })
    }
}

impl Serialize for AddDeclareTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub declare_transaction: &'a BroadcastedDeclareTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            declare_transaction: &self.declare_transaction,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for AddDeclareTransactionRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub declare_transaction: &'a BroadcastedDeclareTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            declare_transaction: self.declare_transaction,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AddDeclareTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub declare_transaction: BroadcastedDeclareTransaction,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub declare_transaction: BroadcastedDeclareTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                declare_transaction: field0.declare_transaction,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                declare_transaction: object.declare_transaction,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for AddDeployAccountTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub deploy_account_transaction: &'a BroadcastedDeployAccountTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            deploy_account_transaction: &self.deploy_account_transaction,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for AddDeployAccountTransactionRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub deploy_account_transaction: &'a BroadcastedDeployAccountTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            deploy_account_transaction: self.deploy_account_transaction,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AddDeployAccountTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub deploy_account_transaction: BroadcastedDeployAccountTransaction,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub deploy_account_transaction: BroadcastedDeployAccountTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                deploy_account_transaction: field0.deploy_account_transaction,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                deploy_account_transaction: object.deploy_account_transaction,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for AddInvokeTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub invoke_transaction: &'a BroadcastedInvokeTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            invoke_transaction: &self.invoke_transaction,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for AddInvokeTransactionRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub invoke_transaction: &'a BroadcastedInvokeTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            invoke_transaction: self.invoke_transaction,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AddInvokeTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub invoke_transaction: BroadcastedInvokeTransaction,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub invoke_transaction: BroadcastedInvokeTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                invoke_transaction: field0.invoke_transaction,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                invoke_transaction: object.invoke_transaction,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for BlockHashAndNumberRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BlockHashAndNumberRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for BlockNumberRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BlockNumberRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for CallRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub request: &'a FunctionCall,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            request: &self.request,
        })?;
        seq.serialize_element(&Field1 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for CallRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub request: &'a FunctionCall,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            request: self.request,
        })?;
        seq.serialize_element(&Field1 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for CallRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub request: FunctionCall,
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub request: FunctionCall,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                request: field0.request,
                block_id: field1.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                request: object.request,
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for ChainIdRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ChainIdRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for EstimateFeeRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub request: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            request: &self.request,
        })?;
        seq.serialize_element(&Field1 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for EstimateFeeRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub request: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            request: self.request,
        })?;
        seq.serialize_element(&Field1 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for EstimateFeeRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub request: Vec<BroadcastedTransaction>,
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub request: Vec<BroadcastedTransaction>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                request: field0.request,
                block_id: field1.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                request: object.request,
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockTransactionCountRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetBlockTransactionCountRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetBlockTransactionCountRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockWithTxHashesRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetBlockWithTxHashesRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetBlockWithTxHashesRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockWithTxsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetBlockWithTxsRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetBlockWithTxsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: &self.contract_address,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetClassAtRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: self.contract_address,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetClassAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                contract_address: field1.contract_address,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                contract_address: object.contract_address,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassHashAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: &self.contract_address,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetClassHashAtRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: self.contract_address,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetClassHashAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                contract_address: field1.contract_address,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                contract_address: object.contract_address,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub class_hash: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            class_hash: &self.class_hash,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetClassRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub class_hash: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            class_hash: self.class_hash,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetClassRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            #[serde_as(as = "FeltHex")]
            pub class_hash: Felt252,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "FeltHex")]
            pub class_hash: Felt252,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                class_hash: field1.class_hash,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                class_hash: object.class_hash,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetEventsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub filter: &'a EventFilterWithPage,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            filter: &self.filter,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetEventsRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub filter: &'a EventFilterWithPage,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            filter: self.filter,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetEventsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub filter: EventFilterWithPage,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub filter: EventFilterWithPage,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                filter: field0.filter,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                filter: object.filter,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetNonceRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: &self.contract_address,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetNonceRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: self.contract_address,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetNonceRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                contract_address: field1.contract_address,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                contract_address: object.contract_address,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetStateUpdateRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetStateUpdateRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetStateUpdateRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetStorageAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub key: &'a Felt252,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            contract_address: &self.contract_address,
        })?;
        seq.serialize_element(&Field1 { key: &self.key })?;
        seq.serialize_element(&Field2 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetStorageAtRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "FeltHex")]
            pub contract_address: &'a Felt252,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "FeltHex")]
            pub key: &'a Felt252,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            contract_address: self.contract_address,
        })?;
        seq.serialize_element(&Field1 { key: self.key })?;
        seq.serialize_element(&Field2 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetStorageAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
            #[serde_as(as = "FeltHex")]
            pub key: Felt252,
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "FeltHex")]
            pub contract_address: Felt252,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "FeltHex")]
            pub key: Felt252,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field2 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field2 = serde_json::from_value::<Field2>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                contract_address: field0.contract_address,
                key: field1.key,
                block_id: field2.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                contract_address: object.contract_address,
                key: object.key,
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionByBlockIdAndIndexRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub index: &'a u64,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 { index: &self.index })?;

        seq.end()
    }
}

impl<'a> Serialize for GetTransactionByBlockIdAndIndexRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub index: &'a u64,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 { index: self.index })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetTransactionByBlockIdAndIndexRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            pub index: u64,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub index: u64,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                index: field1.index,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                index: object.index,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionByHashRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: &self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetTransactionByHashRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetTransactionByHashRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                transaction_hash: field0.transaction_hash,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionReceiptRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: &self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetTransactionReceiptRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: &'a Felt252,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetTransactionReceiptRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "FeltHex")]
            pub transaction_hash: Felt252,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                transaction_hash: field0.transaction_hash,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for PendingTransactionsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for PendingTransactionsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for SyncingRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SyncingRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}
