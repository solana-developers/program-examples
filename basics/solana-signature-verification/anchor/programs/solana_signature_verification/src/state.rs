use anchor_lang::prelude::*;
pub const ESCROW_SEED: &[u8] = b"instruction-introspection-seed";

use bytemuck::{Pod, Zeroable};
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Ed25519SignatureOffsets {
    pub signature_offset: u16,
    pub signature_instruction_index: u16,
    pub public_key_offset: u16,
    pub public_key_instruction_index: u16,
    pub message_data_offset: u16,
    pub message_data_size: u16,
    pub message_instruction_index: u16,
}
#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    pub unlock_price: u64,
    pub escrow_amount: u64,
}
