import { Program } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { AnchorProgramExample } from '../target/types/anchor_program_example';

const IDL = require('../target/idl/anchor_program_example.json');

describe('PDAs', async () => {
  const client = fromWorkspace('');
  const provider = new LiteSVMProvider(client);
  const payer = provider.wallet.payer;
  const program = new anchor.Program<AnchorProgramExample>(IDL, provider);

  // PDA for the page visits account
  const [pageVisitPDA] = PublicKey.findProgramAddressSync([Buffer.from('page_visits'), payer.publicKey.toBuffer()], program.programId);

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

  it('Visit the page!', async () => {
    await client.expireBlockhash();
    const recentBlockhash = await client.latestBlockhash();
    const tx = await program.methods.incrementPageVisits().accounts({ user: payer.publicKey }).transaction();
    tx.recentBlockhash = recentBlockhash;
    tx.feePayer = payer.publicKey;
    await provider.sendAndConfirm(tx, [payer]);
  });

  it('View page visits', async () => {
    const pageVisits = await program.account.pageVisits.fetch(pageVisitPDA);
    console.log(`Number of page visits: ${pageVisits.pageVisits}`);
  });
});
