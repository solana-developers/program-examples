use solana_keypair::Keypair;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_signer::Signer;
use solana_transaction::Transaction;
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::{
    extension::{
        transfer_fee::{TransferFeeAmount, TransferFeeConfig},
        BaseStateWithExtensions, ExtensionType, StateWithExtensions,
    },
    instruction::mint_to,
    state::{Account, Mint},
};
use steel_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "steel_program",
        steel_api::ID,
        processor!(steel_program::process_instruction),
    );
    //Custom SPL Token 2022
    // program_test.add_program("spl_token_2022", spl_token_2022::ID, None);
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, sender, blockhash) = setup().await;

    let first_receiver = Keypair::new();
    let second_receiver = Keypair::new();

    let mint = Keypair::new();

    let decimals = 6;
    let maximum_fee = 500_000; //0.5 Token based on 6 decimals
    let transfer_fee_basis_points = 1000; //10%

    // 1. Initialize Transfer Fee Extension
    let ix = initialize(
        sender.pubkey(),
        mint.pubkey(),
        maximum_fee,
        transfer_fee_basis_points,
        decimals,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&sender.pubkey()),
        &[&sender, &mint],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //Get mint data
    let mint_account = banks.get_account(mint.pubkey()).await.unwrap().unwrap();

    let mint_data = mint_account.data.as_slice();
    let state_with_extensions = StateWithExtensions::<Mint>::unpack(mint_data).unwrap();
    let extension_types = state_with_extensions.get_extension_types().unwrap();
    let extension = state_with_extensions
        .get_extension::<TransferFeeConfig>()
        .unwrap();

    assert_eq!(
        extension.transfer_fee_config_authority.0,
        sender.pubkey(),
        "Transfer fee config authority mismatch"
    );

    assert!(
        extension_types.contains(&ExtensionType::TransferFeeConfig),
        "TransferFeeConfig extension not found in mint account"
    );

    //Derive sender's ata.
    let sender_ata = get_associated_token_address_with_program_id(
        &sender.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );

    //Mint Tokens
    let mint_ix = mint_to(
        &spl_token_2022::ID,
        &mint.pubkey(),
        &sender_ata,
        &sender.pubkey(),
        &[],
        1_000_000_000,
    )
    .unwrap();

    //create token accounts
    let create_token_acc_ix1 = create_associated_token_account(
        &sender.pubkey(),
        &sender.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );
    let create_token_acc_ix2 = create_associated_token_account(
        &sender.pubkey(),
        &first_receiver.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );

    let create_token_acc_ix3 = create_associated_token_account(
        &sender.pubkey(),
        &second_receiver.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );

    // Derive ATAs
    let first_receiver_ata = get_associated_token_address_with_program_id(
        &first_receiver.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );

    let second_receiver_ata = get_associated_token_address_with_program_id(
        &second_receiver.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );

    //2. Transfer with transfer_fee functionality
    let ix1 = transfer(
        1_000_000,
        sender.pubkey(),
        mint.pubkey(),
        sender_ata,
        first_receiver_ata,
    );
    let ix2 = transfer(
        500_000,
        sender.pubkey(),
        mint.pubkey(),
        sender_ata,
        second_receiver_ata,
    );

    let tx = Transaction::new_signed_with_payer(
        &[
            create_token_acc_ix1,
            create_token_acc_ix2,
            create_token_acc_ix3,
            mint_ix,
            ix1,
            ix2,
        ],
        Some(&sender.pubkey()),
        &[&sender],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Check the withheld amount on receiver accounts for expected transfer fees.
    //First receiver
    let receiver_acc = banks
        .get_account(first_receiver_ata)
        .await
        .unwrap()
        .unwrap();

    // let receiver_acc_data = receiver_acc.data.as_slice();
    let state_with_extensions =
        StateWithExtensions::<Account>::unpack(&receiver_acc.data.as_slice()).unwrap();
    let receiver_acc_data = state_with_extensions
        .get_extension::<TransferFeeAmount>()
        .unwrap();
    assert!(u64::from(receiver_acc_data.withheld_amount) == 100000);

    //Second receiver
    let receiver_acc = banks
        .get_account(second_receiver_ata)
        .await
        .unwrap()
        .unwrap();

    let receiver_token =
        StateWithExtensions::<Account>::unpack(&receiver_acc.data.as_slice()).unwrap();
    let receiver_acc_data = receiver_token.get_extension::<TransferFeeAmount>().unwrap();
    assert!(u64::from(receiver_acc_data.withheld_amount) == 50000);

    // 3. Harvest Rewards to Mint account
    let ix = harvest(
        sender.pubkey(),
        mint.pubkey(),
        first_receiver_ata,
        second_receiver_ata,
    );

    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&sender.pubkey()), &[&sender], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //Check if mint has received the harvested fees.
    let mint_account = banks.get_account(mint.pubkey()).await.unwrap().unwrap();

    let mint_data = mint_account.data.as_slice();
    let state_with_extensions = StateWithExtensions::<Mint>::unpack(mint_data).unwrap();
    let extension = state_with_extensions
        .get_extension::<TransferFeeConfig>()
        .unwrap();
    assert!(u64::from(extension.withheld_amount) == 150000);

    // 4. Withdraw Rewards from Mint account
    let ix = withdraw(sender.pubkey(), mint.pubkey(), first_receiver_ata);
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&sender.pubkey()), &[&sender], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //Check if the withdrawn amount is in the account
    let acc = banks
        .get_account(first_receiver_ata)
        .await
        .unwrap()
        .unwrap();

    let acc_data = StateWithExtensions::<Account>::unpack(&acc.data.as_slice()).unwrap();

    // 1,050,000 = Initial transfer of 1,000,000 (after 10% fee → 900,000)
    //            + 150,000 harvested and withdrawn from the mint
    assert!(acc_data.base.amount == 1050000);

    // 5. Update Transfer Fees
    let ix = update_fee(sender.pubkey(), mint.pubkey(), 200000, 2000);
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&sender.pubkey()), &[&sender], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
