import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import { TransferSolProgram } from "../target/types/transfer_sol_program";

const IDL = require('../target/idl/transfer_sol.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun example', async () => {
  const context = await startAnchor('', [{ name: 'transfer_sol', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<TransferSolProgram>(IDL, provider);

  // 1 SOL
  const transferAmount = 1 * LAMPORTS_PER_SOL;

  // Generate a new keypair for the recipient
  const recipient = new Keypair();


  it('Transfer SOL with CPI', async () => {
    await getBalances(payer.publicKey, recipient.publicKey, 'Beginning');

    await program.methods
      .initialize(new anchor.BN(transferAmount))
      .accounts({
        sender: payer.publicKey,
        receiver: recipient.publicKey,
      })
      .rpc();

    await getBalances(payer.publicKey, recipient.publicKey, 'Resulting');
  });


  async function getBalances(payerPubkey: PublicKey, recipientPubkey: PublicKey, timeframe: string) {
    const payerBalance = await provider.connection.getBalance(payerPubkey);
    const recipientBalance = await provider.connection.getBalance(recipientPubkey);
    console.log(`${timeframe} balances:`);
    console.log(`   Payer: ${payerBalance / LAMPORTS_PER_SOL}`);
    console.log(`   Recipient: ${recipientBalance / LAMPORTS_PER_SOL}`);
  }
});