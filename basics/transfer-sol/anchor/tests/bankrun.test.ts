import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { TransferSol } from '../target/types/transfer_sol';

const IDL = require('../target/idl/transfer_sol.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun example', async () => {
  const context = await startAnchor('', [{ name: 'transfer_sol', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<TransferSol>(IDL, provider);

  // 1 SOL
  const transferAmount = 1 * LAMPORTS_PER_SOL;

  // Generate a new keypair for the recipient
  const recipient = new Keypair();

  // Generate a new keypair to create an account owned by our program
  const programOwnedAccount = new Keypair();

  it('Transfer SOL with CPI', async () => {
    await getBalances(payer.publicKey, recipient.publicKey, 'Beginning');

    await program.methods
      .transferSolWithCpi(new anchor.BN(transferAmount))
      .accounts({
        payer: payer.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();

    await getBalances(payer.publicKey, recipient.publicKey, 'Resulting');
  });

  it('Create and fund account owned by our program', async () => {
    const instruction = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: programOwnedAccount.publicKey,
      space: 0,
      lamports: 1 * LAMPORTS_PER_SOL, // 1 SOL
      programId: program.programId, // Program Owner, our program's address
    });

    const transaction = new Transaction().add(instruction);

    await sendAndConfirmTransaction(provider.connection, transaction, [payer.payer, programOwnedAccount]);
  });

  it('Transfer SOL with Program', async () => {
    await getBalances(programOwnedAccount.publicKey, payer.publicKey, 'Beginning');

    await program.methods
      .transferSolWithProgram(new anchor.BN(transferAmount))
      .accounts({
        payer: programOwnedAccount.publicKey,
        recipient: payer.publicKey,
      })
      .rpc();

    await getBalances(programOwnedAccount.publicKey, payer.publicKey, 'Resulting');
  });

  async function getBalances(payerPubkey: PublicKey, recipientPubkey: PublicKey, timeframe: string) {
    const payerBalance = await provider.connection.getBalance(payerPubkey);
    const recipientBalance = await provider.connection.getBalance(recipientPubkey);
    console.log(`${timeframe} balances:`);
    console.log(`   Payer: ${payerBalance / LAMPORTS_PER_SOL}`);
    console.log(`   Recipient: ${recipientBalance / LAMPORTS_PER_SOL}`);
  }
});
