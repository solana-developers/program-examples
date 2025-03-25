// build.rs
use std::fs;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Create the fixtures directory path
    fs::create_dir_all("tests/fixtures").expect("Failed to create fixtures directory");

    let status = Command::new("solana")
        .args([
            "program",
            "dump",
            "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
            "tests/fixtures/token_metadata.so",
        ])
        .status()
        .expect("Failed to run solana program dump command");

    if !status.success() {
        panic!("Failed to dump Solana program");
    }
}
