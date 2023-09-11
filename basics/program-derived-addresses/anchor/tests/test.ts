import * as anchor from "@coral-xyz/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";

describe("PDAs", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace
    .AnchorProgramExample as anchor.Program<AnchorProgramExample>;

  let testUser = anchor.web3.Keypair.generate();

  it("Create a test user", async () => {
    let ix = anchor.web3.SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
      newAccountPubkey: testUser.publicKey,
      programId: anchor.web3.SystemProgram.programId,
      space: 0,
    });
    await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      new anchor.web3.Transaction().add(ix),
      [payer.payer, testUser]
    );
    console.log(`Local Wallet: ${payer.publicKey}`);
    console.log(`Created User: ${testUser.publicKey}`);
  });

  function derivePageVisitsPda(userPubkey: anchor.web3.PublicKey) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("page_visits"), userPubkey.toBuffer()],
      program.programId
    )[0];
  }

  it("Create the page visits tracking PDA", async () => {
    await program.methods
      .createPageVisits()
      .accounts({
        pageVisits: derivePageVisitsPda(testUser.publicKey),
        user: testUser.publicKey,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer.payer])
      .rpc();
  });

  it("Visit the page!", async () => {
    await program.methods
      .incrementPageVisits()
      .accounts({
        pageVisits: derivePageVisitsPda(testUser.publicKey),
        user: testUser.publicKey,
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();
  });

  it("Visit the page!", async () => {
    await program.methods
      .incrementPageVisits()
      .accounts({
        pageVisits: derivePageVisitsPda(testUser.publicKey),
        user: testUser.publicKey,
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();
  });

  it("View page visits", async () => {
    const pageVisits = await program.account.pageVisits.fetch(
      await derivePageVisitsPda(testUser.publicKey)
    );
    console.log(`Number of page visits: ${pageVisits.pageVisits}`);
  });
});
