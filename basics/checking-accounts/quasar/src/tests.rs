use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_checking_accounts_client::CheckAccountsInstruction;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_checking_accounts.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 10_000_000_000)
}

fn system_account(address: Pubkey, lamports: u64) -> Account {
    Account {
        address,
        lamports,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    }
}

fn program_owned_account(address: Pubkey, lamports: u64) -> Account {
    Account {
        address,
        lamports,
        data: vec![0u8; 32],
        owner: Pubkey::from(crate::ID),
        executable: false,
    }
}

#[test]
fn test_check_accounts_succeeds() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let account_to_create = Pubkey::new_unique();
    let account_to_change = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let instruction: Instruction = CheckAccountsInstruction {
        payer: Address::from(payer.to_bytes()),
        account_to_create: Address::from(account_to_create.to_bytes()),
        account_to_change: Address::from(account_to_change.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(payer),
            system_account(account_to_create, 0),
            program_owned_account(account_to_change, 1_000_000),
        ],
    );

    result.assert_success();
}
