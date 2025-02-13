use crate::errors::*;
use crate::state::Ed25519SignatureOffsets;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, sysvar};
use std::convert::TryFrom;

// Constants for serialized sizes and offsets
pub const PUBKEY_SERIALIZED_SIZE: usize = 32; // Size of a serialized public key
pub const SIGNATURE_SERIALIZED_SIZE: usize = 64; // Size of a serialized signature
pub const SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14; // Size of serialized signature offsets
pub const SIGNATURE_OFFSETS_START: usize = 0; // Starting index for signature offsets in instruction data

/// Verifies an ed25519 signature instruction.
///
/// This function checks the previous instruction in the transaction to ensure it is a valid
/// ed25519 signature verification instruction.
pub fn verify_ed25519_ix(instructions: &AccountInfo) -> Result<()> {
    // Fetch the previous instruction relative to the current one
    let verify_instruction = sysvar::instructions::get_instruction_relative(-1, &instructions)?;

    // Ensure the instruction is from the ed25519 program and has no accounts
    if verify_instruction.program_id != solana_program::ed25519_program::ID
        || verify_instruction.accounts.len() != 0
    {
        msg!("Accounts length: {:?}", verify_instruction.accounts.len());
        return Err(SignatureVerificationError::NotSigVerified.into());
    }

    // Calculate the expected end of the signature offsets data
    let data_end = SIGNATURE_OFFSETS_START.saturating_add(SIGNATURE_OFFSETS_SERIALIZED_SIZE + 2);
    if verify_instruction.data.len() < data_end {
        return Err(SignatureVerificationError::LessDataThanExpected.into());
    }

    // Extract the signature offsets data from the instruction
    let data = &verify_instruction.data[SIGNATURE_OFFSETS_START..data_end];

    // Deserialize the Ed25519 signature offsets
    let ed25519_offsets = Ed25519SignatureOffsets {
        signature_offset: u16::from_le_bytes([data[2], data[3]]),
        signature_instruction_index: u16::from_le_bytes([data[4], data[5]]),
        public_key_offset: u16::from_le_bytes([data[6], data[7]]),
        public_key_instruction_index: u16::from_le_bytes([data[8], data[9]]),
        message_data_offset: u16::from_le_bytes([data[10], data[11]]),
        message_data_size: u16::from_le_bytes([data[12], data[13]]),
        message_instruction_index: u16::from_le_bytes([data[14], data[15]]),
    };

    // Validate the public key, signature, and message data offsets
    let expected_pk_offset =
        (SIGNATURE_OFFSETS_START + SIGNATURE_OFFSETS_SERIALIZED_SIZE + 2) as u16;
    if ed25519_offsets.public_key_offset != expected_pk_offset
        || ed25519_offsets.signature_offset
            != ed25519_offsets.public_key_offset + PUBKEY_SERIALIZED_SIZE as u16
        || ed25519_offsets.message_data_offset
            != ed25519_offsets.signature_offset + SIGNATURE_SERIALIZED_SIZE as u16
    {
        return Err(SignatureVerificationError::InvalidSignatureData.into());
    }

    // Validate that all instruction indices are the same
    if ed25519_offsets.signature_instruction_index != ed25519_offsets.public_key_instruction_index
        || ed25519_offsets.signature_instruction_index != ed25519_offsets.message_instruction_index
    {
        return Err(SignatureVerificationError::InvalidSignatureData.into());
    }

    // Extract the public key, signature, and message data from the instruction
    let message_signer = Pubkey::try_from(
        &verify_instruction.data[ed25519_offsets.public_key_offset as usize
            ..ed25519_offsets.public_key_offset as usize + PUBKEY_SERIALIZED_SIZE],
    )
    .map_err(|_| SignatureVerificationError::InvalidSignatureData)?;

    let signature = &verify_instruction.data[ed25519_offsets.signature_offset as usize
        ..ed25519_offsets.signature_offset as usize + SIGNATURE_SERIALIZED_SIZE];

    let message_data = &verify_instruction.data[ed25519_offsets.message_data_offset as usize..];

    // Log debugging information
    msg!("Program ID Index: {}", verify_instruction.program_id);
    msg!("Accounts: {:?}", verify_instruction.accounts);

    let data_hex: String = verify_instruction
        .data
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    msg!("Instruction Data (Hex): {}", data_hex);

    msg!("Signature Offset: {}", ed25519_offsets.signature_offset);
    msg!(
        "Signature Instruction Index: {}",
        ed25519_offsets.signature_instruction_index
    );
    msg!("Public Key Offset: {}", ed25519_offsets.public_key_offset);
    msg!(
        "Public Key Instruction Index: {}",
        ed25519_offsets.public_key_instruction_index
    );
    msg!(
        "Message Data Offset: {}",
        ed25519_offsets.message_data_offset
    );
    msg!("Message Data Size: {}", ed25519_offsets.message_data_size);
    msg!(
        "Message Instruction Index: {}",
        ed25519_offsets.message_instruction_index
    );

    msg!("Message Data: {}", String::from_utf8_lossy(message_data));
    msg!("Message Signer: {}", message_signer);
    msg!("Signature: {:?}", signature);

    Ok(())
}
