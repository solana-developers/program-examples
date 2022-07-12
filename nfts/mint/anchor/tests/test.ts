import * as anchor from "@project-serum/anchor";
import { MintNft } from "../target/types/mint_nft";


const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);


describe("mint-NFT", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.MintNft as anchor.Program<MintNft>;

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
    const testNftTitle = "Solana Platinum NFT";
    const testNftSymbol = "SOLP";
    const testNftUri = "https://raw.githubusercontent.com/solana-developers/program-examples/main/nfts/mint/anchor/assets/token_metadata.json";

    // Transact with the "mint_token" function in our on-chain program
    //
    await program.methods.mintToken(
      testNftTitle, testNftSymbol, testNftUri
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
