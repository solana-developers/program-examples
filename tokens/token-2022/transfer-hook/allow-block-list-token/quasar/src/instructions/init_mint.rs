use quasar_lang::cpi::{BufCpiCall, CpiCall, InstructionAccount, Seed};
use quasar_lang::prelude::*;
use quasar_lang::sysvars::Sysvar;

use crate::constants::*;
use crate::state::mode_to_metadata_value;

/// Token2022 program ID.
pub struct Token2022;
impl Id for Token2022 {
    const ID: Address = Address::new_from_array([
        6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218,
        182, 26, 252, 77, 131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
    ]);
}

#[derive(Accounts)]
pub struct InitMint<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    /// The mint account (must also be a signer for create_account).
    #[account(mut)]
    pub mint: &'info Signer,
    /// ExtraAccountMetaList PDA: ["extra-account-metas", mint]
    #[account(mut)]
    pub extra_metas_account: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
    pub token_program: &'info Program<Token2022>,
}

#[inline(always)]
pub fn handle_init_mint(
    accounts: &InitMint, decimals: u8,
    freeze_authority: &Address,
    permanent_delegate: &Address,
    transfer_hook_authority: &Address,
    mode: u8,
    threshold: u64,
    name: &[u8],
    symbol: &[u8],
    uri: &[u8],
) -> Result<(), ProgramError> {
    let payer_key = accounts.payer.to_account_view().address();
    let mint_key = accounts.mint.to_account_view().address();
    let token_prog = accounts.token_program.to_account_view().address();

    // Calculate mint account size with all extensions:
    // Base mint (82) + padding (82) + AccountType (1)
    // + TransferHook (68) + PermanentDelegate (36) + MetadataPointer (68)
    // + Metadata TLV (variable)
    let mode_value = mode_to_metadata_value(mode);
    // Metadata: TLV header (4) + update_auth (32) + mint (32) + borsh strings:
    //   4 + name.len + 4 + symbol.len + 4 + uri.len
    //   + 4 (additional_metadata length) + additional_metadata entries
    // Additional metadata: ["AB", mode_value]
    let ab_key = b"AB";
    let additional_len = 4 + ab_key.len() + 4 + mode_value.len();
    let threshold_additional = if mode == 2 {
        // "threshold" key + value (up to 20 digits)
        let threshold_str_len = count_digits(threshold);
        4 + b"threshold".len() + 4 + threshold_str_len
    } else {
        0
    };
    let metadata_data_len = 32 + 32 + 4 + name.len() + 4 + symbol.len() + 4 + uri.len()
        + 4 + additional_len + threshold_additional;
    let total_ext_data = 4 + metadata_data_len;
    let mint_size = 82 + 82 + 1 + 68 + 36 + 68 + total_ext_data;
    let lamports = Rent::get()?.try_minimum_balance(mint_size)?;

    // Create the mint account owned by Token2022.
    accounts.system_program
        .create_account(accounts.payer, accounts.mint, lamports, mint_size as u64, token_prog)
        .invoke()?;

    // Initialize PermanentDelegate extension: opcode 35
    let mut pd_data = [0u8; 34];
    pd_data[0] = 35;
    pd_data[2..34].copy_from_slice(permanent_delegate.as_ref());
    CpiCall::new(
        token_prog,
        [InstructionAccount::writable(mint_key)],
        [accounts.mint.to_account_view()],
        pd_data,
    )
    .invoke()?;

    // Initialize TransferHook extension: opcode 36, sub-opcode 0
    let mut th_data = [0u8; 66];
    th_data[0] = 36;
    th_data[1] = 0;
    th_data[2..34].copy_from_slice(transfer_hook_authority.as_ref());
    th_data[34..66].copy_from_slice(crate::ID.as_ref());
    CpiCall::new(
        token_prog,
        [InstructionAccount::writable(mint_key)],
        [accounts.mint.to_account_view()],
        th_data,
    )
    .invoke()?;

    // Initialize MetadataPointer: opcode 39, sub-opcode 0
    let mut mp_data = [0u8; 66];
    mp_data[0] = 39;
    mp_data[1] = 0;
    mp_data[2..34].copy_from_slice(payer_key.as_ref());
    mp_data[34..66].copy_from_slice(mint_key.as_ref());
    CpiCall::new(
        token_prog,
        [InstructionAccount::writable(mint_key)],
        [accounts.mint.to_account_view()],
        mp_data,
    )
    .invoke()?;

    // InitializeMint2: opcode 20
    let mut mint_ix = [0u8; 67];
    mint_ix[0] = 20;
    mint_ix[1] = decimals;
    mint_ix[2..34].copy_from_slice(payer_key.as_ref());
    mint_ix[34] = 1; // has freeze authority
    mint_ix[35..67].copy_from_slice(freeze_authority.as_ref());
    CpiCall::new(
        token_prog,
        [InstructionAccount::writable(mint_key)],
        [accounts.mint.to_account_view()],
        mint_ix,
    )
    .invoke()?;

    // TokenMetadataInitialize: opcode 44, sub-opcode 0
    let mut buf = [0u8; MAX_META_IX];
    let mut pos = 0;
    buf[pos] = 44;
    pos += 1;
    buf[pos] = 0;
    pos += 1;
    // update_authority
    buf[pos..pos + 32].copy_from_slice(payer_key.as_ref());
    pos += 32;
    // mint
    buf[pos..pos + 32].copy_from_slice(mint_key.as_ref());
    pos += 32;
    // name (borsh string: u32 len + bytes)
    buf[pos..pos + 4].copy_from_slice(&(name.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + name.len()].copy_from_slice(name);
    pos += name.len();
    // symbol
    buf[pos..pos + 4].copy_from_slice(&(symbol.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + symbol.len()].copy_from_slice(symbol);
    pos += symbol.len();
    // uri
    buf[pos..pos + 4].copy_from_slice(&(uri.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + uri.len()].copy_from_slice(uri);
    pos += uri.len();

    BufCpiCall::new(
        token_prog,
        [
            InstructionAccount::writable(mint_key),
            InstructionAccount::readonly_signer(payer_key),
            InstructionAccount::readonly_signer(payer_key),
        ],
        [
            accounts.mint.to_account_view(),
            accounts.payer.to_account_view(),
            accounts.payer.to_account_view(),
        ],
        buf,
        pos,
    )
    .invoke()?;

    // TokenMetadataUpdateField for "AB" key: opcode 44, sub-opcode 1
    emit_update_field_cpi(accounts, b"AB", mode_value)?;

    // If Mixed mode, also set "threshold"
    if mode == 2 {
        let mut threshold_buf = [0u8; 20];
        let threshold_len = write_u64_to_buf(threshold, &mut threshold_buf);
        emit_update_field_cpi(accounts, b"threshold", &threshold_buf[..threshold_len])?;
    }

    // Top up mint rent if needed after metadata updates increased the account size.
    top_up_rent(accounts)?;

    // Initialize the ExtraAccountMetaList PDA.
    init_extra_metas(accounts)?;

    log("Mint initialized with transfer hook and metadata");
    Ok(())
}

/// Emit a Token-2022 TokenMetadataUpdateField CPI.
/// Opcode 44, sub-opcode 1, followed by Field::Key (discriminator 2, then borsh
/// string for key, then borsh string for value).
fn emit_update_field_cpi(ctx: &InitMint<'_>, key: &[u8], value: &[u8]) -> Result<(), ProgramError> {
    let token_prog = ctx.token_program.to_account_view().address();
    let mint_key = ctx.mint.to_account_view().address();
    let payer_key = ctx.payer.to_account_view().address();

    let mut buf = [0u8; MAX_META_IX];
    let mut pos = 0;
    buf[pos] = 44;
    pos += 1;
    buf[pos] = 1; // UpdateField sub-instruction
    pos += 1;
    // Field enum: Key = discriminator 2 (borsh enum), then borsh string for the key name
    buf[pos] = 2;
    pos += 1;
    buf[pos..pos + 4].copy_from_slice(&(key.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + key.len()].copy_from_slice(key);
    pos += key.len();
    // value (borsh string)
    buf[pos..pos + 4].copy_from_slice(&(value.len() as u32).to_le_bytes());
    pos += 4;
    buf[pos..pos + value.len()].copy_from_slice(value);
    pos += value.len();

    BufCpiCall::new(
        token_prog,
        [
            InstructionAccount::writable(mint_key),
            InstructionAccount::readonly_signer(payer_key),
        ],
        [
            ctx.mint.to_account_view(),
            ctx.payer.to_account_view(),
        ],
        buf,
        pos,
    )
    .invoke()
}

/// Top up the mint account if its balance is below the rent minimum for its
/// current data size.
fn top_up_rent(ctx: &InitMint<'_>) -> Result<(), ProgramError> {
    let mint_view = ctx.mint.to_account_view();
    let data_len = mint_view.data_len();
    let min_balance = Rent::get()?.try_minimum_balance(data_len)?;
    let current_lamports = mint_view.lamports();

    if min_balance > current_lamports {
        let diff = min_balance - current_lamports;
        ctx.system_program
            .transfer(ctx.payer, ctx.mint, diff)
            .invoke()?;
    }
    Ok(())
}

/// Create the ExtraAccountMetaList PDA and populate it with the ABWallet
/// extra account meta (PDA seeded by [AB_WALLET_SEED, AccountData(2, 32, 32)]).
fn init_extra_metas(ctx: &InitMint<'_>) -> Result<(), ProgramError> {
    let mint_key = ctx.mint.to_account_view().address();

    // Meta list with 1 extra account = 51 bytes
    let meta_list_size: u64 = 51;
    let lamports = Rent::get()?.try_minimum_balance(meta_list_size as usize)?;

    let (expected_pda, bump) = Address::find_program_address(
        &[META_LIST_ACCOUNT_SEED, mint_key.as_ref()],
        &crate::ID,
    );
    if ctx.extra_metas_account.to_account_view().address() != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    let bump_bytes = [bump];
    let seeds = [
        Seed::from(META_LIST_ACCOUNT_SEED),
        Seed::from(mint_key.as_ref()),
        Seed::from(&bump_bytes as &[u8]),
    ];

    ctx.system_program
        .create_account(ctx.payer, &*ctx.extra_metas_account, lamports, meta_list_size, &crate::ID)
        .invoke_signed(&seeds)?;

    let view = unsafe {
        &mut *(ctx.extra_metas_account as *const UncheckedAccount as *mut UncheckedAccount
            as *mut AccountView)
    };
    let mut data = view.try_borrow_mut()?;

    // TLV header
    data[0..8].copy_from_slice(&EXECUTE_DISCRIMINATOR);
    data[8..12].copy_from_slice(&39u32.to_le_bytes()); // data length: 4 + 35
    data[12..16].copy_from_slice(&1u32.to_le_bytes()); // count = 1

    // ExtraAccountMeta for ABWallet PDA:
    //   seeds = [Literal("ab_wallet"), AccountData(account_index=2, data_index=32, length=32)]
    //   Account index 2 = destination token account; data_index 32 = owner field.
    data[16] = 1; // discriminator: PDA from seeds
    let mut config = [0u8; 32];
    config[0] = 2;  // number of seeds

    // Seed 0: Literal "ab_wallet"
    config[1] = 0;  // seed type: literal
    config[2] = 9;  // seed length
    config[3..12].copy_from_slice(AB_WALLET_SEED);

    // Seed 1: AccountData(account_index=2, data_index=32, length=32)
    config[12] = 1; // seed type: account data
    config[13] = 2; // account_index (destination token account)
    config[14] = 32; // data_index (owner field offset in token account)
    config[15] = 32; // length (pubkey size)

    data[17..49].copy_from_slice(&config);
    data[49] = 0; // is_signer = false
    data[50] = 0; // is_writable = false

    Ok(())
}

/// Count decimal digits of a u64 value.
fn count_digits(mut value: u64) -> usize {
    if value == 0 {
        return 1;
    }
    let mut count = 0;
    while value > 0 {
        count += 1;
        value /= 10;
    }
    count
}

/// Write a u64 as decimal ASCII into a buffer. Returns the number of bytes written.
fn write_u64_to_buf(mut value: u64, buf: &mut [u8]) -> usize {
    if value == 0 {
        buf[0] = b'0';
        return 1;
    }
    let mut tmp = [0u8; 20];
    let mut pos = 0;
    while value > 0 {
        tmp[pos] = b'0' + (value % 10) as u8;
        value /= 10;
        pos += 1;
    }
    // Reverse into buf
    for i in 0..pos {
        buf[i] = tmp[pos - 1 - i];
    }
    pos
}
