use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

fn main() {
    let client = RpcClient::new("https://api.devnet.solana.com");
    let payer = Keypair::new();

    // Step 1: Create an account for HelloWorld
    let hello_world_account = Keypair::new();
    let program_id = Pubkey::new_unique();

    // Initialize instruction
    let initialize_instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(hello_world_account.pubkey(), false)],
        data: vec![], // The data for initialization, in this case, "Hello World!" is set inside the function
    };

    // Send transaction
    let transaction = Transaction::new_signed_with_payer(
        &[initialize_instruction],
        Some(&payer.pubkey()),
        &[&payer, &hello_world_account],
        client.get_recent_blockhash().unwrap(),
    );
    client.send_and_confirm_transaction(&transaction).unwrap();

    println!("Hello World account initialized with 'Hello World!' message.");
}
