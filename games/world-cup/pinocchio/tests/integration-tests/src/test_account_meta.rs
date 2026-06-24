use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_signer::Signer;

use crate::{
    tests::{
        asserts::TransactionResultExt,
        constants::{EVENT_AUTHORITY, PROGRAM_ID, SYSTEM_PROGRAM_ID},
        idl,
        pda::{get_config_pda, get_oracle_pda, get_vault_pda},
        utils::{build_and_send, funded_keypair, setup, LOCK_TS},
    },
    WorldCupError,
};

fn init_config_metas(admin: &Pubkey) -> Vec<AccountMeta> {
    let (config, _) = get_config_pda();
    let (oracle, _) = get_oracle_pda();
    let (vault, _) = get_vault_pda();
    vec![
        AccountMeta::new(*admin, true),
        AccountMeta::new(config, false),
        AccountMeta::new(oracle, false),
        AccountMeta::new(vault, false),
        AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        AccountMeta::new_readonly(EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PROGRAM_ID, false),
    ]
}

fn init_config_data() -> Vec<u8> {
    let mut data = vec![0u8];
    data.extend_from_slice(&LOCK_TS.to_le_bytes());
    data
}

#[test]
fn idl_writable_accounts_are_enforced() {
    let writable: Vec<idl::IdlAccount> =
        idl::instruction_accounts("initConfig").into_iter().filter(|a| a.is_writable).collect();
    assert!(!writable.is_empty(), "IDL declares writable accounts");

    for account in writable {
        if account.index == 0 {
            continue; // admin is the fee payer: runtime forces it writable, can't demote
        }
        let (mut litesvm, admin) = setup();
        let mut metas = init_config_metas(&admin.pubkey());
        let demoted = &metas[account.index];
        metas[account.index] = AccountMeta::new_readonly(demoted.pubkey, demoted.is_signer);
        let ix = Instruction { program_id: PROGRAM_ID, accounts: metas, data: init_config_data() };
        build_and_send(&mut litesvm, &[&admin], &admin.pubkey(), &ix).assert_err(WorldCupError::AccountNotWritable);
    }
}

#[test]
fn admin_must_sign_init_config() {
    let (mut litesvm, admin) = setup();
    let fee_payer = funded_keypair(&mut litesvm);

    let mut metas = init_config_metas(&admin.pubkey());
    metas[0] = AccountMeta::new(admin.pubkey(), false); // admin present but not a signer
    let ix = Instruction { program_id: PROGRAM_ID, accounts: metas, data: init_config_data() };

    build_and_send(&mut litesvm, &[&fee_payer], &fee_payer.pubkey(), &ix).assert_err(WorldCupError::NotSigner);
}
