import * as anchor from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { TransferSol } from '../target/types/transfer_sol';

const IDL = require('../target/idl/transfer_sol.json');

describe('transfer-sol', async () => {
  const client = fromWorkspace('');
  const provider = new LiteSVMProvider(client);
  const payer = provider.wallet.payer;
  const program = new anchor.Program<TransferSol>(IDL, provider);

  // airdrop the payer account 2 SOL
  client.airdrop(payer.publicKey, BigInt(2 * 1000000000));

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

    const tx = new Transaction();
    tx.recentBlockhash = client.latestBlockhash();
    tx.add(instruction).sign(payer, programOwnedAccount);

    client.sendTransaction(tx);
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
    const payerBalance = provider.client.getBalance(payerPubkey);
    const recipientBalance = provider.client.getBalance(recipientPubkey);
    console.log(`${timeframe} balances:`);

    console.log(`   Payer: ${Number(payerBalance) / LAMPORTS_PER_SOL}`);
    console.log(`   Recipient: ${Number(recipientBalance) / LAMPORTS_PER_SOL}`);
  }
});
