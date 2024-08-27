import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { Carnival } from '../target/types/carnival';

const IDL = require('../target/idl/carnival.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun example', async () => {
  const context = await startAnchor('', [{ name: 'carnival', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<Carnival>(IDL, provider);

  async function sendCarnivalInstructions(instructionsList: anchor.web3.TransactionInstruction[]) {
    const tx = new Transaction();
    for (const ix of instructionsList) {
      tx.add(ix);
    }
    await provider.sendAndConfirm(tx, [wallet.payer]);
  }

  it('Go on some rides!', async () => {
    await sendCarnivalInstructions([
      await program.methods.goOnRide('Jimmy', 36, 15, 'Scrambler').instruction(),
      await program.methods.goOnRide('Mary', 52, 1, 'Ferris Wheel').instruction(),
      await program.methods.goOnRide('Alice', 56, 15, 'Scrambler').instruction(),
      await program.methods.goOnRide('Bob', 49, 6, 'Tilt-a-Whirl').instruction(),
    ]);
  });

  it('Play some games!', async () => {
    await sendCarnivalInstructions([
      await program.methods.playGame('Jimmy', 15, 'I Got It!').instruction(),
      await program.methods.playGame('Mary', 1, 'Ring Toss').instruction(),
      await program.methods.playGame('Alice', 15, 'Ladder Climb').instruction(),
      await program.methods.playGame('Bob', 6, 'Ring Toss').instruction(),
    ]);
  });

  it('Eat some food!', async () => {
    await sendCarnivalInstructions([
      await program.methods.eatFood('Jimmy', 15, 'Taco Shack').instruction(),
      await program.methods.eatFood('Mary', 1, "Larry's Pizza").instruction(),
      await program.methods.eatFood('Alice', 15, "Dough Boy's").instruction(),
      await program.methods.eatFood('Bob', 6, "Dough Boy's").instruction(),
    ]);
  });
});
