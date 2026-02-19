import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey, LAMPORTS_PER_SOL} from "@solana/web3.js";
import { LiteSVMProvider } from 'anchor-litesvm';
import { LiteSVM } from 'litesvm';
import type { CreateToken } from "../target/types/create_token";

import IDL from "../target/idl/create_token.json";
const PROGRAM_ID = new PublicKey(IDL.address);
const METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
);

describe("Bankrun example", async () => {
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'target/deploy/create_token.so');
  svm.addProgramFromFile(METADATA_PROGRAM_ID, 'target/deploy/token_metadata.so');
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(100 * LAMPORTS_PER_SOL));
  const provider = new LiteSVMProvider(svm, new anchor.Wallet(payer));
  const wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CreateToken>(IDL, provider);

  const metadata = {
    name: "Solana Gold",
    symbol: "GOLDSOL",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
  };

  it("Create an SPL Token!", async () => {
    // Generate new keypair to use as address for mint account.
    const mintKeypair = new Keypair();

    // SPL Token default = 9 decimals
    const transactionSignature = await program.methods
      .createTokenMint(9, metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Success!");
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it("Create an NFT!", async () => {
    // Generate new keypair to use as address for mint account.
    const mintKeypair = new Keypair();

    // NFT default = 0 decimals
    const transactionSignature = await program.methods
      .createTokenMint(0, metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Success!");
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
