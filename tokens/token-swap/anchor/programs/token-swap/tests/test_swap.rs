use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{
        create_associated_token_account, create_token_mint, create_wallet,
        get_token_account_balance, mint_tokens_to_token_account, send_transaction_from_instructions,
    },
    solana_signer::Signer,
};

fn token_program_id() -> Pubkey {
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        .parse()
        .unwrap()
}

fn ata_program_id() -> Pubkey {
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        .parse()
        .unwrap()
}

fn derive_ata(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    let (ata, _bump) = Pubkey::find_program_address(
        &[wallet.as_ref(), token_program_id().as_ref(), mint.as_ref()],
        &ata_program_id(),
    );
    ata
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = swap_example::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/swap_example.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 100_000_000_000).unwrap();
    (svm, program_id, payer)
}

/// Ensure mint_a < mint_b by pubkey ordering (the program may require this).
fn ordered_mints(svm: &mut LiteSVM, authority: &Keypair, decimals: u8) -> (Pubkey, Pubkey) {
    loop {
        let a = create_token_mint(svm, authority, decimals, None).unwrap();
        let b = create_token_mint(svm, authority, decimals, None).unwrap();
        if a.as_ref() < b.as_ref() {
            return (a, b);
        }
    }
}

struct TestSetup {
    svm: LiteSVM,
    program_id: Pubkey,
    payer: Keypair,
    admin: Keypair,
    amm_key: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool_key: Pubkey,
    pool_authority: Pubkey,
    mint_liquidity: Pubkey,
    pool_account_a: Pubkey,
    pool_account_b: Pubkey,
    holder_account_a: Pubkey,
    holder_account_b: Pubkey,
    liquidity_account: Pubkey,
}

