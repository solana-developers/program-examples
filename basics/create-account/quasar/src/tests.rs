use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_create_account_client::CreateSystemAccountInstruction;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_create_account.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 10_000_000_000)
}

fn empty(address: Pubkey) -> Account {
    Account {
        address,
        lamports: 0,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    }
}

#[test]
fn test_create_system_account() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let new_account = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let instruction: Instruction = CreateSystemAccountInstruction {
        payer: Address::from(payer.to_bytes()),
        new_account: Address::from(new_account.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[signer(payer), empty(new_account)],
    );

    result.assert_success();

    // Verify the new account exists and is owned by the system program.
    let account = result.account(&new_account).unwrap();
    assert_eq!(account.owner, system_program, "account should be system-owned");
    assert!(account.lamports > 0, "account should have rent-exempt lamports");
    assert_eq!(account.data.len(), 0, "account should have zero data");
}
