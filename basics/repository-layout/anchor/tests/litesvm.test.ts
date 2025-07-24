import * as anchor from '@coral-xyz/anchor';
import { Transaction } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { Carnival } from '../target/types/carnival';

const IDL = require('../target/idl/carnival.json');

describe('anchor', () => {
  // Configure the Anchor provider & load the program IDL for LiteSVM
  // The IDL gives you a typescript module
  const client = fromWorkspace('');
  const provider = new LiteSVMProvider(client);
  const payer = provider.wallet.payer;
  const program = new anchor.Program<Carnival>(IDL, provider);

  async function sendCarnivalInstructions(instructionsList: anchor.web3.TransactionInstruction[]) {
    const tx = new Transaction();
    for (const ix of instructionsList) {
      tx.add(ix);
    }
    await provider.sendAndConfirm(tx, [payer]);
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
