import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenSwapEscrow } from "../target/types/token_swap_escrow";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { randomBytes } from "crypto";

describe("token-swap-escrow", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenSwapEscrow as Program<TokenSwapEscrow>;

  // Generate Maker and Taker keypairs.
  let maker = Keypair.generate();
  let taker = Keypair.generate();

  // Prepare two mint addresses for the two tokens we'll be swapping.
  let mintX: PublicKey;
  let mintY: PublicKey;

  // Prepare two maker ATA addresses for the two tokens we'll be swapping.
  let makerAtaX: PublicKey;
  let makerAtaY: PublicKey;

  // Prepare two taker ATA addresses for the two tokens we'll be swapping.
  let takerAtaX: PublicKey;
  let takerAtaY: PublicKey;

  // Generate a random seed.
  const seed = new anchor.BN(randomBytes(8));

  // Generate the escrow PDA address.
  let escrow = findProgramAddressSync([Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toBuffer('le', 8)], program.programId)[0];

  // Prepare a vault address.
  let vault: PublicKey;

  // Helper function to confirm a transaction.
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

  // Request an airdrop for our maker.
  it("Airdrop SOL to Escrow Maker", async () => {
    const tx = await anchor.getProvider().connection.requestAirdrop(maker.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL).then(confirmTx);

    console.log("\n\nAirdrop to maker successful! TxID: ", tx);
  });

  // Request an airdrop for our taker.
  it("Airdrop SOL to Escrow Taker", async () => {
    const tx = await anchor.getProvider().connection.requestAirdrop(taker.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL).then(confirmTx);

    console.log("\n\nAirdrop to taker successful! TxID: ", tx);
  });

  it("Create mint and mint some tokens", async () => {
    // Create Mint X and Mint Y.
    mintX = await createMint(provider.connection, maker, maker.publicKey, null, 6);
    console.log("\n\nMint X created: ", mintX.toBase58());
    mintY = await createMint(provider.connection, maker, maker.publicKey, null, 6);
    console.log("Mint Y created: ", mintY.toBase58());

    // Get the vault ATA
    vault = getAssociatedTokenAddressSync(mintX, escrow, true);
    console.log("\nVault created: ", vault.toBase58());

    // Get the maker ATAs
    makerAtaX = (await getOrCreateAssociatedTokenAccount(provider.connection, maker, mintX, maker.publicKey)).address;
    console.log("\nMaker ATA X created: ", makerAtaX.toBase58());
    makerAtaY = getAssociatedTokenAddressSync(mintY, maker.publicKey);
    console.log("Maker ATA Y created: ", makerAtaY.toBase58());

    // Get the taker ATAs
    takerAtaX = getAssociatedTokenAddressSync(mintX, taker.publicKey);
    console.log("\nTaker ATA X created: ", takerAtaX.toBase58());
    takerAtaY = (await getOrCreateAssociatedTokenAccount(provider.connection, maker, mintY, taker.publicKey)).address;
    console.log("Taker ATA Y created: ", takerAtaY.toBase58());

    // Mint some tokens X to the maker ATA
    const makerMintA = await mintTo(provider.connection, maker, mintX, makerAtaX, maker, 1000000000);
    console.log("\nMinted 100 tokens X to maker ATA - TxID: ", makerMintA);

    // Mint some tokens Y to the taker ATA
    const takerMintY = await mintTo(provider.connection, maker, mintY, takerAtaY, maker, 1000000000);
    console.log("\nMinted 100 tokens Y to taker ATA - TxID: ", takerMintY);
  })

  it("Create Escrow", async () => {
    const tx = await program.methods.make(seed, new anchor.BN(100000000), new anchor.BN(100000000))
    .accounts({
      maker: maker.publicKey,
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
    .signers([maker])
    .rpc();
    console.log("\n\nEscrow Created! TxID: ", tx);

    // Fetch and log details
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
      maker: maker.publicKey,
      mintX,
      mintY,
      vault,
      makerAtaX,
      escrow,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([maker])
    .rpc();

    console.log("\nEscrow Refunded! TxID: ", tx);

    // Fetch and log details
    console.log("\nAmount in maker ATA X after refunding escrow: ", (await provider.connection.getTokenAccountBalance(makerAtaX)).value.uiAmount, " Tokens");
  });

  it("Take Escrow", async () => {
    console.log("\n\nAmount in taker ATA Y before taking escrow: ", (await provider.connection.getTokenAccountBalance(takerAtaY)).value.uiAmount, " Tokens");

    const tx = await program.methods.take()
    .accounts({
      taker: taker.publicKey,
      maker: maker.publicKey,
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

    // Fetch and log details
    console.log("\nAmount in taker ATA X after taking escrow: ", (await provider.connection.getTokenAccountBalance(takerAtaX)).value.uiAmount, " Tokens");
    console.log("Amount in taker ATA Y after taking escrow: ", (await provider.connection.getTokenAccountBalance(takerAtaY)).value.uiAmount, " Tokens");
  });
});
