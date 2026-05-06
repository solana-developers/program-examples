use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_kite::{
        create_wallet, send_transaction_from_instructions,
        token_extensions::{
            create_token_extensions_account, get_token_extensions_account_address,
            mint_tokens_to_token_extensions_account, TOKEN_EXTENSIONS_PROGRAM_ID,
        },
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

fn associated_token_program_id() -> Pubkey {
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        .parse()
        .unwrap()
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = transfer_fee::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/transfer_fee.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_transfer_fee_full_flow() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();
    let recipient = Keypair::new();
    let ata_program = associated_token_program_id();

    let sender_ata = get_token_extensions_account_address(&payer.pubkey(), &mint_keypair.pubkey());
    let recipient_ata = get_token_extensions_account_address(&recipient.pubkey(), &mint_keypair.pubkey());

    // Step 1: Create mint with transfer fee (100 basis points = 1%, max fee = 1)
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_fee::instruction::Initialize {
            transfer_fee_basis_points: 100,
            maximum_fee: 1,
        }
        .data(),
        transfer_fee::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 2: Create sender ATA and mint 300 tokens
    create_token_extensions_account(
        &mut svm,
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint_keypair.pubkey(),
        &sender_ata,
        300,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    // Step 3: Transfer 100 tokens (fee = min(1% * 100 = 1, max_fee = 1) = 1)
    let transfer_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_fee::instruction::Transfer { amount: 100 }.data(),
        transfer_fee::accounts::Transfer {
            sender: payer.pubkey(),
            recipient: recipient.pubkey(),
            mint_account: mint_keypair.pubkey(),
            sender_token_account: sender_ata,
            recipient_token_account: recipient_ata,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            associated_token_program: ata_program,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![transfer_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 4: Transfer 200 tokens (fee = min(1% * 200 = 2, max_fee = 1) = 1, capped by maximumFee)
    let transfer_ix2 = Instruction::new_with_bytes(
        program_id,
        &transfer_fee::instruction::Transfer { amount: 200 }.data(),
        transfer_fee::accounts::Transfer {
            sender: payer.pubkey(),
            recipient: recipient.pubkey(),
            mint_account: mint_keypair.pubkey(),
            sender_token_account: sender_ata,
            recipient_token_account: recipient_ata,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            associated_token_program: ata_program,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![transfer_ix2], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 5: Harvest transfer fees from recipient token account to mint
    let harvest_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_fee::instruction::Harvest {}.data(),
        {
            let mut metas = transfer_fee::accounts::Harvest {
                mint_account: mint_keypair.pubkey(),
                token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            }
            .to_account_metas(None);
            metas.push(AccountMeta::new(recipient_ata, false));
            metas
        },
    );
    send_transaction_from_instructions(&mut svm, vec![harvest_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 6: Withdraw harvested fees from mint to sender's token account
    let withdraw_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_fee::instruction::Withdraw {}.data(),
        transfer_fee::accounts::Withdraw {
            authority: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_account: sender_ata,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![withdraw_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 7: Update transfer fee to 0
    let update_fee_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_fee::instruction::UpdateFee {
            transfer_fee_basis_points: 0,
            maximum_fee: 0,
        }
        .data(),
        transfer_fee::accounts::UpdateFee {
            authority: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![update_fee_ix], &[&payer], &payer.pubkey()).unwrap();
}
