import { 
  PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID
} from '@metaplex-foundation/mpl-token-metadata';
import * as anchor from "@project-serum/anchor";
import { ASSOCIATED_PROGRAM_ID } from '@project-serum/anchor/dist/cjs/utils/token';
import { TransferTokens } from "../target/types/transfer_tokens";


describe("Transfer Tokens", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.TransferTokens as anchor.Program<TransferTokens>;

  const tokenMintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  const nftMintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  
  const recipientWallet: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  it("Create an SPL Token!", async () => {

    const metadataAddress = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        tokenMintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )[0];

    const sx = await program.methods.createToken(
      "Solana Gold",
      "GOLDSOL",
      "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
      9,
    )
      .accounts({
        metadataAccount: metadataAddress,
        mintAccount: tokenMintKeypair.publicKey,
        mintAuthority: payer.publicKey,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([tokenMintKeypair, payer.payer])
      .rpc();

    console.log("Success!");
        console.log(`   Mint Address: ${tokenMintKeypair.publicKey}`);
        console.log(`   Tx Signature: ${sx}`);
  });

  it("Create an NFT!", async () => {

    const metadataAddress = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        nftMintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )[0];

    const sx = await program.methods.createToken(
      "Homer NFT",
      "HOMR",
      "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json",
      0,
    )
      .accounts({
        metadataAccount: metadataAddress,
        mintAccount: nftMintKeypair.publicKey,
        mintAuthority: payer.publicKey,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([nftMintKeypair, payer.payer])
      .rpc();

    console.log("Success!");
        console.log(`   Mint Address: ${nftMintKeypair.publicKey}`);
        console.log(`   Tx Signature: ${sx}`);
  });

  it("Mint some tokens to your wallet!", async () => {

    const associatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: tokenMintKeypair.publicKey,
      owner: payer.publicKey,
    });

    const sx = await program.methods.mintSpl(
      new anchor.BN(150)
    )
      .accounts({
        associatedTokenAccount: associatedTokenAccountAddress,
        mintAccount: tokenMintKeypair.publicKey,
        mintAuthority: payer.publicKey,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      })
      .signers([payer.payer])
      .rpc();

    console.log("Success!");
        console.log(`   Mint Address: ${tokenMintKeypair.publicKey}`);
        console.log(`   Tx Signature: ${sx}`);
  });

  it("Mint the NFT to your wallet!", async () => {

    const metadataAddress = (anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          nftMintKeypair.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
    ))[0];

    const editionAddress = (anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          nftMintKeypair.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
    ))[0];

    const associatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: nftMintKeypair.publicKey,
      owner: payer.publicKey,
    });

    const sx = await program.methods.mintNft()
      .accounts({
        associatedTokenAccount: associatedTokenAccountAddress,
        editionAccount: editionAddress,
        metadataAccount: metadataAddress,
        mintAccount: nftMintKeypair.publicKey,
        mintAuthority: payer.publicKey,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([payer.payer])
      .rpc();

      console.log("Success!");
      console.log(`   ATA Address: ${associatedTokenAccountAddress}`);
      console.log(`   Tx Signature: ${sx}`);
  });

  it("Prep a new test wallet for transfers", async () => {
        
    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(
            recipientWallet.publicKey, 
            await provider.connection.getMinimumBalanceForRentExemption(0),
        )
    );
    console.log(`Recipient Pubkey: ${recipientWallet.publicKey}`);
  });

  it("Transfer some tokens to another wallet!", async () => {

    const fromAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: tokenMintKeypair.publicKey,
      owner: payer.publicKey,
    });
    const toAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: tokenMintKeypair.publicKey,
      owner: recipientWallet.publicKey,
    });

    const sx = await program.methods.transferTokens(
      new anchor.BN(150)
    )
      .accounts({
        mintAccount: tokenMintKeypair.publicKey,
        fromAssociatedTokenAccount: fromAssociatedTokenAccountAddress,
        owner: payer.publicKey,
        toAssociatedTokenAccount: toAssociatedTokenAccountAddress,
        recipient: recipientWallet.publicKey,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      })
      .signers([payer.payer])
      .rpc();

    console.log("Success!");
        console.log(`   Mint Address: ${tokenMintKeypair.publicKey}`);
        console.log(`   Tx Signature: ${sx}`);
  });

  it("Transfer the NFT to another wallet!", async () => {

    const fromAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: nftMintKeypair.publicKey,
      owner: payer.publicKey,
    });
    const toAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: nftMintKeypair.publicKey,
      owner: recipientWallet.publicKey,
    });

    const sx = await program.methods.transferTokens(
      new anchor.BN(1)
    )
      .accounts({
        mintAccount: nftMintKeypair.publicKey,
        fromAssociatedTokenAccount: fromAssociatedTokenAccountAddress,
        owner: payer.publicKey,
        toAssociatedTokenAccount: toAssociatedTokenAccountAddress,
        recipient: recipientWallet.publicKey,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      })
      .signers([payer.payer])
      .rpc();

    console.log("Success!");
        console.log(`   Mint Address: ${nftMintKeypair.publicKey}`);
        console.log(`   Tx Signature: ${sx}`);
  });
});
