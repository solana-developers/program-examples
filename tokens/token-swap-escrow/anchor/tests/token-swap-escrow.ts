import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenSwapEscrow } from "../target/types/token_swap_escrow";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddress, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { randomBytes } from "crypto";

describe("token-swap-escrow", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  let providerWallet = provider.wallet as NodeWallet;

  const program = anchor.workspace.TokenSwapEscrow as Program<TokenSwapEscrow>;

  let mintX: PublicKey;
  let mintY: PublicKey;

  let makerAtaX: PublicKey;
  let makerAtaY: PublicKey;

  let takerAtaX: PublicKey;
  let takerAtaY: PublicKey;

  let taker = Keypair.generate();

  const seed = new anchor.BN(randomBytes(8));
  let escrow = findProgramAddressSync([Buffer.from("escrow"), providerWallet.publicKey.toBuffer(), seed.toBuffer('le', 8)], program.programId)[0];
  let vault: PublicKey;

  const confirmTx = async (signature: string) => {
    const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      "confirmed"
    )
    return signature
  }

  // Request an airdrop for our taker.
  it("Airdrop SOL to Escrow Taker", async () => {
    const tx = await anchor.getProvider().connection.requestAirdrop(taker.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL).then(confirmTx);

    console.log("\n\nAirdrop to taker successful! TxID: ", tx);
  });

  it("Create mint and mint some tokens", async () => {
    // Create Mint X and Mint Y.
    mintX = await createMint(provider.connection, providerWallet.payer, provider.publicKey, null, 6);
    console.log("\n\nMint X created: ", mintX.toBase58());
    mintY = await createMint(provider.connection, providerWallet.payer, provider.publicKey, null, 6);
    console.log("Mint Y created: ", mintY.toBase58());

    // Get the vault ATA
    vault = getAssociatedTokenAddressSync(mintX, escrow, true);
    console.log("\nVault created: ", vault.toBase58());

    // Get the maker ATAs
    makerAtaX = (await getOrCreateAssociatedTokenAccount(provider.connection, providerWallet.payer, mintX, providerWallet.publicKey)).address;
    console.log("\nMaker ATA X created: ", makerAtaX.toBase58());
    makerAtaY = getAssociatedTokenAddressSync(mintY, providerWallet.publicKey);
    console.log("Maker ATA Y created: ", makerAtaY.toBase58());

    // Get the taker ATAs
    takerAtaX = getAssociatedTokenAddressSync(mintX, taker.publicKey);
    console.log("\nTaker ATA X created: ", takerAtaX.toBase58());
    takerAtaY = (await getOrCreateAssociatedTokenAccount(provider.connection, providerWallet.payer, mintY, taker.publicKey)).address;
    console.log("Taker ATA Y created: ", takerAtaY.toBase58());

    // Mint some tokens X to the maker ATA
    const makerMintA = await mintTo(provider.connection, providerWallet.payer, mintX, makerAtaX, providerWallet.payer, 1000000000);
    console.log("\nMinted 100 tokens X to maker ATA - TxID: ", makerMintA);

    // Mint some tokens Y to the taker ATA
    const takerMintY = await mintTo(provider.connection, providerWallet.payer, mintY, takerAtaY, providerWallet.payer, 1000000000);
    console.log("\nMinted 100 tokens Y to taker ATA - TxID: ", takerMintY);
  })

  it("Create Escrow", async () => {
    // Add your test here.
    const tx = await program.methods.make(seed, new anchor.BN(100000000), new anchor.BN(100000000))
    .accounts({
      maker: providerWallet.publicKey,
      mintX,
      mintY,
      escrow,
      vault,
      makerAtaX,
      makerAtaY,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .rpc();
    console.log("\n\nEscrow Created! TxID: ", tx);

    let escrowAccount = await program.account.escrow.fetch(escrow);
    console.log("\nEscrow Account mint X: ", escrowAccount.mintX.toBase58());
    console.log("Escrow Account mint Y: ", escrowAccount.mintY.toBase58());
    console.log("Escrow Account amount: ", escrowAccount.amount.toString());

    console.log("\nAmount deposited in the vault: ", (await provider.connection.getTokenAccountBalance(vault)).value.uiAmount, " Tokens");
  });

  xit("Refund Escrow", async () => {
    console.log("\n\nAmount in maker ATA X before refunding escrow: ", (await provider.connection.getTokenAccountBalance(makerAtaX)).value.uiAmount, " Tokens");

    const tx = await program.methods.refund()
    .accounts({
      maker: providerWallet.publicKey,
      mintX,
      mintY,
      vault,
      makerAtaX,
      escrow,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();

    console.log("\nEscrow Refunded! TxID: ", tx);
    console.log("\nAmount in maker ATA X after refunding escrow: ", (await provider.connection.getTokenAccountBalance(makerAtaX)).value.uiAmount, " Tokens");
  });

  it("Take Escrow", async () => {
    console.log("\n\nAmount in taker ATA Y before taking escrow: ", (await provider.connection.getTokenAccountBalance(takerAtaY)).value.uiAmount, " Tokens");

    const tx = await program.methods.take()
    .accounts({
      taker: taker.publicKey,
      maker: providerWallet.publicKey,
      mintX,
      mintY,
      escrow,
      takerMintXAta: takerAtaX,
      takerMintYAta: takerAtaY,
      makerMintYAta: makerAtaY,
      vault,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([taker])
    .rpc();
    console.log("\nEscrow Taken! TxID: ", tx);

    console.log("\nAmount in taker ATA X after taking escrow: ", (await provider.connection.getTokenAccountBalance(takerAtaX)).value.uiAmount, " Tokens");
    console.log("Amount in taker ATA Y after taking escrow: ", (await provider.connection.getTokenAccountBalance(takerAtaY)).value.uiAmount, " Tokens");
  });
});
