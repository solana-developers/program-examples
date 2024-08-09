import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey, Transaction } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { AnchorProgramExample } from '../target/types/anchor_program_example';

const IDL = require('../target/idl/anchor_program_example.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('PDAs', async () => {
  const context = await startAnchor('', [{ name: 'anchor_program_example', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const client = context.banksClient;

  const payer = provider.wallet as anchor.Wallet;
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
