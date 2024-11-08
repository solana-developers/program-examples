use super::SteelInstruction;
use solana_program::{msg, program::invoke};
use spl_token::instruction::{self as token_instruction};
use steel::*;

instruction!(SteelInstruction, MintTo);
/// MintTo Instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintTo {
    quantity: u64,
}

impl MintTo {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = MintTo::try_from_bytes(data)?;

        let [mint_account, mint_authority, associated_token_account, payer, system_program, token_program, associated_token_program] =
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

        let quantity = args.quantity;
        msg!("Minting {} tokens to associated token account...", quantity);

        invoke(
            &token_instruction::mint_to(
                token_program.key,
                mint_account.key,
                associated_token_account.key,
                mint_authority.key,
                &[mint_authority.key],
                args.quantity,
            )?,
            &[
                mint_account.clone(),
                mint_authority.clone(),
                associated_token_account.clone(),
                token_program.clone(),
            ],
        )?;

        msg!("Tokens minted to wallet successfully.");
        Ok(())
    }
}
