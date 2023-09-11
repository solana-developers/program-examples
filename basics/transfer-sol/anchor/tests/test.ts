import * as anchor from "@coral-xyz/anchor";
import { TransferSol } from "../target/types/transfer_sol";

describe("transfer-sol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.TransferSol as anchor.Program<TransferSol>;

  const transferAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
  const test1Recipient = anchor.web3.Keypair.generate();
  const test2Recipient1 = anchor.web3.Keypair.generate();
  const test2Recipient2 = anchor.web3.Keypair.generate();

  it("Transfer between accounts using the system program", async () => {
    await getBalances(payer.publicKey, test1Recipient.publicKey, "Beginning");

    await program.methods
      .transferSolWithCpi(new anchor.BN(transferAmount))
      .accounts({
        payer: payer.publicKey,
        recipient: test1Recipient.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer.payer])
      .rpc();

    await getBalances(payer.publicKey, test1Recipient.publicKey, "Resulting");
  });

  it("Create two accounts for the following test", async () => {
    const ix = (pubkey: anchor.web3.PublicKey) => {
      return anchor.web3.SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: pubkey,
        space: 0,
        lamports: 2 * anchor.web3.LAMPORTS_PER_SOL,
        programId: program.programId,
      });
    };

    await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      new anchor.web3.Transaction()
        .add(ix(test2Recipient1.publicKey))
        .add(ix(test2Recipient2.publicKey)),
      [payer.payer, test2Recipient1, test2Recipient2]
    );
  });

  it("Transfer between accounts using our program", async () => {
    await getBalances(
      test2Recipient1.publicKey,
      test2Recipient2.publicKey,
      "Beginning"
    );

    await program.methods
      .transferSolWithProgram(new anchor.BN(transferAmount))
      .accounts({
        payer: test2Recipient1.publicKey,
        recipient: test2Recipient2.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    await getBalances(
      test2Recipient1.publicKey,
      test2Recipient2.publicKey,
      "Resulting"
    );
  });

  async function getBalances(
    payerPubkey: anchor.web3.PublicKey,
    recipientPubkey: anchor.web3.PublicKey,
    timeframe: string
  ) {
    let payerBalance = await provider.connection.getBalance(payerPubkey);
    let recipientBalance = await provider.connection.getBalance(
      recipientPubkey
    );
    console.log(`${timeframe} balances:`);
    console.log(`   Payer: ${payerBalance}`);
    console.log(`   Recipient: ${recipientBalance}`);
  }
});
