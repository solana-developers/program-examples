use crate::state::MintAuthorityPda;
use mpl_token_metadata::instructions as mpl_instruction;
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
        let [mint_account, metadata_account, edition_account, mint_authority, associated_token_account, payer, rent, system_program, token_program, associated_token_program, _token_metadata_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        mint_authority.has_seeds(&[MintAuthorityPda::SEED_PREFIX], &crate::ID)?;

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
        let ix = &mpl_instruction::CreateMasterEditionV3 {
            edition: *edition_account.key,
            metadata: *metadata_account.key,
            mint: *mint_account.key,
            mint_authority: *mint_authority.key,
            payer: *payer.key,
            rent: None,
            system_program: *system_program.key,
            token_program: *token_program.key,
            update_authority: *mint_authority.key,
        }
        .instruction(mpl_instruction::CreateMasterEditionV3InstructionArgs { max_supply: None });

        invoke_signed(
            ix,
            &[
                edition_account.clone(),
                mint_account.clone(),
                payer.clone(),
                mint_authority.clone(),
                mint_authority.clone(),
                metadata_account.clone(),
                token_program.clone(),
                system_program.clone(),
                rent.clone(),
            ],
            &crate::ID,
            &[MintAuthorityPda::SEED_PREFIX],
        )?;

        msg!("NFT minted successfully.");

        Ok(())
    }
}
