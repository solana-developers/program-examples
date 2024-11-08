use super::SteelInstruction;
use crate::state::MintAuthorityPda;
use solana_program::msg;
use steel::*;

instruction!(SteelInstruction, Init);
/// Init Instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Init {}

impl Init {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [mint_authority, payer, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        mint_authority.has_seeds(&[MintAuthorityPda::SEED_PREFIX], program_id)?;

        msg!("Creating mint authority PDA...");
        msg!("Mint Authority: {}", &mint_authority.key);
        create_account::<MintAuthorityPda>(
            mint_authority,
            system_program,
            payer,
            program_id,
            &[MintAuthorityPda::SEED_PREFIX],
        )?;

        msg!("Token mint created successfully.");

        Ok(())
    }
}
