use crate::SteelInstruction;
use mpl_token_metadata::instructions as mpl_instruction;
use solana_program::{msg, program::invoke};
use spl_token::instruction::{self as token_instruction};
use steel::*;

instruction!(SteelInstruction, MintNft);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintNft {}

impl MintNft {
    pub fn process(accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [mint_account, metadata_account, edition_account, mint_authority, associated_token_account, payer, _rent, system_program, token_program, associated_token_program, token_metadata_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

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

        // Mint the NFT to the user's wallet
        //
        msg!("Minting NFT to associated token account...");
        invoke(
            &token_instruction::mint_to(
                token_program.key,
                mint_account.key,
                associated_token_account.key,
                mint_authority.key,
                &[mint_authority.key],
                1,
            )?,
            &[
                mint_account.clone(),
                mint_authority.clone(),
                associated_token_account.clone(),
                token_program.clone(),
            ],
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
        .invoke()?;

        msg!("NFT minted successfully.");

        Ok(())
    }
}
