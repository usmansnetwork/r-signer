use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Hash256 = primitive_types::H256;

#[derive(Error, Debug)]
pub enum SigningRootError {
    #[error("Unexpected Error in converting vector to array")]
    VectorConversionError,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct BeaconBlockHeader {
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body_root: Hash256,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DomainType {
    BeaconProposer,
    BeaconAttester,
    Randao,
    Deposit,
    VoluntaryExit,
    SelectionProof,
    AggregateAndProof,
    ApplicationBuilder,
    // altair
    SyncCommittee,
    SyncCommitteeSelectionProof,
    ContributionAndProof,
    // Capella
    DomainBlsToExecutionChange,
    // Deneb
    DomainBlobSidecar,
}

impl DomainType {
    pub fn value(&self) -> [u8; 4] {
        match self {
            DomainType::BeaconProposer => [0, 0, 0, 0],
            DomainType::BeaconAttester => [1, 0, 0, 0],
            DomainType::Randao => [2, 0, 0, 0],
            DomainType::Deposit => [3, 0, 0, 0],
            DomainType::VoluntaryExit => [4, 0, 0, 0],
            DomainType::SelectionProof => [5, 0, 0, 0],
            DomainType::AggregateAndProof => [6, 0, 0, 0],
            DomainType::ApplicationBuilder => [0, 0, 0, 1],
            DomainType::SyncCommittee => [7, 0, 0, 0],
            DomainType::SyncCommitteeSelectionProof => [8, 0, 0, 0],
            DomainType::ContributionAndProof => [9, 0, 0, 0],
            DomainType::DomainBlsToExecutionChange => [10, 0, 0, 0],
            DomainType::DomainBlobSidecar => [11, 0, 0, 0],
        }
    }
}

pub trait Domain {
    fn compute_domain(&self, domain_type: &DomainType, epoch: u64) -> Result<Hash256>;
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Fork {
    #[serde(with = "serde_utils::bytes_4_hex")]
    pub previous_version: [u8; 4],
    #[serde(with = "serde_utils::bytes_4_hex")]
    pub current_version: [u8; 4],
    #[serde(with = "serde_utils::quoted_u64")]
    pub epoch: u64,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForkInfo {
    pub fork: Fork,
    pub genesis_validators_root: Hash256,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AttestationData {
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub index: u64,
    pub beacon_block_root: Hash256,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AggregationSlot {
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    #[serde(with = "serde_utils::quoted_u64")]
    pub epoch: u64,
    pub root: Hash256,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Attestation {
    #[serde(with = "serde_utils::hex_vec")]
    pub aggregation_bits: Vec<u8>,
    pub data: AttestationData,
    #[serde(with = "serde_utils::hex_vec")]
    pub signature: Vec<u8>,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AggregateAndProof {
    #[serde(with = "serde_utils::quoted_u64")]
    pub aggregator_index: u64,
    pub aggregate: Attestation,
    #[serde(with = "serde_utils::hex_vec")]
    pub selection_proof: Vec<u8>,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RandaoReveal {
    #[serde(with = "serde_utils::quoted_u64")]
    pub epoch: u64,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct VoluntaryExit {
    #[serde(with = "serde_utils::quoted_u64")]
    pub epoch: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub validator_index: u64,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct DepositMessage {
    #[serde(with = "serde_utils::hex_vec")]
    pub pubkey: Vec<u8>, // TODO: Customize for asserting length 48
    pub withdrawal_credentials: Hash256,
    #[serde(with = "serde_utils::quoted_u64")]
    pub amount: u64,
    #[serde(with = "serde_utils::bytes_4_hex")]
    pub genesis_fork_version: [u8; 4],
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct ValidatorRegistration {
    #[serde(with = "serde_utils::hex_vec")]
    pub fee_recipient: Vec<u8>, // TODO: Customize for asserting length 20
    #[serde(with = "serde_utils::quoted_u64")]
    pub gas_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub timestamp: u64,
    #[serde(with = "serde_utils::hex_vec")]
    pub pubkey: Vec<u8>, // TODO: Customize for asserting length 48
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SyncCommitteeMessage {
    pub beacon_block_root: Hash256,
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SyncAggregatorSelectionData {
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub subcommittee_index: u64,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SyncCommitteeContribution {
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
    pub beacon_block_root: Hash256,
    #[serde(with = "serde_utils::quoted_u64")]
    pub subcommittee_index: u64,
    #[serde(with = "serde_utils::hex_vec")]
    pub aggregation_bits: Vec<u8>,
    #[serde(with = "serde_utils::hex_vec")]
    pub signature: Vec<u8>,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct ContributionAndProof {
    #[serde(with = "serde_utils::quoted_u64")]
    pub aggregator_index: u64,
    pub contribution: SyncCommitteeContribution,
    #[serde(with = "serde_utils::hex_vec")]
    pub selection_proof: Vec<u8>, //bls signature
}
