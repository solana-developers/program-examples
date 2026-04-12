extern crate std;
use {
    quasar_svm::QuasarSvm,
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_nft_operations.so").unwrap();
    QuasarSvm::new()
        .with_program(&crate::ID, &elf)
        .with_token_program()
}

// Note: All three instructions (create_collection, mint_nft, verify_collection)
// require the Metaplex Token Metadata program deployed in the SVM. The
// quasar-svm harness does not currently include it, so we verify the program
// builds and loads. Full integration testing requires a localnet deploy with
// the Metaplex program.

#[test]
fn test_program_builds() {
    let _svm = setup();
    println!("  NFT operations program loaded successfully");
}
