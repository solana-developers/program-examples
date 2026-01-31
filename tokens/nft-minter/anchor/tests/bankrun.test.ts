import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { Keypair, PublicKey, LAMPORTS_PER_SOL} from "@solana/web3.js";
import { LiteSVMProvider } from 'anchor-litesvm';
import { LiteSVM } from 'litesvm';
import type { NftMinter } from "../target/types/nft_minter";

import IDL from "../target/idl/nft_minter.json";
const PROGRAM_ID = new PublicKey(IDL.address);
const METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
);

describe("NFT litesvm Minter", async () => {
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'target/deploy/nft_minter.so');
  svm.addProgramFromFile(METADATA_PROGRAM_ID, 'target/deploy/token_metadata.so');
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(100 * LAMPORTS_PER_SOL));
  const provider = new LiteSVMProvider(svm, new anchor.Wallet(payer));
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<NftMinter>(IDL, provider);

  // The metadata for our NFT
  const metadata = {
    name: "Homer NFT",
    symbol: "HOMR",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json",
  };

  it("Create an NFT!", async () => {
    // Generate a keypair to use as the address of our mint account
    const mintKeypair = new Keypair();

    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
      mintKeypair.publicKey,
      payer.publicKey,
    );

    const transactionSignature = await program.methods
      .mintNft(metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
        associatedTokenAccount: associatedTokenAccountAddress,
      })
      .signers([mintKeypair])
      .rpc({ skipPreflight: true });

    console.log("Success!");
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
