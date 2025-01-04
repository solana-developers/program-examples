use pda_mint_authority_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "pda_mint_authority_program",
        pda_mint_authority_api::ID,
        processor!(pda_mint_authority_program::process_instruction),
    );

    program_test.add_program("token_metadata", mpl_token_metadata::ID, None);

    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let token_mint_keypair = Keypair::new();

    let name = str_to_bytes::<32>("Solana Gold");
    let symbol = str_to_bytes::<8>("GOLDSOL");
    let uri = str_to_bytes::<64>("https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json");

    // Submit init transaction.
    let ix = init(payer.pubkey());
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Submit create transaction for spl token.
    let ix = create(
        payer.pubkey(),
        token_mint_keypair.pubkey(),
        name,
        symbol,
        uri,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &token_mint_keypair],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let to_ata = spl_associated_token_account::get_associated_token_address(
        &payer.pubkey(),
        &token_mint_keypair.pubkey(),
    );

    // Submit mint transaction.
    let ix = mint(payer.pubkey(), token_mint_keypair.pubkey(), to_ata, 100);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}

pub fn str_to_bytes<const N: usize>(str: &str) -> [u8; N] {
    let mut str_bytes = [0u8; N];
    let copy_len = str.len().min(N);
    str_bytes[..copy_len].copy_from_slice(&str.as_bytes()[..copy_len]);
    str_bytes
}
