import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { Hand } from '../target/types/hand';
import { Lever } from '../target/types/lever';

const HAND_IDL = require('../target/idl/hand.json');
const LEVER_IDL = require('../target/idl/lever.json');

describe('anchor', () => {
  let client: any;
  let provider: LiteSVMProvider;
  let hand: Program<Hand>;
  let lever: Program<Lever>;
  let payer: Keypair;
  let accountToChange: Keypair;
  let accountToCreate: Keypair;

  before(async () => {
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    hand = new anchor.Program<Hand>(HAND_IDL, provider);
    lever = new anchor.Program<Lever>(LEVER_IDL, provider);

    // We'll create this ahead of time.
    // Our program will try to modify it.
    accountToChange = new Keypair();
    // Our program will create this.
    accountToCreate = new Keypair();
  });

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
