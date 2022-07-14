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

  const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  console.log(`New token: ${mintKeypair.publicKey}`);

  const testTokenTitle = "Solana Gold";
  const testTokenSymbol = "GOLDSOL";
  const testTokenUri = "https://raw.githubusercontent.com/solana-developers/program-examples/main/tokens/token_metadata.json";

  it("Mint!", async () => {

    const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];

    // Transact with the "create_token_mint" function in our on-chain program
    //
    await program.methods.createTokenMint(
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

  it("Mint to a wallet!", async () => {

    const amountToMint = 1;

    const tokenAddress = await anchor.utils.token.associatedAddress({
        mint: mintKeypair.publicKey,
        owner: payer.publicKey
    });

    // Transact with the "mint_to_wallet" function in our on-chain program
    //
    await program.methods.mintToWallet(new anchor.BN(amountToMint))
    .accounts({
      mintAccount: mintKeypair.publicKey,
      tokenAccount: tokenAddress,
      mintAuthority: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    })
    .signers([payer.payer, mintKeypair])
    .rpc();
});
});