fn full_setup() -> TestSetup {
    let (mut svm, program_id, payer) = setup();
    let admin = create_wallet(&mut svm, 100_000_000_000).unwrap();

    let decimals: u8 = 6;
    let minted_amount: u64 = 100 * 10u64.pow(decimals as u32);

    let (mint_a, mint_b) = ordered_mints(&mut svm, &admin, decimals);
    let amm_id = Keypair::new().pubkey();
    let fee: u16 = 500;

    // Derive PDAs
    let (amm_key, _) = Pubkey::find_program_address(&[amm_id.as_ref()], &program_id);
    let (pool_key, _) = Pubkey::find_program_address(
        &[amm_key.as_ref(), mint_a.as_ref(), mint_b.as_ref()],
        &program_id,
    );
    let (pool_authority, _) = Pubkey::find_program_address(
        &[
            amm_key.as_ref(),
            mint_a.as_ref(),
            mint_b.as_ref(),
            b"authority",
        ],
        &program_id,
    );
    let (mint_liquidity, _) = Pubkey::find_program_address(
        &[
            amm_key.as_ref(),
            mint_a.as_ref(),
            mint_b.as_ref(),
            b"liquidity",
        ],
        &program_id,
    );

    let pool_account_a = derive_ata(&pool_authority, &mint_a);
    let pool_account_b = derive_ata(&pool_authority, &mint_b);
    let liquidity_account = derive_ata(&admin.pubkey(), &mint_liquidity);

    // Create ATAs for admin and mint tokens
    let holder_account_a =
        create_associated_token_account(&mut svm, &admin.pubkey(), &mint_a, &payer).unwrap();
    let holder_account_b =
        create_associated_token_account(&mut svm, &admin.pubkey(), &mint_b, &payer).unwrap();

    mint_tokens_to_token_account(&mut svm, &mint_a, &holder_account_a, minted_amount, &admin).unwrap();
    mint_tokens_to_token_account(&mut svm, &mint_b, &holder_account_b, minted_amount, &admin).unwrap();

    // Create AMM
    let create_amm_ix = Instruction::new_with_bytes(
        program_id,
        &swap_example::instruction::CreateAmm { id: amm_id, fee }.data(),
        swap_example::accounts::CreateAmm {
            amm: amm_key,
            admin: admin.pubkey(),
            payer: payer.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![create_amm_ix],
        &[&payer],
        &payer.pubkey(),
    )
    .unwrap();

    // Create Pool
    let create_pool_ix = Instruction::new_with_bytes(
        program_id,
        &swap_example::instruction::CreatePool {}.data(),
        swap_example::accounts::CreatePool {
            amm: amm_key,
            pool: pool_key,
            pool_authority,
            mint_liquidity,
            mint_a,
            mint_b,
            pool_account_a,
            pool_account_b,
            payer: payer.pubkey(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![create_pool_ix],
        &[&payer],
        &payer.pubkey(),
    )
    .unwrap();

    TestSetup {
        svm,
        program_id,
        payer,
        admin,
        amm_key,
        mint_a,
        mint_b,
        pool_key,
        pool_authority,
        mint_liquidity,
        pool_account_a,
        pool_account_b,
        holder_account_a,
        holder_account_b,
        liquidity_account,
    }
}

#[test]
fn test_create_amm() {
    let (mut svm, program_id, payer) = setup();
    let amm_id = Keypair::new().pubkey();
    let fee: u16 = 500;
    let admin = Keypair::new();

    let (amm_key, _) = Pubkey::find_program_address(&[amm_id.as_ref()], &program_id);

    let create_amm_ix = Instruction::new_with_bytes(
        program_id,
        &swap_example::instruction::CreateAmm { id: amm_id, fee }.data(),
        swap_example::accounts::CreateAmm {
            amm: amm_key,
            admin: admin.pubkey(),
            payer: payer.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![create_amm_ix],
        &[&payer],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify AMM account exists
    let amm_account = svm.get_account(&amm_key).expect("AMM account should exist");
    assert!(!amm_account.data.is_empty());
}

#[test]
fn test_deposit_liquidity() {
    let mut ts = full_setup();
    let deposit_amount_a: u64 = 4_000_000;
    let deposit_amount_b: u64 = 1_000_000;

    let deposit_ix = Instruction::new_with_bytes(
        ts.program_id,
        &swap_example::instruction::DepositLiquidity {
            amount_a: deposit_amount_a,
            amount_b: deposit_amount_b,
        }
        .data(),
        swap_example::accounts::DepositLiquidity {
            pool: ts.pool_key,
            pool_authority: ts.pool_authority,
            depositor: ts.admin.pubkey(),
            mint_liquidity: ts.mint_liquidity,
            mint_a: ts.mint_a,
            mint_b: ts.mint_b,
            pool_account_a: ts.pool_account_a,
            pool_account_b: ts.pool_account_b,
            depositor_account_liquidity: ts.liquidity_account,
            depositor_account_a: ts.holder_account_a,
            depositor_account_b: ts.holder_account_b,
            payer: ts.payer.pubkey(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut ts.svm,
        vec![deposit_ix],
        &[&ts.payer, &ts.admin],
        &ts.payer.pubkey(),
    )
    .unwrap();

    // Verify liquidity tokens were minted
    let liq_amount = get_token_account_balance(&ts.svm, &ts.liquidity_account).unwrap();
    assert!(liq_amount > 0, "Should have received liquidity tokens");
}

#[test]
fn test_swap_a_to_b() {
    let mut ts = full_setup();

    // Deposit liquidity first
    let deposit_ix = Instruction::new_with_bytes(
        ts.program_id,
        &swap_example::instruction::DepositLiquidity {
            amount_a: 4_000_000,
            amount_b: 1_000_000,
        }
        .data(),
        swap_example::accounts::DepositLiquidity {
            pool: ts.pool_key,
            pool_authority: ts.pool_authority,
            depositor: ts.admin.pubkey(),
            mint_liquidity: ts.mint_liquidity,
            mint_a: ts.mint_a,
            mint_b: ts.mint_b,
            pool_account_a: ts.pool_account_a,
            pool_account_b: ts.pool_account_b,
            depositor_account_liquidity: ts.liquidity_account,
            depositor_account_a: ts.holder_account_a,
            depositor_account_b: ts.holder_account_b,
            payer: ts.payer.pubkey(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut ts.svm,
        vec![deposit_ix],
        &[&ts.payer, &ts.admin],
        &ts.payer.pubkey(),
    )
    .unwrap();

    // Get balances before swap
    let before_b = get_token_account_balance(&ts.svm, &ts.holder_account_b).unwrap();

    // Swap 1M of token A for token B
    let swap_ix = Instruction::new_with_bytes(
        ts.program_id,
        &swap_example::instruction::SwapExactTokensForTokens {
            swap_a: true,
            input_amount: 1_000_000,
            min_output_amount: 100,
        }
        .data(),
        swap_example::accounts::SwapExactTokensForTokens {
            amm: ts.amm_key,
            pool: ts.pool_key,
            pool_authority: ts.pool_authority,
            trader: ts.admin.pubkey(),
            mint_a: ts.mint_a,
            mint_b: ts.mint_b,
            pool_account_a: ts.pool_account_a,
            pool_account_b: ts.pool_account_b,
            trader_account_a: ts.holder_account_a,
            trader_account_b: ts.holder_account_b,
            payer: ts.payer.pubkey(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut ts.svm,
        vec![swap_ix],
        &[&ts.payer, &ts.admin],
        &ts.payer.pubkey(),
    )
    .unwrap();

    // After swap, token B balance should have increased
    let after_b = get_token_account_balance(&ts.svm, &ts.holder_account_b).unwrap();
    assert!(
        after_b > before_b,
        "Token B balance should increase after swap A->B"
    );
}

#[test]
fn test_withdraw_liquidity() {
    let mut ts = full_setup();

    // Deposit liquidity
    let deposit_ix = Instruction::new_with_bytes(
        ts.program_id,
        &swap_example::instruction::DepositLiquidity {
            amount_a: 4_000_000,
            amount_b: 4_000_000,
        }
        .data(),
        swap_example::accounts::DepositLiquidity {
            pool: ts.pool_key,
            pool_authority: ts.pool_authority,
            depositor: ts.admin.pubkey(),
            mint_liquidity: ts.mint_liquidity,
            mint_a: ts.mint_a,
            mint_b: ts.mint_b,
            pool_account_a: ts.pool_account_a,
            pool_account_b: ts.pool_account_b,
            depositor_account_liquidity: ts.liquidity_account,
            depositor_account_a: ts.holder_account_a,
            depositor_account_b: ts.holder_account_b,
            payer: ts.payer.pubkey(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut ts.svm,
        vec![deposit_ix],
        &[&ts.payer, &ts.admin],
        &ts.payer.pubkey(),
    )
    .unwrap();

    // Get liquidity token balance
    let liq_amount = get_token_account_balance(&ts.svm, &ts.liquidity_account).unwrap();
    assert!(liq_amount > 0);

    // Withdraw all liquidity
    let withdraw_ix = Instruction::new_with_bytes(
        ts.program_id,
        &swap_example::instruction::WithdrawLiquidity {
            amount: liq_amount,
        }
        .data(),
        swap_example::accounts::WithdrawLiquidity {
            amm: ts.amm_key,
            pool: ts.pool_key,
            pool_authority: ts.pool_authority,
            depositor: ts.admin.pubkey(),
            mint_liquidity: ts.mint_liquidity,
            mint_a: ts.mint_a,
            mint_b: ts.mint_b,
            pool_account_a: ts.pool_account_a,
            pool_account_b: ts.pool_account_b,
            depositor_account_liquidity: ts.liquidity_account,
            depositor_account_a: ts.holder_account_a,
            depositor_account_b: ts.holder_account_b,
            payer: ts.payer.pubkey(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut ts.svm,
        vec![withdraw_ix],
        &[&ts.payer, &ts.admin],
        &ts.payer.pubkey(),
    )
    .unwrap();

    // Liquidity balance should be 0
    let liq_amount = get_token_account_balance(&ts.svm, &ts.liquidity_account).unwrap();
    assert_eq!(liq_amount, 0, "Liquidity should be fully withdrawn");
}
