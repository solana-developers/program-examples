import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { Hand } from '../target/types/hand';
import type { Lever } from '../target/types/lever';

const HAND_IDL = require('../target/idl/hand.json');
const LEVER_IDL = require('../target/idl/lever.json');
const HAND_PROGRAM_ID = new PublicKey(HAND_IDL.address);
const LEVER_PROGRAM_ID = new PublicKey(LEVER_IDL.address);

describe('cpi', async () => {
  const context = await startAnchor(
    '',
    [
      {
        name: 'hand',
        programId: HAND_PROGRAM_ID,
      },
      {
        name: 'lever',
        programId: LEVER_PROGRAM_ID,
      },
    ],
    [],
  );
  const provider = new BankrunProvider(context);

  const hand = new anchor.Program<Hand>(HAND_IDL, provider);
  const lever = new anchor.Program<Lever>(LEVER_IDL, provider);

  // Generate a new keypair for the power account
  const powerAccount = new anchor.web3.Keypair();

  it('Initialize the lever!', async () => {
    await lever.methods
      .initialize()
      .accounts({
        power: powerAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([powerAccount])
      .rpc();
  });

  it('Pull the lever!', async () => {
    await hand.methods
      .pullLever('Chris')
      .accounts({
        power: powerAccount.publicKey,
      })
      .rpc();
  });

  it('Pull it again!', async () => {
    await hand.methods
      .pullLever('Ashley')
      .accounts({
        power: powerAccount.publicKey,
      })
      .rpc();
  });
});
