use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct zilchAccountState {
    pub is_initialized: bool,
    pub outputs: u8,
    pub proof_account: String,
    pub program_hash: String,
}
