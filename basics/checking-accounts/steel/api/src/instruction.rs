use solana_program::msg;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    CheckingAccounts = 0,
}

instruction!(SteelInstruction, CheckingAccounts);
// CheckingAccounts instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CheckingAccounts {}

impl CheckingAccounts {
    pub fn process(accounts: &[AccountInfo]) -> ProgramResult {
        // Accounts passed in a vector must be in the expected order.
        // You can verify the list has the correct number of accounts.
        // This error will get thrown by default if you
        // reach pass the exact no of accounts specified.
        let [payer, account_to_create, account_to_change, system_program] = accounts else {
            msg!("This instruction requires 4 accounts:");
            msg!("  payer, account_to_create, account_to_change, system_program");
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // you can make sure an account is indeed a signer
        if let Err(err) = payer.is_signer() {
            msg!("The program expects payer to be a signer.");
            return Err(err);
        };

        // You can make sure an account has NOT been initialized.
        msg!("New account: {}", account_to_create.key);
        if account_to_create.lamports() != 0 {
            msg!("The program expected the account to create to not yet be initialized.");
            return Err(ProgramError::AccountAlreadyInitialized);
        };
        // (Create account...)

        // You can also make sure an account has been initialized.
        msg!("Account to change: {}", account_to_change.key);
        if account_to_change.lamports() == 0 {
            msg!("The program expected the account to change to be initialized.");
            return Err(ProgramError::UninitializedAccount);
        };

        // If we want to modify an account's data, it must be owned by our program.
        account_to_change.has_owner(&crate::ID)?;

        // You can also check pubkeys against constants.
        system_program.has_address(&system_program::ID)?;

        Ok(())
    }
}
