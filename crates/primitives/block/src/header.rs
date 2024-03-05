use alloc::sync::Arc;

use blockifier::block_context::BlockContext;
use mp_fee::ResourcePrice;
use mp_felt::Felt252Wrapper;
use mp_hashers::HasherT;
use sp_core::U256;
use starknet_api::api_core::{ChainId, ContractAddress};
use starknet_api::block::{BlockNumber, BlockTimestamp};
use starknet_api::hash::StarkHash;
use starknet_api::stdlib::collections::HashMap;
use starknet_core::types::FieldElement;

/// Block status.
///
/// The status of the block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "parity-scale-codec", derive(parity_scale_codec::Encode, parity_scale_codec::Decode))]
pub enum BlockStatus {
    #[cfg_attr(feature = "serde", serde(rename = "PENDING"))]
    Pending,
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "ACCEPTED_ON_L2"))]
    AcceptedOnL2,
    #[cfg_attr(feature = "serde", serde(rename = "ACCEPTED_ON_L1"))]
    AcceptedOnL1,
    #[cfg_attr(feature = "serde", serde(rename = "REJECTED"))]
    Rejected,
}

impl From<BlockStatus> for starknet_core::types::BlockStatus {
    fn from(status: BlockStatus) -> Self {
        match status {
            BlockStatus::Pending => starknet_core::types::BlockStatus::Pending,
            BlockStatus::AcceptedOnL2 => starknet_core::types::BlockStatus::AcceptedOnL2,
            BlockStatus::AcceptedOnL1 => starknet_core::types::BlockStatus::AcceptedOnL1,
            BlockStatus::Rejected => starknet_core::types::BlockStatus::Rejected,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "parity-scale-codec", derive(parity_scale_codec::Encode, parity_scale_codec::Decode))]
// #[cfg_attr(feature = "scale-info", derive(scale_info::TypeInfo))]
/// Starknet header definition.
pub struct Header {
    /// The hash of this block’s parent.
    pub parent_block_hash: StarkHash,
    /// The number (height) of this block.
    pub block_number: u64,
    /// The state commitment after this block.
    pub global_state_root: StarkHash,
    /// The Starknet address of the sequencer who created this block.
    pub sequencer_address: ContractAddress,
    /// The time the sequencer created this block before executing transactions
    pub block_timestamp: u64,
    /// The number of transactions in a block
    pub transaction_count: u128,
    /// A commitment to the transactions included in the block
    pub transaction_commitment: StarkHash,
    /// The number of events
    pub event_count: u128,
    /// A commitment to the events produced in this block
    pub event_commitment: StarkHash,
    /// The version of the Starknet protocol used when creating this block
    pub protocol_version: Felt252Wrapper,
    /// l1 gas price for this block
    pub l1_gas_price: ResourcePrice,
    /// Extraneous data that might be useful for running transactions
    pub extra_data: Option<U256>,
}

impl Header {
    /// Creates a new header.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        parent_block_hash: StarkHash,
        block_number: u64,
        global_state_root: StarkHash,
        sequencer_address: ContractAddress,
        block_timestamp: u64,
        transaction_count: u128,
        transaction_commitment: StarkHash,
        event_count: u128,
        event_commitment: StarkHash,
        protocol_version: Felt252Wrapper,
        l1_gas_price: ResourcePrice,
        extra_data: Option<U256>,
    ) -> Self {
        Self {
            parent_block_hash,
            block_number,
            global_state_root,
            sequencer_address,
            block_timestamp,
            transaction_count,
            transaction_commitment,
            event_count,
            event_commitment,
            protocol_version,
            l1_gas_price,
            extra_data,
        }
    }

    /// Converts to a blockifier BlockContext
    pub fn into_block_context(self, fee_token_address: ContractAddress, chain_id: ChainId) -> BlockContext {
        BlockContext {
            chain_id,
            block_number: BlockNumber(self.block_number),
            block_timestamp: BlockTimestamp(self.block_timestamp),
            sequencer_address: self.sequencer_address,
            vm_resource_fee_cost: Arc::new(HashMap::default()),
            fee_token_address,
            invoke_tx_max_n_steps: 1000000,
            validate_max_n_steps: 1000000,
            // FIXME: https://github.com/keep-starknet-strange/madara/issues/329
            gas_price: 10,
            max_recursion_depth: 50,
        }
    }

    /// Compute the hash of the header.
    pub fn hash<H: HasherT>(&self) -> Felt252Wrapper {
        if self.block_number >= 833 {
            // Computes the block hash for blocks generated after Cairo 0.7.0
            let data: &[Felt252Wrapper] = &[
                self.block_number.into(),           // block number
                self.global_state_root.into(),      // global state root
                self.sequencer_address.into(),      // sequencer address
                self.block_timestamp.into(),        // block timestamp
                self.transaction_count.into(),      // number of transactions
                self.transaction_commitment.into(), // transaction commitment
                self.event_count.into(),            // number of events
                self.event_commitment.into(),       // event commitment
                Felt252Wrapper::ZERO,               // reserved: protocol version
                Felt252Wrapper::ZERO,               // reserved: extra data
                self.parent_block_hash.into(),      // parent block hash
            ];

            H::compute_hash_on_wrappers(data)
        } else {
            // Computes the block hash for blocks generated before Cairo 0.7.0
            let data: &[Felt252Wrapper] = &[
                self.block_number.into(),
                self.global_state_root.into(),
                Felt252Wrapper::ZERO,
                Felt252Wrapper::ZERO,
                self.transaction_count.into(),
                self.transaction_commitment.into(),
                Felt252Wrapper::ZERO,
                Felt252Wrapper::ZERO,
                Felt252Wrapper::ZERO,
                Felt252Wrapper::ZERO,
                Felt252Wrapper(FieldElement::from_byte_slice_be(b"SN_MAIN").unwrap()),
                self.parent_block_hash.into(),
            ];

            // for (i, item) in data.iter().enumerate() {
            //     frame_support::log::info!("data[{}]: {:?}", i, item);
            // }

            H::compute_hash_on_wrappers(data)
        }
    }
}
