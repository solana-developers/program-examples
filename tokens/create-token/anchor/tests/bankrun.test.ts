import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreateToken } from "../target/types/create_token"; // ts-mocha handles resolution
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { PublicKey } from "@solana/web3.js";
// import { MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
const MPL_TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
import { assert } from "chai";
import { readFileSync } from "fs";

describe("Create Token with Metadata", () => {
  it("Mints and adds metadata!", async () => {
    const context = await startAnchor(
      "", 
      [
        { name: "create_token", programId: new PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS") },
        { name: "mpl_token_metadata", programId: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s") }
      ],
      []
    );
    const provider = new BankrunProvider(context);
    anchor.setProvider(provider);
    
    // Load IDL
    const idl = JSON.parse(readFileSync("./target/idl/create_token.json", "utf-8"));
    const program = new Program<CreateToken>(idl, provider);

    const mintKeypair = anchor.web3.Keypair.generate();
    
    // Derive Metadata PDA (Crucial Step)
    const [metadataAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID).toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID)
    );

    await program.methods.createToken(
        "SuperToken", "SUP", "http://uri", new anchor.BN(1000)
    )
    .accounts({
        mint: mintKeypair.publicKey,
        metadataAccount: metadataAddress,
        tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
    })
    .signers([mintKeypair])
    .rpc();

    // Verify metadata account exists
    const accountInfo = await context.banksClient.getAccount(metadataAddress);
    assert.isNotNull(accountInfo, "Metadata account should exist");

    console.log("Success: Token Minted with Metadata!");
  });
});
