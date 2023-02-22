use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct zilchAccountState {
    pub is_initialized: bool,
    pub inputs: u8,
    pub program_code: String,
    pub program_hash: String,
}
