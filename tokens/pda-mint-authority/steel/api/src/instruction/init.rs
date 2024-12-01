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
    pub fn process(accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [mint_authority, payer, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        let (mint_authority_key, bump) =
            Pubkey::find_program_address(&[MintAuthorityPda::SEED_PREFIX], &crate::ID);

        mint_authority.has_address(&mint_authority_key)?;

        msg!("Creating mint authority PDA...");
        msg!("Mint Authority: {}", &mint_authority.key);
        create_account_with_bump::<MintAuthorityPda>(
            mint_authority,
            system_program,
            payer,
            &crate::ID,
            &[MintAuthorityPda::SEED_PREFIX],
            bump,
        )?;

        let mint_authority_account =
            mint_authority.as_account_mut::<MintAuthorityPda>(&crate::ID)?;

        mint_authority_account.bump = bump;

        Ok(())
    }
}
