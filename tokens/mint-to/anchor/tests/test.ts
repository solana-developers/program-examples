import * as anchor from "@project-serum/anchor";
import { MintTokenTo } from "../target/types/mint_token_to";


const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);


describe("mint-token", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.MintTokenTo as anchor.Program<MintTokenTo>;

  it("Mint!", async () => {

    const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    console.log(`New token: ${mintKeypair.publicKey}`);

    // Derive the metadata account's address and set the metadata
    //
    const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];
    const testTokenTitle = "Solana Gold";
    const testTokenSymbol = "GOLDSOL";
    const testTokenUri = "https://raw.githubusercontent.com/solana-developers/program-examples/main/tokens/mint/native/assets/token_metadata.json";

    // Transact with the "mint_token" function in our on-chain program
    //
    await program.methods.mintToken(
      testTokenTitle, testTokenSymbol, testTokenUri
    )
    .accounts({
      metadataAccount: metadataAddress,
      mintAccount: mintKeypair.publicKey,
      mintAuthority: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .signers([payer.payer, mintKeypair])
    .rpc();
  });
});
