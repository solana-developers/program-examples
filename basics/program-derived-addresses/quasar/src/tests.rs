use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_program_derived_addresses_client::{
    CreatePageVisitsInstruction, IncrementPageVisitsInstruction,
};

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_program_derived_addresses.so");
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
fn test_create_page_visits() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    // Derive the page visits PDA from ["page_visits", payer].
    let (page_visits, _) = Pubkey::find_program_address(
        &[b"page_visits", payer.as_ref()],
        &Pubkey::from(crate::ID),
    );

    let instruction: Instruction = CreatePageVisitsInstruction {
        payer: Address::from(payer.to_bytes()),
        page_visits: Address::from(page_visits.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[signer(payer), empty(page_visits)],
    );

    result.assert_success();

    // Verify the page visits account was created with count = 0.
    let pv_account = result.account(&page_visits).unwrap();
    // Data: 1 byte discriminator (1) + 8 bytes u64 (0)
    assert_eq!(pv_account.data.len(), 9);
    assert_eq!(pv_account.data[0], 1); // discriminator
    assert_eq!(&pv_account.data[1..], &[0u8; 8]); // page_visits = 0
}

#[test]
fn test_increment_page_visits() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    // Derive the page visits PDA.
    let (page_visits, _) = Pubkey::find_program_address(
        &[b"page_visits", payer.as_ref()],
        &Pubkey::from(crate::ID),
    );

    // First, create the page visits account.
    let create_instruction: Instruction = CreatePageVisitsInstruction {
        payer: Address::from(payer.to_bytes()),
        page_visits: Address::from(page_visits.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &create_instruction,
        &[signer(payer), empty(page_visits)],
    );
    result.assert_success();

    // Grab updated page_visits account after init.
    let pv_after_init = result.account(&page_visits).unwrap().clone();

    // Increment page visits.
    let increment_instruction: Instruction = IncrementPageVisitsInstruction {
        user: Address::from(payer.to_bytes()),
        page_visits: Address::from(page_visits.to_bytes()),
    }
    .into();

    // The user account is only used for PDA derivation, not as a signer.
    let user_account = Account {
        address: payer,
        lamports: 10_000_000_000,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    };

    let result = svm.process_instruction(
        &increment_instruction,
        &[user_account, pv_after_init],
    );
    result.assert_success();

    // Verify page_visits = 1.
    let pv_account = result.account(&page_visits).unwrap();
    let count_bytes: [u8; 8] = pv_account.data[1..9].try_into().unwrap();
    let count = u64::from_le_bytes(count_bytes);
    assert_eq!(count, 1, "page_visits should be 1 after one increment");
}
