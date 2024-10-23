#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use light_poseidon::{Poseidon, PoseidonHasher};
use ark_bn254::Fr; // Import the BN254 field type
use ark_ff::PrimeField;
use std::convert::TryInto; // For converting data types

declare_id!("DgoL5J44aspizyUs9fcnpGEUJjWTLJRCfx8eYtUMYczf");

#[error_code]
pub enum ProcessingInstructionsError {
    #[msg("Poseidon initialization error")]
    PoseidonInitializationError,
    #[msg("Hashing error")]
    HashingError,
}

#[program]
pub mod processing_instructions_poseidon {
    use super::*;

    pub fn go_to_park(_ctx: Context<Park>, name: String, height: u32) -> Result<()> {
        // Initialize Poseidon hasher with parameters
        let mut poseidon = Poseidon::<Fr>::new_circom(2).map_err(|_| {
            msg!("Poseidon initialization error");
            ProcessingInstructionsError::PoseidonInitializationError
        })?;

        // Convert inputs to prime fields
        let name_bytes = name.as_bytes();
        let name_field = Fr::from_be_bytes_mod_order(&name_bytes.try_into().unwrap_or_else(|_| [0u8; 32]));
        let height_field = Fr::from(height);

        // Calculate the Poseidon hash
        let hash = poseidon.hash(&[name_field, height_field]).map_err(|_| {
            msg!("Hashing error");
            ProcessingInstructionsError::HashingError
        })?;

        // Output messages
        msg!("Welcome to the park, {}!", name);
        if height > 5 {
            msg!("You are tall enough to ride this ride. Congratulations.");
        } else {
            msg!("You are NOT tall enough to ride this ride. Sorry mate.");
        }

        msg!("Poseidon hash of the name and height: {:?}", hash);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Park {}
