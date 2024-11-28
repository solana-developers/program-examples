// program/tests/test.rs
#[cfg(test)]
mod tests {
    use solana_program::pubkey::Pubkey;
    use solana_program::account_info::{AccountInfo, next_account_info};
    use solana_program::entrypoint::ProgramResult;
    use solana_program::clock::Epoch;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Import the functions from your main program
    use program::example_function;
    use program::with_program::pda_rent_payer;

    // Helper function to create a test account
    fn create_account_info(
        lamports: u64,
        owner: &Pubkey,
    ) -> AccountInfo {
        let key = Pubkey::new_unique();
        let lamports = Rc::new(RefCell::new(lamports));
        let data = Rc::new(RefCell::new(vec![]));
        let owner = Rc::new(RefCell::new(*owner));
        let executable = false;
        let rent_epoch = Epoch::default();

        AccountInfo {
            key: &key,
            is_signer: false,
            is_writable: false,
            lamports,
            data,
            owner,
            executable,
            rent_epoch,
        }
    }

    #[test]
    fn transfer_with_cpi_works() {
        // Setup test accounts and initial state
        let program_id = Pubkey::new_unique();
        let owner = Pubkey::new_unique();

        let account_info = create_account_info(1000000000, &owner);
        let accounts = vec![account_info];

        // Call the function being tested
        let result = example_function(&program_id, &accounts);

        // Check the result and state changes
        assert!(result.is_ok());
        assert_eq!(*accounts[0].lamports.borrow(), 1000000000);
    }

    #[test]
    fn transfer_with_program_works() {
        // Setup test accounts and initial state
        let program_id = Pubkey::new_unique();
        let owner = Pubkey::new_unique();

        let account_info = create_account_info(0, &owner);
        let accounts = vec![account_info];

        // Call the function being tested
        let result = example_function(&program_id, &accounts);

        // Check the result and state changes
        assert!(result.is_ok());
        assert_eq!(*accounts[0].lamports.borrow(), 0);
    }
}
