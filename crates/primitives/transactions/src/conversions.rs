use alloc::sync::Arc;
use alloc::vec::Vec;

use blockifier::execution::contract_class::ContractClass;
use blockifier::transaction::objects::TransactionExecutionResult;
use blockifier::transaction::transactions as btx;
use mp_felt::Felt252Wrapper;
use mp_hashers::HasherT;
use starknet_api::api_core::Nonce;
use starknet_api::hash::StarkFelt;
use starknet_api::transaction as sttx;
use starknet_api::transaction::{Fee, TransactionVersion};

use super::compute_hash::ComputeTransactionHash;
use super::{
    DeclareTransaction, DeclareTransactionV0, DeclareTransactionV1, DeclareTransactionV2, DeployAccountTransaction,
    DeployTransaction, HandleL1MessageTransaction, InvokeTransaction, InvokeTransactionV0, InvokeTransactionV1,
};

impl DeclareTransactionV0 {
    fn try_into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        contract_class: ContractClass,
        offset_version: bool,
    ) -> TransactionExecutionResult<btx::DeclareTransaction> {
        let transaction_hash = self.compute_hash::<H>(chain_id, offset_version, None);

        btx::DeclareTransaction::new(
            sttx::DeclareTransaction::V0(sttx::DeclareTransactionV0V1 {
                max_fee: sttx::Fee(self.max_fee),
                signature: vec_of_felt_to_signature(&self.signature),
                nonce: self.nonce.into(),
                class_hash: self.class_hash.into(),
                sender_address: self.sender_address.into(),
            }),
            transaction_hash.into(),
            contract_class,
        )
    }

    pub fn from_starknet(inner: starknet_api::transaction::DeclareTransactionV0V1) -> Self {
        Self {
            nonce: inner.nonce.0.into(),
            max_fee: inner.max_fee.0.into(),
            signature: inner.signature.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
            sender_address: inner.sender_address.into(),
            class_hash: inner.class_hash.into(),
        }
    }
}

impl DeclareTransactionV1 {
    fn try_into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        contract_class: ContractClass,
        offset_version: bool,
    ) -> TransactionExecutionResult<btx::DeclareTransaction> {
        let transaction_hash = self.compute_hash::<H>(chain_id, offset_version, None);

        btx::DeclareTransaction::new(
            sttx::DeclareTransaction::V1(sttx::DeclareTransactionV0V1 {
                max_fee: sttx::Fee(self.max_fee),
                signature: vec_of_felt_to_signature(&self.signature),
                nonce: self.nonce.into(),
                class_hash: self.class_hash.into(),
                sender_address: self.sender_address.into(),
            }),
            transaction_hash.into(),
            contract_class,
        )
    }

    pub fn from_starknet(inner: starknet_api::transaction::DeclareTransactionV0V1) -> Self {
        Self {
            nonce: inner.nonce.0.into(),
            max_fee: inner.max_fee.0.into(),
            signature: inner.signature.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
            sender_address: inner.sender_address.into(),
            class_hash: inner.class_hash.into(),
            offset_version: false,
        }
    }
}

impl DeclareTransactionV2 {
    fn try_into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        contract_class: ContractClass,
        offset_version: bool,
    ) -> TransactionExecutionResult<btx::DeclareTransaction> {
        let transaction_hash = self.compute_hash::<H>(chain_id, offset_version, None);

        btx::DeclareTransaction::new(
            sttx::DeclareTransaction::V2(sttx::DeclareTransactionV2 {
                max_fee: sttx::Fee(self.max_fee),
                signature: vec_of_felt_to_signature(&self.signature),
                nonce: self.nonce.into(),
                class_hash: self.class_hash.into(),
                compiled_class_hash: self.compiled_class_hash.into(),
                sender_address: self.sender_address.into(),
            }),
            transaction_hash.into(),
            contract_class,
        )
    }

    pub fn from_starknet(inner: starknet_api::transaction::DeclareTransactionV2) -> Self {
        Self {
            nonce: inner.nonce.0.into(),
            max_fee: inner.max_fee.0.into(),
            signature: inner.signature.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
            sender_address: inner.sender_address.into(),
            class_hash: inner.class_hash.into(),
            compiled_class_hash: inner.compiled_class_hash.into(),
            offset_version: false,
        }
    }
}

