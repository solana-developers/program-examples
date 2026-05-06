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
    let program_id = escrow::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/escrow.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 100_000_000_000).unwrap();
    (svm, program_id, payer)
}

struct EscrowSetup {
    svm: LiteSVM,
    program_id: Pubkey,
    payer: Keypair,
    alice: Keypair,
    bob: Keypair,
    mint_a: Pubkey,
    mint_b: Pubkey,
    alice_ata_a: Pubkey,
    alice_ata_b: Pubkey,
    bob_ata_a: Pubkey,
    bob_ata_b: Pubkey,
}

fn full_setup() -> EscrowSetup {
    let (mut svm, program_id, payer) = setup();

    let alice = create_wallet(&mut svm, 10_000_000_000).unwrap();
    let bob = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let decimals: u8 = 6;
    let alice_amount: u64 = 1_000_000_000;
    let bob_amount: u64 = 1_000_000_000;

    // Create mints (payer is mint authority)
    let mint_a = create_token_mint(&mut svm, &payer, decimals, None).unwrap();
    let mint_b = create_token_mint(&mut svm, &payer, decimals, None).unwrap();

    // Create ATAs
    let alice_ata_a =
        create_associated_token_account(&mut svm, &alice.pubkey(), &mint_a, &payer).unwrap();
    let alice_ata_b =
        create_associated_token_account(&mut svm, &alice.pubkey(), &mint_b, &payer).unwrap();
    let bob_ata_b =
        create_associated_token_account(&mut svm, &bob.pubkey(), &mint_b, &payer).unwrap();

    // bob_ata_a is derived but not pre-created (program uses init_if_needed)
    let bob_ata_a = derive_ata(&bob.pubkey(), &mint_a);

    // Mint tokens: Alice gets token A, Bob gets token B
    mint_tokens_to_token_account(&mut svm, &mint_a, &alice_ata_a, alice_amount, &payer).unwrap();
    mint_tokens_to_token_account(&mut svm, &mint_b, &bob_ata_b, bob_amount, &payer).unwrap();

    EscrowSetup {
        svm,
        program_id,
        payer,
        alice,
        bob,
        mint_a,
        mint_b,
        alice_ata_a,
        alice_ata_b,
        bob_ata_a,
        bob_ata_b,
    }
}

#[test]
fn test_make_offer() {
    let mut es = full_setup();

    let offer_id: u64 = 1;
    let token_a_offered_amount: u64 = 1_000_000;
    let token_b_wanted_amount: u64 = 1_000_000;

    // Derive offer PDA
    let (offer_pda, _bump) = Pubkey::find_program_address(
        &[
            b"offer",
            es.alice.pubkey().as_ref(),
            &offer_id.to_le_bytes(),
        ],
        &es.program_id,
    );

    // Vault is the ATA of the offer PDA for mint_a
    let vault = derive_ata(&offer_pda, &es.mint_a);

    let make_offer_ix = Instruction::new_with_bytes(
        es.program_id,
        &escrow::instruction::MakeOffer {
            id: offer_id,
            token_a_offered_amount,
            token_b_wanted_amount,
        }
        .data(),
        escrow::accounts::MakeOffer {
            maker: es.alice.pubkey(),
            token_mint_a: es.mint_a,
            token_mint_b: es.mint_b,
            maker_token_account_a: es.alice_ata_a,
            offer: offer_pda,
            vault,
            associated_token_program: ata_program_id(),
            token_program: token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut es.svm,
        vec![make_offer_ix],
        &[&es.payer, &es.alice],
        &es.payer.pubkey(),
    )
    .unwrap();

    // Verify vault contains the offered tokens
    assert_eq!(
        get_token_account_balance(&es.svm, &vault).unwrap(),
        token_a_offered_amount
    );

    // Verify offer account data
    let offer_data = es.svm.get_account(&offer_pda).expect("Offer should exist");
    let data = &offer_data.data[8..]; // Skip 8-byte discriminator
    let stored_id = u64::from_le_bytes(data[0..8].try_into().unwrap());
    assert_eq!(stored_id, offer_id);
    let stored_maker = Pubkey::try_from(&data[8..40]).unwrap();
    assert_eq!(stored_maker, es.alice.pubkey());
}

#[test]
fn test_take_offer() {
    let mut es = full_setup();

    let offer_id: u64 = 2;
    let token_a_offered_amount: u64 = 1_000_000;
    let token_b_wanted_amount: u64 = 1_000_000;

    // Derive offer PDA
    let (offer_pda, _bump) = Pubkey::find_program_address(
        &[
            b"offer",
            es.alice.pubkey().as_ref(),
            &offer_id.to_le_bytes(),
        ],
        &es.program_id,
    );

    let vault = derive_ata(&offer_pda, &es.mint_a);

    // Step 1: Alice makes the offer
    let make_offer_ix = Instruction::new_with_bytes(
        es.program_id,
        &escrow::instruction::MakeOffer {
            id: offer_id,
            token_a_offered_amount,
            token_b_wanted_amount,
        }
        .data(),
        escrow::accounts::MakeOffer {
            maker: es.alice.pubkey(),
            token_mint_a: es.mint_a,
            token_mint_b: es.mint_b,
            maker_token_account_a: es.alice_ata_a,
            offer: offer_pda,
            vault,
            associated_token_program: ata_program_id(),
            token_program: token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut es.svm,
        vec![make_offer_ix],
        &[&es.payer, &es.alice],
        &es.payer.pubkey(),
    )
    .unwrap();

    // Verify vault has tokens
    assert_eq!(
        get_token_account_balance(&es.svm, &vault).unwrap(),
        token_a_offered_amount
    );

    // Step 2: Bob takes the offer
    let take_offer_ix = Instruction::new_with_bytes(
        es.program_id,
        &escrow::instruction::TakeOffer {}.data(),
        escrow::accounts::TakeOffer {
            taker: es.bob.pubkey(),
            maker: es.alice.pubkey(),
            token_mint_a: es.mint_a,
            token_mint_b: es.mint_b,
            taker_token_account_a: es.bob_ata_a,
            taker_token_account_b: es.bob_ata_b,
            maker_token_account_b: es.alice_ata_b,
            offer: offer_pda,
            vault,
            associated_token_program: ata_program_id(),
            token_program: token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut es.svm,
        vec![take_offer_ix],
        &[&es.payer, &es.bob],
        &es.payer.pubkey(),
    )
    .unwrap();

    // Verify Bob received token A from vault
    assert_eq!(
        get_token_account_balance(&es.svm, &es.bob_ata_a).unwrap(),
        token_a_offered_amount
    );

    // Verify Alice received token B from Bob
    assert_eq!(
        get_token_account_balance(&es.svm, &es.alice_ata_b).unwrap(),
        token_b_wanted_amount
    );

    // Verify vault is closed
    assert!(
        es.svm.get_account(&vault).is_none(),
        "Vault should be closed after take_offer"
    );

    // Verify offer account is closed
    assert!(
        es.svm.get_account(&offer_pda).is_none(),
        "Offer should be closed after take_offer"
    );
}
