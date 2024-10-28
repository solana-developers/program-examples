import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;

  // Generate new user keypairs for testing
  const user = Keypair.generate();

  let counterPDA: PublicKey;
  let counterBump: number;

  before(async () => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    // Airdrop 1 SOL to the user
    const airdropUser = await provider.connection.requestAirdrop(
      user.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropUser,
    });

    // Derive PDA for the counter account
    [counterPDA, counterBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      program.programId
    );
  });

  it("Initializes a counter account", async () => {
    
    // Invoke the Initialize Counter instruction from the program
    await program.methods
      .initializeCounter()
      .accountsPartial({
        payer: user.publicKey,
        counter: counterPDA
      })
      .signers([user])
      .rpc()

    // Fetch the counter account info
    const currentCounter = await program.account.counterAccount.fetch(counterPDA);
    
    // Assert and compare the account data
    assert.equal(currentCounter.count.toNumber(), 0, "Expected count to be 0");
  });

  it("Increments the counter account", async () => {
    
    // Invoke the Increment Counter instruction from the program
    await program.methods
      .incrementCounter()
      .accountsPartial({
        payer: user.publicKey,
        counter: counterPDA
      })
      .signers([user])
      .rpc()
    
    // Fetch the counter account info
    const currentCounter = await program.account.counterAccount.fetch(counterPDA);
    
    // Assert and compare the account data
    assert.equal(currentCounter.count.toNumber(), 1, "Expected count to be 1");
  });

  it("Increments the counter account again", async () => {
    
    // Invoke the Increment Counter instruction from the program
    await program.methods
      .incrementCounter()
      .accountsPartial({
        payer: user.publicKey,
        counter: counterPDA
      })
      .signers([user])
      .rpc()
    
    // Fetch the counter account info
    const currentCounter = await program.account.counterAccount.fetch(counterPDA);
    
    // Assert and compare the account data
    assert.equal(currentCounter.count.toNumber(), 2, "Expected count to be 2");
  });
});



  

 
