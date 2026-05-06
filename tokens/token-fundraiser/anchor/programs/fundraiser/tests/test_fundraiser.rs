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
    let program_id = fundraiser::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/fundraiser.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 100_000_000_000).unwrap();
    (svm, program_id, payer)
}

struct FundraiserSetup {
    svm: LiteSVM,
    program_id: Pubkey,
    payer: Keypair,
    maker: Keypair,
    mint: Pubkey,
    fundraiser_pda: Pubkey,
    vault: Pubkey,
}

fn full_setup() -> FundraiserSetup {
    let (mut svm, program_id, payer) = setup();

    let maker = create_wallet(&mut svm, 10_000_000_000).unwrap();

    // Create mint (6 decimals) — payer is mint authority
    let mint = create_token_mint(&mut svm, &payer, 6, None).unwrap();

    // Derive the fundraiser PDA
    let (fundraiser_pda, _bump) = Pubkey::find_program_address(
        &[b"fundraiser", maker.pubkey().as_ref()],
        &program_id,
    );

    // Vault is the ATA of the fundraiser PDA for the mint
    let vault = derive_ata(&fundraiser_pda, &mint);

    FundraiserSetup {
        svm,
        program_id,
        payer,
        maker,
        mint,
        fundraiser_pda,
        vault,
    }
}

