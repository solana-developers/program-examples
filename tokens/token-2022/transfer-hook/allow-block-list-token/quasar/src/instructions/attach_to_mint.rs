use quasar_lang::cpi::{CpiCall, InstructionAccount, Seed};
use quasar_lang::prelude::*;
use quasar_lang::sysvars::Sysvar;

use crate::constants::*;
use crate::instructions::init_mint::Token2022;

#[derive(Accounts)]
pub struct AttachToMint<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub mint: &'info UncheckedAccount,
    #[account(mut)]
    pub extra_metas_account: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
    pub token_program: &'info Program<Token2022>,
}

#[inline(always)]
pub fn handle_attach_to_mint(accounts: &AttachToMint) -> Result<(), ProgramError> {
    let mint_key = accounts.mint.to_account_view().address();
    let payer_key = accounts.payer.to_account_view().address();
    let token_prog = accounts.token_program.to_account_view().address();

    // TransferHookUpdate: opcode 36, sub-opcode 1
    // Sets the transfer hook program_id on the mint.
    let mut update_data = [0u8; 37];
    update_data[0] = 36;
    update_data[1] = 1; // Update sub-instruction
    // COption<Pubkey>: 4 bytes discriminator (1 = Some) + 32 bytes pubkey
    update_data[2..6].copy_from_slice(&1u32.to_le_bytes()); // Some
    update_data[6..38 - 1].copy_from_slice(&crate::ID.as_ref()[..31]);
    // Actually, COption encoding is: [1u8 if Some, 0 if None] but SPL uses 4 bytes
    // Let me use the right encoding: just 1 byte bool then 32 byte address? No.
    // SPL token uses: 1 byte discriminator (1=Some) followed by 32 bytes.
    // But wait - the TransferHookUpdate instruction encoding for the optional program_id is:
    //   COption: 4 bytes (0 = None, 1 = Some), then 32 bytes if Some.
    // Total ix data = 2 (opcode + sub) + 4 + 32 = 38
    // Let me redo this properly.
    let mut update_data = [0u8; 38];
    update_data[0] = 36;  // TransferHookExtension opcode
    update_data[1] = 1;   // Update sub-instruction
    update_data[2..6].copy_from_slice(&1u32.to_le_bytes()); // COption::Some
    update_data[6..38].copy_from_slice(crate::ID.as_ref());

    CpiCall::new(
        token_prog,
        [
            InstructionAccount::writable(mint_key),
            InstructionAccount::readonly_signer(payer_key),
        ],
        [
            accounts.mint.to_account_view(),
            accounts.payer.to_account_view(),
        ],
        update_data,
    )
    .invoke()?;

    // Initialize the ExtraAccountMetaList PDA (same as in init_mint).
    let meta_list_size: u64 = 51;
    let lamports = Rent::get()?.try_minimum_balance(meta_list_size as usize)?;

    let (expected_pda, bump) = Address::find_program_address(
        &[META_LIST_ACCOUNT_SEED, mint_key.as_ref()],
        &crate::ID,
    );
    if accounts.extra_metas_account.to_account_view().address() != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let bump_bytes = [bump];
    let seeds = [
        Seed::from(META_LIST_ACCOUNT_SEED),
        Seed::from(mint_key.as_ref()),
        Seed::from(&bump_bytes as &[u8]),
    ];

    accounts.system_program
        .create_account(
            accounts.payer,
            &*accounts.extra_metas_account,
            lamports,
            meta_list_size,
            &crate::ID,
        )
        .invoke_signed(&seeds)?;

    // Write ExtraAccountMeta TLV data
    let view = unsafe {
        &mut *(accounts.extra_metas_account as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;

    data[0..8].copy_from_slice(&EXECUTE_DISCRIMINATOR);
    data[8..12].copy_from_slice(&39u32.to_le_bytes());
    data[12..16].copy_from_slice(&1u32.to_le_bytes());

    // ABWallet PDA: seeds = [Literal("ab_wallet"), AccountData(2, 32, 32)]
    data[16] = 1; // PDA from seeds
    let mut config = [0u8; 32];
    config[0] = 2;  // 2 seeds
    config[1] = 0;  // literal
    config[2] = 9;  // length
    config[3..12].copy_from_slice(AB_WALLET_SEED);
    config[12] = 1; // account data
    config[13] = 2; // account_index (destination token account)
    config[14] = 32; // data_index
    config[15] = 32; // length
    data[17..49].copy_from_slice(&config);
    data[49] = 0; // not signer
    data[50] = 0; // not writable

    log("Transfer hook attached to mint");
    Ok(())
}
