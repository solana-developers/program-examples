// build.rs
use std::fs;
use std::process::Command;

fn check_solana_installation() -> Result<(), String> {
    match Command::new("solana").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err("Solana CLI is available but returned an error".to_string())
            }
        }
        Err(e) => Err(format!("Solana CLI is not installed or not in PATH: {}", e)),
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Check if Solana is installed
    if let Err(err) = check_solana_installation() {
        println!("cargo:warning=Solana check failed: {}", err);
        return;
    }

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
