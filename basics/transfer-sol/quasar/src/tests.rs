use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_transfer_sol_client::{
    TransferSolWithCpiInstruction, TransferSolWithProgramInstruction,
};

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_transfer_sol.so");
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

#[test]
fn test_transfer_sol_with_cpi() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;
    let amount = 1_000_000_000; // 1 SOL

    let instruction: Instruction = TransferSolWithCpiInstruction {
        payer: Address::from(payer.to_bytes()),
        recipient: Address::from(recipient.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
        amount,
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[signer(payer), system_account(recipient, 0)],
    );

    result.assert_success();

    // Verify balances after transfer.
    let payer_account = result.account(&payer).unwrap();
    assert_eq!(payer_account.lamports, 10_000_000_000 - amount);

    let recipient_account = result.account(&recipient).unwrap();
    assert_eq!(recipient_account.lamports, amount);
}

#[test]
fn test_transfer_sol_with_program() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();
    let amount = 500_000_000; // 0.5 SOL

    // The payer must be owned by our program for direct lamport manipulation.
    let payer_account = Account {
        address: payer,
        lamports: 2_000_000_000,
        data: vec![],
        owner: Pubkey::from(crate::ID),
        executable: false,
    };

    let recipient_account = Account {
        address: recipient,
        lamports: 1_000_000_000,
        data: vec![],
        owner: Pubkey::from(crate::ID),
        executable: false,
    };

    let instruction: Instruction = TransferSolWithProgramInstruction {
        payer: Address::from(payer.to_bytes()),
        recipient: Address::from(recipient.to_bytes()),
        amount,
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[payer_account, recipient_account],
    );

    result.assert_success();

    // Verify balances.
    let payer_after = result.account(&payer).unwrap();
    assert_eq!(payer_after.lamports, 2_000_000_000 - amount);

    let recipient_after = result.account(&recipient).unwrap();
    assert_eq!(recipient_after.lamports, 1_000_000_000 + amount);
}
