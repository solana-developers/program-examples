use rand::Rng;
use std::collections::HashMap;

use escrow_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, program_pack::Pack, signature::Keypair, signer::Signer,
    system_instruction, transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::{
    id,
    instruction::{initialize_mint, mint_to},
    state::Account,
    state::Mint,
};
use steel::*;

fn get_random_big_number(_size: usize) -> u64 {
    let mut rng = rand::thread_rng();
    // let signed: BigInt = rng.sample(RandomBits::new(256));
    rng.gen::<u64>()
}

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "escrow_program",
        escrow_api::ID,
        processor!(escrow_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn make_offer_and_take_offer_successful() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Set up Keypairs for alice, bob, and the token mints
    let alice = Keypair::new();
    let bob = Keypair::new();
    let token_mint_a = Keypair::new();
    let token_mint_b = Keypair::new();

    // Token amounts
    let token_a_offered_amount = 5_000_000;
    let token_b_wanted_amount = 1_000_000;

    //Variables for offer_id, pubkey and bump
    let offer_id = get_random_big_number(8);
    let offer_pubkey = offer_pda(alice.pubkey(), offer_id).0;
    let offer_bump = offer_pda(alice.pubkey(), offer_id).1;
    
    // Calculate Associated Token Accounts for alice (maker), bob (taker) and the vault
    let alice_token_account_a =
        get_associated_token_address(&alice.pubkey(), &token_mint_a.pubkey());
    let alice_token_account_b =
        get_associated_token_address(&alice.pubkey(), &token_mint_b.pubkey());
    let bob_token_account_a = get_associated_token_address(&bob.pubkey(), &token_mint_a.pubkey());
    let bob_token_account_b = get_associated_token_address(&bob.pubkey(), &token_mint_b.pubkey());
    let vault = get_associated_token_address(&offer_pubkey, &token_mint_a.pubkey());


    let _accounts = HashMap::from([
        ("maker", alice.pubkey()),
        ("taker", bob.pubkey()),
        ("tokenMintA", token_mint_a.pubkey()),
        ("makerTokenAccountA", alice_token_account_a),
        ("takerTokenAccountA", bob_token_account_a),
        ("tokenMintB", token_mint_b.pubkey()),
        ("makerTokenAccountB", alice_token_account_b),
        ("takerTokenAccountB", bob_token_account_b),
        ("offer", offer_pubkey),
        ("vault", vault),
    ]);

    // Airdrop SOL to alice and bob
    let lamports = LAMPORTS_PER_SOL * 10;
    let airdrop_tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::transfer(&payer.pubkey(), &alice.pubkey(), lamports),
            system_instruction::transfer(&payer.pubkey(), &bob.pubkey(), lamports),
        ],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(airdrop_tx).await;
    assert!(res.is_ok());

    // Calculate minimum lamports required for rent exemption on mint accounts
    let rent = banks.get_rent().await.unwrap();
    let mint_min_balance = rent.minimum_balance(Mint::LEN);

    // Create mint accounts for token A and token B
    let create_mint_tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &token_mint_a.pubkey(),
                mint_min_balance,
                Mint::LEN as u64,
                &id(),
            ),
            system_instruction::create_account(
                &payer.pubkey(),
                &token_mint_b.pubkey(),
                mint_min_balance,
                Mint::LEN as u64,
                &id(),
            ),
        ],
        Some(&payer.pubkey()),
        &[&payer, &token_mint_a, &token_mint_b],
        blockhash,
    );
    let res = banks.process_transaction(create_mint_tx).await;
    assert!(res.is_ok());

    // Transaction to initialize mints
    let init_mint_tx = Transaction::new_signed_with_payer(
        &[
            initialize_mint(&id(), &token_mint_a.pubkey(), &alice.pubkey(), None, 6).unwrap(),
            initialize_mint(&id(), &token_mint_b.pubkey(), &bob.pubkey(), None, 6).unwrap(),
        ],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(init_mint_tx).await;
    assert!(res.is_ok());

    // Transaction to create associated token accounts and mint tokens
    let mint_tokens_tx = Transaction::new_signed_with_payer(
        &[
            create_associated_token_account(
                &payer.pubkey(),
                &alice.pubkey(),
                &token_mint_a.pubkey(),
                &id(),
            ),
            create_associated_token_account(
                &payer.pubkey(),
                &bob.pubkey(),
                &token_mint_b.pubkey(),
                &id(),
            ),
            mint_to(
                &id(),
                &token_mint_a.pubkey(),
                &alice_token_account_a,
                &alice.pubkey(),
                &[],
                1_000_000_000,
            )
            .unwrap(),
            mint_to(
                &id(),
                &token_mint_b.pubkey(),
                &bob_token_account_b,
                &bob.pubkey(),
                &[],
                1_000_000_000,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[&payer, &bob, &alice],
        blockhash,
    );
    let res = banks.process_transaction(mint_tokens_tx).await;
    assert!(res.is_ok());

    // Create and submit make_offer transaction.
    let ix = make_offer(
        alice.pubkey(),
        token_mint_a.pubkey(),
        token_mint_b.pubkey(),
        alice_token_account_a,
        vault,
        token_a_offered_amount,
        token_b_wanted_amount,
        offer_id,
        offer_bump,
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&alice, &payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify vault balance has increased.
    let vault_account = banks
        .get_account(vault)
        .await
        .unwrap()
        .expect("could not fetch account");
    let account_inifo = Account::unpack(&vault_account.data).unwrap();
    assert_eq!(account_inifo.amount, token_a_offered_amount);

    //Verify the offer account contains the necessar details
    let offer_account = banks
        .get_account(offer_pubkey)
        .await
        .unwrap()
        .expect("could not fetch account");
    let offer_data = Offer::try_from_bytes(&offer_account.data).unwrap();
    assert_eq!(offer_data.maker, alice.pubkey());
    assert_eq!(offer_data.id, offer_id);
    assert_eq!(offer_data.token_mint_a, token_mint_a.pubkey());
    assert_eq!(offer_data.token_mint_b, token_mint_b.pubkey());
    assert_eq!(offer_data.token_b_wanted_amount, token_b_wanted_amount);

    // Create and submit take_offer transaction.
    let ix = take_offer(
        bob.pubkey(),
        alice.pubkey(),
        token_mint_a.pubkey(),
        token_mint_b.pubkey(),
        bob_token_account_a,
        bob_token_account_b,
        alice_token_account_b,
        vault,
        offer_id,
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&bob, &payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //Verify alice balance contains tokens wanted amount.
    let alice_account = banks
        .get_account(alice_token_account_b)
        .await
        .unwrap()
        .expect("could not fetch account");
    let account_inifo = Account::unpack(&alice_account.data).unwrap();
    assert_eq!(account_inifo.amount, token_b_wanted_amount);

    //Verify bob balance contains tokens offered amount.
    let bob_account = banks
        .get_account(bob_token_account_a)
        .await
        .unwrap()
        .expect("could not fetch account");
    let account_inifo = Account::unpack(&bob_account.data).unwrap();
    assert_eq!(account_inifo.amount, token_a_offered_amount);
}
