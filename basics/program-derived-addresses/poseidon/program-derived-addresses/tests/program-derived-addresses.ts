import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import { ProgramDerivedAddresses } from '../target/types/program_derived_addresses';

describe('program_derived_addresses', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ProgramDerivedAddresses as Program<ProgramDerivedAddresses>;
  const payer = provider.wallet as anchor.Wallet;

  // Constants for seeds and page visit account derivation
  const pageVisitsSeed = 'page_visits';
  let pageVisitsPDA: PublicKey;
  let pageVisitsBump: number;

  before(async () => {
    // Derive the PDA and bump for the page visits account using seeds
    [pageVisitsPDA, pageVisitsBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from(pageVisitsSeed), payer.publicKey.toBuffer()],
      program.programId,
    );
  });

  it('Creates a new page visits account', async () => {
    const tx = await program.methods
      .createPageVisits()
      .accounts({
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log('Transaction Signature:', tx);

    // Fetch the newly created account to check initial values
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

    // Fetch the account again to check the incremented value
    const pageVisitsAccount = await program.account.pageVisit.fetch(pageVisitsPDA);

    assert.equal(pageVisitsAccount.pageVisits, 1, 'Page visits should increment to 1');
    console.log('Page visits incremented successfully');
  });
});
