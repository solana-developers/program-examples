import * as anchor from "@project-serum/anchor";
import { Mint2 } from "../target/types/mint_2";


const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);


describe("mint-token", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.Mint2 as anchor.Program<Mint2>;

  const testTokenTitle = "Solana Gold";
  const testTokenSymbol = "GOLDSOL";
  const testTokenUri = "https://raw.githubusercontent.com/solana-developers/program-examples/main/tokens/mint-2/anchor/tests/token_metadata.json";

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

    const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];

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

  it("Mint to your wallet!", async () => {

    const [mintAuthorityPda, mintAuthorityPdaBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("mint_authority_"),
        mintKeypair.publicKey.toBuffer(),
      ],
      program.programId,
    );

    const amountToMint = 1;

    const tokenAddress = await anchor.utils.token.associatedAddress({
        mint: mintKeypair.publicKey,
        owner: payer.publicKey
    });
    console.log(`Token Address: ${tokenAddress}`);

    await program.methods.mintToYourWallet(
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
    .signers([payer.payer])
    .rpc();
  });

  it("Mint to another person's wallet (airdrop)!", async () => {

    const recipientKeypair = anchor.web3.Keypair.generate();
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(recipientKeypair.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL)
    );
    console.log(`Recipient pubkey: ${recipientKeypair.publicKey}`);

    const [mintAuthorityPda, mintAuthorityPdaBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("mint_authority_"),
        mintKeypair.publicKey.toBuffer(),
      ],
      program.programId,
    );

    const amountToMint = 1;

    const tokenAddress = await anchor.utils.token.associatedAddress({
        mint: mintKeypair.publicKey,
        owner: recipientKeypair.publicKey
    });
    console.log(`Token Address: ${tokenAddress}`);

    await program.methods.mintToAnotherWallet(
      new anchor.BN(amountToMint), mintAuthorityPdaBump
    )
    .accounts({
      mintAccount: mintKeypair.publicKey,
      mintAuthority: mintAuthorityPda,
      recipient: recipientKeypair.publicKey,
      tokenAccount: tokenAddress,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    })
    .signers([payer.payer])
    .rpc();
  });

  it("Transfer to another person's wallet!", async () => {

    const recipientWallet = anchor.web3.Keypair.generate();
    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(recipientWallet.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    );
    console.log(`Recipient Pubkey: ${recipientWallet.publicKey}`);

    const amountToTransfer = 1;

    const ownerTokenAddress = await anchor.utils.token.associatedAddress({
        mint: mintKeypair.publicKey,
        owner: payer.publicKey
    });
    console.log(`Owner Token Address: ${ownerTokenAddress}`);
    const recipientTokenAddress = await anchor.utils.token.associatedAddress({
        mint: mintKeypair.publicKey,
        owner: recipientWallet.publicKey
    });
    console.log(`Recipient Token Address: ${recipientTokenAddress}`);

    await program.methods.transferToAnotherWallet(
      new anchor.BN(amountToTransfer)
    )
    .accounts({
      mintAccount: mintKeypair.publicKey,
      ownerTokenAccount: ownerTokenAddress,
      recipientTokenAccount: recipientTokenAddress,
      owner: payer.publicKey,
      recipient: recipientWallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    })
    .signers([payer.payer])
    .rpc();
  });
});
