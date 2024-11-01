import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { ProgramDerivedAddresses } from '../target/types/program_derived_addresses';

const IDL = require('../target/idl/program_derived_addresses.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun - program_derived_addresses', async () => {
  // Initialize the Bankrun context with the program
  const context = await startAnchor('', [{ name: 'program_derived_addresses', programId: PROGRAM_ID }], []);

  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const program = new anchor.Program<ProgramDerivedAddresses>(IDL, provider);

  const payer = provider.wallet as anchor.Wallet;

  const pageVisitsSeed = 'page_visits';

  const [pageVisitsPDA, pageVisitsBump] = PublicKey.findProgramAddressSync(
    [Buffer.from(pageVisitsSeed), payer.publicKey.toBuffer()],
    program.programId,
  );

  it('Creates a new page visits account', async () => {
    const tx = await program.methods
      .createPageVisits()
      .accounts({
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log('Transaction Signature:', tx);

    const pageVisitsAccount = await program.account.pageVisit.fetch(pageVisitsPDA);

    assert.equal(pageVisitsAccount.pageVisits, 0, 'Page visits should initialize to 0');
    assert.equal(pageVisitsAccount.bump, pageVisitsBump, 'Bump should match the expected PDA bump');
    console.log('Page visits account created with 0 visits');
  });

  it('Increments page visits counter', async () => {
    const tx = await program.methods
      .increment()
      .accounts({
        user: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log('Increment Transaction Signature:', tx);

    const pageVisitsAccount = await program.account.pageVisit.fetch(pageVisitsPDA);

    assert.equal(pageVisitsAccount.pageVisits, 1, 'Page visits should increment to 1');
    console.log('Page visits incremented successfully');
  });
});