impl DeclareTransaction {
    pub fn try_into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        contract_class: ContractClass,
        offset_version: bool,
    ) -> TransactionExecutionResult<btx::DeclareTransaction> {
        match self {
            DeclareTransaction::V0(tx) => tx.try_into_executable::<H>(chain_id, contract_class, offset_version),
            DeclareTransaction::V1(tx) => tx.try_into_executable::<H>(chain_id, contract_class, offset_version),
            DeclareTransaction::V2(tx) => tx.try_into_executable::<H>(chain_id, contract_class, offset_version),
        }
    }
}

impl InvokeTransactionV0 {
    pub fn into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        offset_version: bool,
    ) -> btx::InvokeTransaction {
        let transaction_hash = self.compute_hash::<H>(chain_id, offset_version, None);

        btx::InvokeTransaction {
            tx: sttx::InvokeTransaction::V0(sttx::InvokeTransactionV0 {
                max_fee: sttx::Fee(self.max_fee),
                signature: vec_of_felt_to_signature(&self.signature),
                contract_address: self.contract_address.into(),
                entry_point_selector: self.entry_point_selector.into(),
                calldata: vec_of_felt_to_calldata(&self.calldata),
            }),
            tx_hash: transaction_hash.into(),
        }
    }

    pub fn from_starknet(inner: starknet_api::transaction::InvokeTransactionV0) -> Self {
        Self {
            max_fee: inner.max_fee.0.into(),
            signature: inner.signature.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
            contract_address: inner.contract_address.into(),
            entry_point_selector: inner.entry_point_selector.into(),
            calldata: inner.calldata.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
        }
    }
}

impl InvokeTransactionV1 {
    pub fn into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        offset_version: bool,
    ) -> btx::InvokeTransaction {
        let transaction_hash = self.compute_hash::<H>(chain_id, offset_version, None);

        btx::InvokeTransaction {
            tx: sttx::InvokeTransaction::V1(sttx::InvokeTransactionV1 {
                max_fee: sttx::Fee(self.max_fee),
                signature: vec_of_felt_to_signature(&self.signature),
                nonce: self.nonce.into(),
                calldata: vec_of_felt_to_calldata(&self.calldata),
                sender_address: self.sender_address.into(),
            }),
            tx_hash: transaction_hash.into(),
        }
    }

    pub fn from_starknet(inner: starknet_api::transaction::InvokeTransactionV1) -> Self {
        Self {
            max_fee: inner.max_fee.0,
            signature: inner.signature.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
            nonce: inner.nonce.into(),
            sender_address: inner.sender_address.into(),
            calldata: inner.calldata.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
            offset_version: false,
        }
    }
}

impl InvokeTransaction {
    pub fn into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        offset_version: bool,
    ) -> btx::InvokeTransaction {
        match self {
            InvokeTransaction::V0(tx) => tx.into_executable::<H>(chain_id, offset_version),
            InvokeTransaction::V1(tx) => tx.into_executable::<H>(chain_id, offset_version),
        }
    }
}

impl DeployTransaction {
    pub fn into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        offset_version: bool,
    ) -> btx::DeployTransaction {
        let account_address = self.get_account_address();
        let transaction_hash: Felt252Wrapper = self
            .compute_hash_given_contract_address::<H>(chain_id.into(), account_address, offset_version, None)
            .into();
        let contract_address: Felt252Wrapper = account_address.into();
        // let transaction_hash = self.compute_hash::<H>(chain_id, offset_version, None);

        btx::DeployTransaction {
            tx: sttx::DeployTransaction {
                version: sttx::TransactionVersion(StarkFelt::from(1u128)),
                class_hash: self.class_hash.into(),
                contract_address_salt: self.contract_address_salt.into(),
                constructor_calldata: vec_of_felt_to_calldata(&self.constructor_calldata),
            },
            tx_hash: transaction_hash.into(),
            contract_address: contract_address.into(),
        }
    }

    pub fn from_starknet(inner: starknet_api::transaction::DeployTransaction) -> Self {
        Self {
            class_hash: inner.class_hash.into(),
            version: inner.version.into(),
            contract_address_salt: inner.contract_address_salt.into(),
            constructor_calldata: inner.constructor_calldata.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
        }
    }
}

