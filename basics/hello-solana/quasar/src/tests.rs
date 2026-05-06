use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_hello_solana_client::HelloInstruction;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_hello_solana.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

#[test]
fn test_hello() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();

    let instruction: Instruction = HelloInstruction {
        payer: Address::from(payer.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[Account {
            address: payer,
            lamports: 1_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();
}
