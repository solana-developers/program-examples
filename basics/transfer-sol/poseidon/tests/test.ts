import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, type PublicKey } from '@solana/web3.js';
import { TransferSolProgram } from '../target/types/transfer_sol_program';

describe('transfer-sol', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CreateAccount as Program<TransferSolProgram>;
  const transferAmount = 1 * LAMPORTS_PER_SOL;
  const recipient = new Keypair();

  it('It transfer sol!', async () => {
    await getBalances(payer.publicKey, recipient.publicKey, 'Beginning');

    // Add your test here.
    const tx = await program.methods
      .initialize(new anchor.BN(transferAmount))
      .accounts({
        sender: payer.publicKey,
        receiver: recipient.publicKey,
      })
      .rpc();

    console.log('Your transaction signature', tx);

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
