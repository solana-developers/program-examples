import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { CreateSystemAccountProgram } from '../target/types/create_system_account_program';

const IDL = require('../target/idl/create_system_account_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Create System Account Program', async () => {
  const context = await startAnchor('', [{ name: 'create-system-account', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CreateSystemAccountProgram>(IDL, payer);

  // Generate a new keypair to create the state account owned by our program
  const stateAccount = new Keypair();

  // Helper function to get account balances
  async function getBalances(payerPubkey: PublicKey, statePubkey: PublicKey, timeframe: string) {
    const payerBalance = await provider.connection.getBalance(payerPubkey);
    const stateBalance = await provider.connection.getBalance(statePubkey);
    console.log(`${timeframe} balances:`);
    console.log(`   Payer: ${payerBalance / LAMPORTS_PER_SOL}`);
    console.log(`   State Account: ${stateBalance / LAMPORTS_PER_SOL}`);
  }

  it('Initialize state account', async () => {
    await getBalances(payer.publicKey, stateAccount.publicKey, 'Beginning');

    await program.methods
      .initialize()
      .accounts({
        owner: payer.publicKey,
        state: stateAccount.publicKey,
        auth: payer.publicKey, // Using the payer as auth for this example
        systemProgram: SystemProgram.programId,
      })
      .signers([stateAccount]) // Sign with the state account
      .rpc();

    await getBalances(payer.publicKey, stateAccount.publicKey, 'After Initialization');

    // Fetch and check the initialized state
    const stateData = await program.account.accountState.fetch(stateAccount.publicKey);
    console.log(`Initialized state owner: ${stateData.owner.toString()}`);
    console.log(`Initialized state value: ${stateData.value}`); // Should be 0 if not set
  });

  it('Update state value', async () => {
    const newValue = 42; // Example value to update

    await program.methods
      .update(newValue)
      .accounts({
        state: stateAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    // Fetch the updated state
    const updatedState = await program.account.accountState.fetch(stateAccount.publicKey);
    console.log(`Updated value in state: ${updatedState.value}`);

    // Assert that the updated value matches the expected new value
    if (updatedState.value !== newValue) {
      throw new Error(`Expected value ${newValue}, but got ${updatedState.value}`);
    }
  });
});