impl DeployAccountTransaction {
    pub fn into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        offset_version: bool,
    ) -> btx::DeployAccountTransaction {
        let account_address = self.get_account_address();
        let transaction_hash: Felt252Wrapper =
            self.compute_hash_given_contract_address::<H>(chain_id.into(), account_address, offset_version).into();
        let contract_address: Felt252Wrapper = account_address.into();

        btx::DeployAccountTransaction {
            tx: sttx::DeployAccountTransaction {
                max_fee: sttx::Fee(self.max_fee),
                version: sttx::TransactionVersion(StarkFelt::from(1u128)),
                signature: vec_of_felt_to_signature(&self.signature),
                nonce: self.nonce.into(),
                class_hash: self.class_hash.into(),
                contract_address_salt: self.contract_address_salt.into(),
                constructor_calldata: vec_of_felt_to_calldata(&self.constructor_calldata),
            },
            tx_hash: transaction_hash.into(),
            contract_address: contract_address.into(),
        }
    }

    pub fn from_starknet(inner: starknet_api::transaction::DeployAccountTransaction) -> Self {
        Self {
            max_fee: inner.max_fee.0,
            signature: inner.signature.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
            nonce: inner.nonce.into(),
            contract_address_salt: inner.contract_address_salt.into(),
            constructor_calldata: inner.constructor_calldata.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
            class_hash: inner.class_hash.into(),
            offset_version: false,
        }
    }
}

impl HandleL1MessageTransaction {
    pub fn into_executable<H: HasherT>(
        &self,
        chain_id: Felt252Wrapper,
        paid_fee_on_l1: Fee,
        offset_version: bool,
    ) -> btx::L1HandlerTransaction {
        let transaction_hash = self.compute_hash::<H>(chain_id, offset_version, None);

        let tx = sttx::L1HandlerTransaction {
            version: TransactionVersion(StarkFelt::from(0u8)),
            nonce: Nonce(StarkFelt::from(self.nonce)),
            contract_address: self.contract_address.into(),
            entry_point_selector: self.entry_point_selector.into(),
            calldata: vec_of_felt_to_calldata(&self.calldata),
        };

        btx::L1HandlerTransaction { tx, paid_fee_on_l1, tx_hash: transaction_hash.into() }
    }

    pub fn from_starknet(inner: starknet_api::transaction::L1HandlerTransaction) -> Self {
        Self {
            nonce: u64::try_from(inner.nonce.0).unwrap(),
            contract_address: inner.contract_address.into(),
            entry_point_selector: inner.entry_point_selector.0.into(),
            calldata: inner.calldata.0.iter().map(|felt| Felt252Wrapper::from(*felt)).collect(),
        }
    }
}

fn vec_of_felt_to_signature(felts: &[Felt252Wrapper]) -> sttx::TransactionSignature {
    sttx::TransactionSignature(felts.iter().map(|&f| f.into()).collect())
}

fn vec_of_felt_to_calldata(felts: &[Felt252Wrapper]) -> sttx::Calldata {
    sttx::Calldata(Arc::new(felts.iter().map(|&f| f.into()).collect()))
}

impl From<sttx::DeclareTransactionV0V1> for DeclareTransactionV0 {
    fn from(remote: sttx::DeclareTransactionV0V1) -> Self {
        DeclareTransactionV0 {
            max_fee: remote.max_fee.0, // This assumes the `Fee` type's internal value can be accessed this way.
            signature: signature_to_vec_of_felt(&remote.signature),
            nonce: remote.nonce.into(),
            class_hash: remote.class_hash.into(),
            sender_address: remote.sender_address.into(),
        }
    }
}

// Conversion for DeclareTransactionV1
impl From<sttx::DeclareTransactionV0V1> for DeclareTransactionV1 {
    fn from(remote: sttx::DeclareTransactionV0V1) -> Self {
        DeclareTransactionV1 {
            max_fee: remote.max_fee.0, // Assuming `Fee` type's inner value can be accessed with `.0`.
            signature: signature_to_vec_of_felt(&remote.signature),
            nonce: remote.nonce.into(),
            class_hash: remote.class_hash.into(),
            sender_address: remote.sender_address.into(),
            offset_version: false,
        }
    }
}

// Conversion for DeclareTransactionV2
impl From<sttx::DeclareTransactionV2> for DeclareTransactionV2 {
    fn from(remote: sttx::DeclareTransactionV2) -> Self {
        DeclareTransactionV2 {
            max_fee: remote.max_fee.0, // Assuming `Fee` type's inner value can be accessed with `.0`.
            signature: signature_to_vec_of_felt(&remote.signature),
            nonce: remote.nonce.into(),
            class_hash: remote.class_hash.into(),
            compiled_class_hash: remote.compiled_class_hash.into(),
            sender_address: remote.sender_address.into(),
            offset_version: false,
        }
    }
}

// Utility function to convert `TransactionSignature` into a vector of `Felt252Wrapper`
fn signature_to_vec_of_felt(sig: &sttx::TransactionSignature) -> Vec<Felt252Wrapper> {
    sig.0.iter().map(|&f| Felt252Wrapper::from(f)).collect()
}
