use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash, borsh::BorshSerialize
)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
}
