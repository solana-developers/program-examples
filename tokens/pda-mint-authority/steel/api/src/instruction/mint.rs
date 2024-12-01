use crate::state::MintAuthorityPda;
use mpl_token_metadata::instructions::{self as mpl_instruction};
use solana_program::msg;
use steel::*;

use super::SteelInstruction;

instruction!(SteelInstruction, MintTo);
// MintTo instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintTo {}

impl MintTo {
    pub fn process(accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [mint_account, metadata_account, edition_account, mint_authority, associated_token_account, payer, _rent, system_program, token_program, associated_token_program, token_metadata_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        mint_authority.has_seeds(&[MintAuthorityPda::SEED_PREFIX], &crate::ID)?;

        let bump = mint_authority
            .as_account::<MintAuthorityPda>(&crate::ID)?
            .bump;

        // First create the token account for the user
        //
        if associated_token_account.lamports() == 0 {
            msg!("Creating associated token account...");
            create_associated_token_account(
                payer,
                payer,
                associated_token_account,
                mint_account,
                system_program,
                token_program,
                associated_token_program,
            )?;
        } else {
            msg!("Associated token account exists.");
        }
        msg!("Associated Token Address: {}", associated_token_account.key);

        msg!("Minting NFT to associated token account...");
        mint_to_signed(
            mint_account,
            associated_token_account,
            mint_authority,
            token_program,
            1,
            &[MintAuthorityPda::SEED_PREFIX],
        )?;

        // We can make this a Limited Edition NFT through Metaplex,
        // which will disable minting by setting the Mint & Freeze Authorities to the
        // Edition Account.
        //
        mpl_instruction::CreateMasterEditionV3Cpi {
            __program: token_metadata_program,
            __args: mpl_instruction::CreateMasterEditionV3InstructionArgs { max_supply: None },
            edition: edition_account,
            metadata: metadata_account,
            mint: mint_account,
            mint_authority,
            payer,
            rent: None,
            system_program,
            token_program,
            update_authority: mint_authority,
        }
        .invoke_signed(&[&[MintAuthorityPda::SEED_PREFIX, &[bump]]])?;

        msg!("NFT minted successfully.");

        Ok(())
    }
}
