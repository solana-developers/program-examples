#![allow(dead_code)]
use solana_program::hash::Hash;
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use token_swap_api::prelude::*;

use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    program_pack::Pack,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    transport::TransportError,
};
use spl_token::{
    id,
    instruction::{initialize_mint, mint_to},
    state::Mint,
};

pub async fn setup() -> (BanksClient, Keypair, Hash) {
    std::env::set_var("RUST_LOG", "warn");
    let mut program_test = ProgramTest::new(
        "token_swap_program",
        token_swap_api::ID,
        processor!(token_swap_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

pub struct TestValues {
    pub id: Pubkey,
    pub fee: u16,
    pub admin: Keypair,
    pub mint_a_keypair: Keypair,
    pub mint_b_keypair: Keypair,
    pub default_supply: u64,
    pub amm_key: Pubkey,
    pub minimum_liquidity: u64,
    pub pool_key: Pubkey,
    pub pool_authority: Pubkey,
    pub mint_liquidity: Pubkey,
    pub deposit_amount_a: u64,
    pub deposit_amount_b: u64,
    pub liquidity_account: Pubkey,
    pub pool_account_a: Pubkey,
    pub pool_account_b: Pubkey,
    pub holder_account_a: Pubkey,
    pub holder_account_b: Pubkey,
}

impl TestValues {
    /// Create a new `TestValues` instance with default or random values for testing.
    pub fn new(program_id: &Pubkey) -> Self {
        let id = Pubkey::new_unique();
        let admin = Keypair::new();
        let amm_key = Pubkey::find_program_address(&[id.as_ref()], program_id).0;

        // Generating mints in ascending order
        let mint_a_keypair = Keypair::new();
        let mut mint_b_keypair = Keypair::new();
        while mint_b_keypair.pubkey() < mint_a_keypair.pubkey() {
            mint_b_keypair = Keypair::new();
        }

        // Finding various program addresses based on the mints and AMM key
        let pool_authority = Pubkey::find_program_address(
            &[
                amm_key.as_ref(),
                mint_a_keypair.pubkey().as_ref(),
                mint_b_keypair.pubkey().as_ref(),
                b"authority",
            ],
            program_id,
        )
        .0;

        let mint_liquidity = Pubkey::find_program_address(
            &[
                amm_key.as_ref(),
                mint_a_keypair.pubkey().as_ref(),
                mint_b_keypair.pubkey().as_ref(),
                b"liquidity",
            ],
            program_id,
        )
        .0;

        let pool_key = Pubkey::find_program_address(
            &[
                amm_key.as_ref(),
                mint_a_keypair.pubkey().as_ref(),
                mint_b_keypair.pubkey().as_ref(),
            ],
            program_id,
        )
        .0;

        let default_supply = 100 * 10u64.pow(6);
        let minimum_liquidity = 100;
        let deposit_amount_a = 4 * 10u64.pow(6);
        let deposit_amount_b = 1 * 10u64.pow(6);

        // Associated token accounts
        let liquidity_account = spl_associated_token_account::get_associated_token_address(
            &admin.pubkey(),
            &mint_liquidity,
        );

        let pool_account_a = spl_associated_token_account::get_associated_token_address(
            &pool_authority,
            &mint_a_keypair.pubkey(),
        );

        let pool_account_b = spl_associated_token_account::get_associated_token_address(
            &pool_authority,
            &mint_b_keypair.pubkey(),
        );

        let holder_account_a = spl_associated_token_account::get_associated_token_address(
            &admin.pubkey(),
            &mint_a_keypair.pubkey(),
        );

        let holder_account_b = spl_associated_token_account::get_associated_token_address(
            &admin.pubkey(),
            &mint_b_keypair.pubkey(),
        );

        TestValues {
            id,
            fee: 500,
            admin,
            mint_a_keypair,
            mint_b_keypair,
            default_supply,
            amm_key,
            minimum_liquidity,
            pool_key,
            pool_authority,
            mint_liquidity,
            deposit_amount_a,
            deposit_amount_b,
            liquidity_account,
            pool_account_a,
            pool_account_b,
            holder_account_a,
            holder_account_b,
        }
    }
}

pub async fn create_and_initialize_mint(
    authority: &Keypair,
    mint_account: &Keypair,
    banks: &mut BanksClient,
    payer: &Keypair,
    blockhash: Hash,
    decimals: u8,
) -> Result<(), TransportError> {
    let rent = banks.get_rent().await.unwrap();
    let mint_min_balance = rent.minimum_balance(Mint::LEN);

    let create_mint_and_add_transactions = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint_account.pubkey(),
                mint_min_balance,
                Mint::LEN as u64,
                &id(),
            ),
            initialize_mint(
                &id(),
                &mint_account.pubkey(),
                &authority.pubkey(),
                None,
                decimals,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        blockhash,
    );
    let res = banks
        .process_transaction(create_mint_and_add_transactions)
        .await;
    assert!(res.is_ok());
    Ok(())
}

/// Helper function to simulate token minting for test purposes
pub async fn mint_tokens(
    receiver: Pubkey,
    token_account: Pubkey,
    mint_account: &Keypair,
    banks: &mut BanksClient,
    payer: &Keypair,
    blockhash: Hash,
) -> Result<(), TransportError> {
    let mint_tokens_transaction = Transaction::new_signed_with_payer(
        &[
            create_associated_token_account_idempotent(
                &payer.pubkey(),
                &receiver,
                &token_account,
                &id(),
            ),
            mint_to(
                &id(),
                &mint_account.pubkey(),
                &token_account,
                &receiver,
                &[],
                100_000_000,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        blockhash,
    );
    let res = banks.process_transaction(mint_tokens_transaction).await;
    assert!(res.is_ok());
    Ok(())
}

pub async fn _sleep(seconds: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
}

pub async fn _airdrop(
    user: Pubkey,
    banks: &mut BanksClient,
    payer: Keypair,
    blockhash: Hash,
) -> Result<(), TransportError> {
    // Airdrop SOL to alice and bob
    let lamports = LAMPORTS_PER_SOL * 10;
    let airdrop_tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &payer.pubkey(),
            &user,
            lamports,
        )],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(airdrop_tx).await;
    assert!(res.is_ok());

    Ok(())
}

pub async fn creates_amm(
    banks: &mut BanksClient,
    payer: &Keypair,
    blockhash: Hash,
    accounts: &TestValues,
) -> Result<(), TransportError> {
    // Submit initialize transaction.
    let ix = create_amm(
        payer.pubkey(),
        accounts.amm_key,
        accounts.admin.pubkey(),
        accounts.id,
        accounts.fee,
    );
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;

    Ok(res?)
}

pub async fn creates_pool(
    banks: &mut BanksClient,
    payer: &Keypair,
    blockhash: Hash,
    accounts: &TestValues,
) -> Result<(), TransportError> {
    // Submit initialize transaction.
    let mints = [&accounts.mint_a_keypair, &accounts.mint_b_keypair];

    for item in mints {
        let _ = create_and_initialize_mint(&accounts.admin, item, banks, payer, blockhash, 6).await;
        let _ = mint_tokens(
            accounts.admin.pubkey(),
            get_associated_token_address(&accounts.admin.pubkey(), &item.pubkey()),
            item,
            banks,
            &payer,
            blockhash,
        );
    }

    let ix = create_pool(
        payer.pubkey(),
        accounts.amm_key,
        accounts.pool_key,
        accounts.pool_authority,
        accounts.mint_liquidity,
        accounts.mint_a_keypair.pubkey(),
        accounts.mint_b_keypair.pubkey(),
        accounts.pool_account_a,
        accounts.pool_account_b,
    );
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    Ok(res?)
}

pub async fn deposits_liquidity(
    banks: &mut BanksClient,
    payer: &Keypair,
    blockhash: Hash,
    accounts: &TestValues,
) -> Result<(), TransportError> {
    let ix = deposit_liquidity(
        payer.pubkey(),
        accounts.admin.pubkey(),
        accounts.pool_key,
        accounts.pool_authority,
        accounts.mint_liquidity,
        accounts.mint_a_keypair.pubkey(),
        accounts.mint_b_keypair.pubkey(),
        accounts.pool_account_a,
        accounts.pool_account_b,
        accounts.liquidity_account,
        accounts.holder_account_a,
        accounts.holder_account_b,
        accounts.deposit_amount_a,
        accounts.deposit_amount_a,
    );
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    Ok(res?)
}
