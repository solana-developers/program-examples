import * as anchor from "@coral-xyz/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";

describe("Anchor example", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace
    .AnchorProgramExample as anchor.Program<AnchorProgramExample>;
  const payer = provider.wallet as anchor.Wallet;

  // We'll create this ahead of time.
  // Our program will try to modify it.
  const accountToChange = anchor.web3.Keypair.generate();
  // Our program will create this.
  const accountToCreate = anchor.web3.Keypair.generate();

  it("Create an account owned by our program", async () => {
    let ix = anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
      space: 0,
      programId: program.programId, // Our program
    });

    await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      new anchor.web3.Transaction().add(ix),
      [payer.payer, accountToChange]
    );
  });

  it("Check accounts", async () => {
    await program.methods
      .checkAccounts()
      .accounts({
        payer: provider.wallet.publicKey,
        accountToCreate: accountToCreate.publicKey,
        accountToChange: accountToChange.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer.payer])
      .rpc();
  });
});