#[test]
fn test_initialize_fundraiser() {
    let mut fs = full_setup();

    let amount_to_raise: u64 = 30_000_000;
    let duration: u16 = 0;

    let init_ix = Instruction::new_with_bytes(
        fs.program_id,
        &fundraiser::instruction::Initialize {
            amount: amount_to_raise,
            duration,
        }
        .data(),
        fundraiser::accounts::Initialize {
            maker: fs.maker.pubkey(),
            mint_to_raise: fs.mint,
            fundraiser: fs.fundraiser_pda,
            vault: fs.vault,
            system_program: system_program::id(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut fs.svm,
        vec![init_ix],
        &[&fs.maker],
        &fs.maker.pubkey(),
    )
    .unwrap();

    // Verify fundraiser account exists
    let fundraiser_data = fs
        .svm
        .get_account(&fs.fundraiser_pda)
        .expect("Fundraiser account should exist");
    assert!(!fundraiser_data.data.is_empty());

    // Verify vault exists with zero balance
    assert_eq!(get_token_account_balance(&fs.svm, &fs.vault).unwrap(), 0);
}

#[test]
fn test_contribute_and_refund() {
    let mut fs = full_setup();

    let amount_to_raise: u64 = 30_000_000;
    let duration: u16 = 0;

    // Initialize fundraiser
    let init_ix = Instruction::new_with_bytes(
        fs.program_id,
        &fundraiser::instruction::Initialize {
            amount: amount_to_raise,
            duration,
        }
        .data(),
        fundraiser::accounts::Initialize {
            maker: fs.maker.pubkey(),
            mint_to_raise: fs.mint,
            fundraiser: fs.fundraiser_pda,
            vault: fs.vault,
            system_program: system_program::id(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut fs.svm,
        vec![init_ix],
        &[&fs.maker],
        &fs.maker.pubkey(),
    )
    .unwrap();

    // Setup contributor using Kite
    let contributor = create_wallet(&mut fs.svm, 10_000_000_000).unwrap();

    let contributor_ata =
        create_associated_token_account(&mut fs.svm, &contributor.pubkey(), &fs.mint, &fs.payer)
            .unwrap();

    let mint_amount: u64 = 10_000_000;
    mint_tokens_to_token_account(&mut fs.svm, &fs.mint, &contributor_ata, mint_amount, &fs.payer)
        .unwrap();

    // Derive contributor account PDA
    let (contributor_account_pda, _bump) = Pubkey::find_program_address(
        &[
            b"contributor",
            fs.fundraiser_pda.as_ref(),
            contributor.pubkey().as_ref(),
        ],
        &fs.program_id,
    );

    // Contribute 1_000_000
    let contribute_amount: u64 = 1_000_000;
    let contribute_ix = Instruction::new_with_bytes(
        fs.program_id,
        &fundraiser::instruction::Contribute {
            amount: contribute_amount,
        }
        .data(),
        fundraiser::accounts::Contribute {
            contributor: contributor.pubkey(),
            mint_to_raise: fs.mint,
            fundraiser: fs.fundraiser_pda,
            contributor_account: contributor_account_pda,
            contributor_ata,
            vault: fs.vault,
            token_program: token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut fs.svm,
        vec![contribute_ix],
        &[&contributor],
        &contributor.pubkey(),
    )
    .unwrap();

    // Verify vault balance
    assert_eq!(
        get_token_account_balance(&fs.svm, &fs.vault).unwrap(),
        contribute_amount
    );

    // Expire blockhash to avoid AlreadyProcessed error (same accounts, same amount = same tx hash)
    fs.svm.expire_blockhash();

    // Contribute again
    let contribute_ix2 = Instruction::new_with_bytes(
        fs.program_id,
        &fundraiser::instruction::Contribute {
            amount: contribute_amount,
        }
        .data(),
        fundraiser::accounts::Contribute {
            contributor: contributor.pubkey(),
            mint_to_raise: fs.mint,
            fundraiser: fs.fundraiser_pda,
            contributor_account: contributor_account_pda,
            contributor_ata,
            vault: fs.vault,
            token_program: token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut fs.svm,
        vec![contribute_ix2],
        &[&contributor],
        &contributor.pubkey(),
    )
    .unwrap();

    // Verify vault balance is now 2_000_000
    assert_eq!(
        get_token_account_balance(&fs.svm, &fs.vault).unwrap(),
        contribute_amount * 2
    );

    fs.svm.expire_blockhash();

    // Refund
    let refund_ix = Instruction::new_with_bytes(
        fs.program_id,
        &fundraiser::instruction::Refund {}.data(),
        fundraiser::accounts::Refund {
            contributor: contributor.pubkey(),
            maker: fs.maker.pubkey(),
            mint_to_raise: fs.mint,
            fundraiser: fs.fundraiser_pda,
            contributor_account: contributor_account_pda,
            contributor_ata,
            vault: fs.vault,
            token_program: token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut fs.svm,
        vec![refund_ix],
        &[&contributor],
        &contributor.pubkey(),
    )
    .unwrap();

    // Verify vault is empty after refund
    assert_eq!(get_token_account_balance(&fs.svm, &fs.vault).unwrap(), 0);

    // Verify contributor got tokens back
    assert_eq!(
        get_token_account_balance(&fs.svm, &contributor_ata).unwrap(),
        mint_amount
    );

    // Contributor account PDA should be closed
    assert!(
        fs.svm.get_account(&contributor_account_pda).is_none(),
        "Contributor account should be closed after refund"
    );
}

#[test]
fn test_check_contributions_success() {
    let mut fs = full_setup();

    let amount_to_raise: u64 = 1_000;
    let duration: u16 = 0;

    // Initialize fundraiser
    let init_ix = Instruction::new_with_bytes(
        fs.program_id,
        &fundraiser::instruction::Initialize {
            amount: amount_to_raise,
            duration,
        }
        .data(),
        fundraiser::accounts::Initialize {
            maker: fs.maker.pubkey(),
            mint_to_raise: fs.mint,
            fundraiser: fs.fundraiser_pda,
            vault: fs.vault,
            system_program: system_program::id(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut fs.svm,
        vec![init_ix],
        &[&fs.maker],
        &fs.maker.pubkey(),
    )
    .unwrap();

    // Need 10 contributors each contributing 100 (10% of 1000) to reach goal
    for _ in 0..10 {
        let contributor = create_wallet(&mut fs.svm, 10_000_000_000).unwrap();

        let contributor_ata = create_associated_token_account(
            &mut fs.svm,
            &contributor.pubkey(),
            &fs.mint,
            &fs.payer,
        )
        .unwrap();

        mint_tokens_to_token_account(&mut fs.svm, &fs.mint, &contributor_ata, 10_000, &fs.payer)
            .unwrap();

        let (contributor_pda, _) = Pubkey::find_program_address(
            &[
                b"contributor",
                fs.fundraiser_pda.as_ref(),
                contributor.pubkey().as_ref(),
            ],
            &fs.program_id,
        );

        let contribute_ix = Instruction::new_with_bytes(
            fs.program_id,
            &fundraiser::instruction::Contribute { amount: 100 }.data(),
            fundraiser::accounts::Contribute {
                contributor: contributor.pubkey(),
                mint_to_raise: fs.mint,
                fundraiser: fs.fundraiser_pda,
                contributor_account: contributor_pda,
                contributor_ata,
                vault: fs.vault,
                token_program: token_program_id(),
                system_program: system_program::id(),
            }
            .to_account_metas(None),
        );
        send_transaction_from_instructions(
            &mut fs.svm,
            vec![contribute_ix],
            &[&contributor],
            &contributor.pubkey(),
        )
        .unwrap();

        // Check if we've hit the goal
        let current = get_token_account_balance(&fs.svm, &fs.vault).unwrap();
        if current >= amount_to_raise {
            break;
        }
    }

    // Verify vault has enough
    assert!(get_token_account_balance(&fs.svm, &fs.vault).unwrap() >= amount_to_raise);

    // Check contributions (maker claims the funds)
    let maker_ata = derive_ata(&fs.maker.pubkey(), &fs.mint);

    let check_ix = Instruction::new_with_bytes(
        fs.program_id,
        &fundraiser::instruction::CheckContributions {}.data(),
        fundraiser::accounts::CheckContributions {
            maker: fs.maker.pubkey(),
            mint_to_raise: fs.mint,
            fundraiser: fs.fundraiser_pda,
            vault: fs.vault,
            maker_ata,
            token_program: token_program_id(),
            system_program: system_program::id(),
            associated_token_program: ata_program_id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut fs.svm,
        vec![check_ix],
        &[&fs.maker],
        &fs.maker.pubkey(),
    )
    .unwrap();

    // Verify maker received the funds
    assert!(
        get_token_account_balance(&fs.svm, &maker_ata).unwrap() >= amount_to_raise
    );

    // Fundraiser account should be closed
    assert!(
        fs.svm.get_account(&fs.fundraiser_pda).is_none(),
        "Fundraiser account should be closed after check_contributions"
    );
}
