import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Anchor } from "../target/types/anchor";

const loadKeypairFromFile = (path: string): anchor.web3.Keypair => {
  return anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(require("fs").readFileSync(path, "utf-8")))
  );
};

describe("anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Anchor as Program<Anchor>;
  const connection = program.provider.connection;
  const TOKEN_2022_PROGRAM_ID = new anchor.web3.PublicKey(
    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
  );
  const payer = anchor.web3.Keypair.generate();
  const ATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );

  const tokenName = "TestToken";
  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("token-2022-token"),
      payer.publicKey.toBytes(),
      Buffer.from(tokenName),
    ],
    program.programId
  );
  const [payerATA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      payer.publicKey.toBytes(),
      TOKEN_2022_PROGRAM_ID.toBytes(),
      mint.toBytes(),
    ],
    ATA_PROGRAM_ID
  );

  const receiver = anchor.web3.Keypair.generate();

  connection.requestAirdrop(receiver.publicKey, 1000000000);

  const [receiverATA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      receiver.publicKey.toBytes(),
      TOKEN_2022_PROGRAM_ID.toBytes(),
      mint.toBytes(),
    ],
    ATA_PROGRAM_ID
  );

  it("Create Token-2022 Token", async () => {
    const tx = new anchor.web3.Transaction();

    const ix = await program.methods
      .createToken(tokenName)
      .accounts({
        mint: mint,
        signer: payer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .instruction();

    tx.add(ix);

    const sig = await anchor.web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [payer]
    );
    console.log("Your transaction signature", sig);
  });

  it("Initialize payer ATA", async () => {
    const tx = new anchor.web3.Transaction();

    const ix = await program.methods
      .createAssociatedTokenAccount()
      .accounts({
        tokenAccount: payerATA,
        mint: mint,
        signer: payer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .instruction();

    tx.add(ix);

    const sig = await anchor.web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [payer]
    );
    console.log("Your transaction signature", sig);
  });

  /*
  // Comment out because we use init in the transfer instruction
  it("Initialize receiver ATA", async () => {
    const tx = new anchor.web3.Transaction();

    const ix = await program.methods
      .createAssociatedTokenAccount()
      .accounts({
        tokenAccount: receiverATA,
        mint: mint,
        signer: receiver.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ATA_PROGRAM_ID,
      })
      .signers([receiver])
      .instruction();

    tx.add(ix);

    const sig = await anchor.web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [receiver]
    );
    console.log("Your transaction signature", sig);
  });
*/

  it("Mint Token to payer", async () => {
    const tx = new anchor.web3.Transaction();

    const ix = await program.methods
      .mintToken(new anchor.BN(200000000))
      .accounts({
        mint: mint,
        signer: payer.publicKey,
        receiver: payerATA,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([payer])
      .instruction();

    tx.add(ix);

    const sig = await anchor.web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [payer]
    );
    console.log("Your transaction signature", sig);
  });

  // init_if_needed not working with Token 22 ATA
  it("Transfer Token", async () => {
    const tx = new anchor.web3.Transaction();

    const ix = await program.methods
      .transferToken(new anchor.BN(100))
      .accounts({
        mint: mint,
        signer: payer.publicKey,
        from: payerATA,
        to: receiver.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ATA_PROGRAM_ID,
        toAta: receiverATA,
      })
      .instruction();

    tx.add(ix);

    const sig = await anchor.web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [payer]
    );
    console.log("Your transaction signature", sig);
  });
});
