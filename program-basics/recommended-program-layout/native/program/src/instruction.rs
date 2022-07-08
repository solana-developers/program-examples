use borsh::{BorshDeserialize, BorshSerialize};

// the various types of instruction data possible

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    pub name: String,
    pub height: u32,
}