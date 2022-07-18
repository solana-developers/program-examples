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

  const testTokenTitle = "Solana Gold G";
  const testTokenSymbol = "GOLDSOL";
  const testTokenUri = "https://raw.githubusercontent.com/solana-developers/program-examples/main/tokens/token_metadata.json";

  const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  console.log(`New token: ${mintKeypair.publicKey}`);

  it("Create the mint", async () => {

    const [mintAuthorityPda, mintAuthorityPdaBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("mint_authority_"),
        mintKeypair.publicKey.toBuffer(),
      ],
      program.programId,
    );
    console.log(`PDA: ${mintAuthorityPda}`);
    console.log(`Bump: ${mintAuthorityPdaBump}`);

    const [metadataAddress, metadataBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    await program.methods.createTokenMint(
      testTokenTitle, testTokenSymbol, testTokenUri, mintAuthorityPdaBump
    )
    .accounts({
      metadataAccount: metadataAddress,
      mintAccount: mintKeypair.publicKey,
      mintAuthority: mintAuthorityPda,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .signers([mintKeypair, payer.payer])
    .rpc();
  });

  it("Mint to a wallet!", async () => {

    const [mintAuthorityPda, mintAuthorityPdaBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("mint_authority_"),
        mintKeypair.publicKey.toBuffer(),
      ],
      program.programId,
    );
    console.log(`PDA: ${mintAuthorityPda}`);
    console.log(`Bump: ${mintAuthorityPdaBump}`);

    const amountToMint = 1;

    const tokenAddress = await anchor.utils.token.associatedAddress({
        mint: mintKeypair.publicKey,
        owner: payer.publicKey
    });
    console.log(`Token Address: ${tokenAddress}`);

    await program.methods.mintToWallet(
      new anchor.BN(amountToMint), mintAuthorityPdaBump
    )
    .accounts({
      mintAccount: mintKeypair.publicKey,
      mintAuthority: mintAuthorityPda,
      tokenAccount: tokenAddress,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    })
    // .signers([payer.payer])
    .rpc();
  });
});
