use crate::SteelInstruction;
use solana_program::msg;
use steel::*;

instruction!(SteelInstruction, TransferTokens);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferTokens {
    quantity: u64,
}

impl TransferTokens {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = TransferTokens::try_from_bytes(data)?;

        let [mint_account, from_associated_token_account, to_associated_token_account, owner, recipient, payer, system_program, token_program, associated_token_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // First create the token account for the user
        //
        if to_associated_token_account.lamports() == 0 {
            msg!("Creating associated token account...");
            create_associated_token_account(
                payer,
                recipient,
                to_associated_token_account,
                mint_account,
                system_program,
                token_program,
                associated_token_program,
            )?;
        } else {
            msg!("Associated token account exists.");
        }
        msg!(
            "Associated Token Address: {}",
            to_associated_token_account.key
        );

        msg!(
            "Recipient Associated Token Address: {}",
            to_associated_token_account.key
        );

        let quantity = args.quantity;
        msg!("Transferring {} tokens...", quantity);
        msg!("Mint: {}", mint_account.key);
        msg!("Owner Token Address: {}", from_associated_token_account.key);
        msg!(
            "Recipient Token Address: {}",
            to_associated_token_account.key
        );
        transfer(
            owner,
            from_associated_token_account,
            to_associated_token_account,
            token_program,
            args.quantity,
        )?;

        msg!("Token mint created successfully.");

        Ok(())
    }
}
