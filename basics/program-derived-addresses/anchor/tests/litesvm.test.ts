import { Program } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { AnchorProgramExample } from '../target/types/anchor_program_example';

const IDL = require('../target/idl/anchor_program_example.json');

describe('PDAs', async () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<AnchorProgramExample>;
  let payer: Keypair;
  let pageVisitPDA: PublicKey;

  before(async () => {
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new anchor.Program<AnchorProgramExample>(IDL, provider);

    // PDA for the page visits account
    [pageVisitPDA] = PublicKey.findProgramAddressSync([Buffer.from('page_visits'), payer.publicKey.toBuffer()], program.programId);
  });

  it('Create the page visits tracking PDA', async () => {
    await program.methods
      .createPageVisits()
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();
  });

  it('Visit the page!', async () => {
    await program.methods
      .incrementPageVisits()
      .accounts({
        user: payer.publicKey,
      })
      .rpc();
  });

  it('View page visits', async () => {
    const pageVisits = await program.account.pageVisits.fetch(pageVisitPDA);
    console.log(`Number of page visits: ${pageVisits.pageVisits}`);
  });
});
