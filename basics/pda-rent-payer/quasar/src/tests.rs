use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_pda_rent_payer_client::{InitRentVaultInstruction, CreateNewAccountInstruction};

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_pda_rent_payer.so");
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
fn test_init_rent_vault() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;
    let fund_amount: u64 = 5_000_000_000; // 5 SOL

    // Derive the rent vault PDA from ["rent_vault"].
    let (rent_vault, _) = Pubkey::find_program_address(
        &[b"rent_vault"],
        &Pubkey::from(crate::ID),
    );

    let instruction: Instruction = InitRentVaultInstruction {
        payer: Address::from(payer.to_bytes()),
        rent_vault: Address::from(rent_vault.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
        fund_lamports: fund_amount,
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[signer(payer), empty(rent_vault)],
    );

    result.assert_success();

    // Verify the vault received funds.
    let vault_account = result.account(&rent_vault).unwrap();
    assert_eq!(vault_account.lamports, fund_amount);
}

#[test]
fn test_create_new_account_from_vault() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;
    let fund_amount: u64 = 5_000_000_000;

    // Derive the rent vault PDA.
    let (rent_vault, _) = Pubkey::find_program_address(
        &[b"rent_vault"],
        &Pubkey::from(crate::ID),
    );

    // Step 1: Fund the rent vault.
    let init_instruction: Instruction = InitRentVaultInstruction {
        payer: Address::from(payer.to_bytes()),
        rent_vault: Address::from(rent_vault.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
        fund_lamports: fund_amount,
    }
    .into();

    let result = svm.process_instruction(
        &init_instruction,
        &[signer(payer), empty(rent_vault)],
    );
    result.assert_success();

    // Grab updated vault account.
    let vault_after_init = result.account(&rent_vault).unwrap().clone();

    // Step 2: Create a new account funded by the vault.
    let new_account = Pubkey::new_unique();

    let create_instruction: Instruction = CreateNewAccountInstruction {
        new_account: Address::from(new_account.to_bytes()),
        rent_vault: Address::from(rent_vault.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
    }
    .into();

    // The new_account must be a signer but have zero lamports (not yet created).
    let new_account_entry = Account {
        address: new_account,
        lamports: 0,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    };

    let result = svm.process_instruction(
        &create_instruction,
        &[new_account_entry, vault_after_init],
    );

    result.assert_success();

    // Verify the new account was created.
    let new_acc = result.account(&new_account).unwrap();
    assert_eq!(new_acc.owner, system_program, "new account should be system-owned");
    assert!(new_acc.lamports > 0, "new account should have rent-exempt lamports");
    assert_eq!(new_acc.data.len(), 0, "new account should have zero data");

    // Verify the vault balance decreased.
    let vault_after = result.account(&rent_vault).unwrap();
    assert!(
        vault_after.lamports < fund_amount,
        "vault should have less lamports after paying rent"
    );
}
