import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { assert } from 'chai';
import { ProgramDerivedAddresses } from '../target/types/program_derived_addresses';

describe('program-derived-addresses', () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Generate new user keypairs for testing
  const user = Keypair.generate();

  const program = anchor.workspace.ProgramDerivedAddresses as Program<ProgramDerivedAddresses>;

  // Variables for storing Program Derived Address (PDA) and bump
  let pageVisitsPDA: PublicKey; // PDA for page visits account
  let pageVisitsBump: number;

  // Define a randomized seed that'll be used for the account
  const seed = new anchor.BN(Math.floor(Math.random() * 10000));

  // Before all tests, airdrop SOL to user for transaction fees and derive the PDA for the page visits account
  before(async () => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    // Airdrop 1 SOL to the first user
    const airdropUser = await provider.connection.requestAirdrop(user.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropUser,
    });

    // Derive PDA for the first user account
    [pageVisitsPDA, pageVisitsBump] = PublicKey.findProgramAddressSync(
      [seed.toArrayLike(Buffer, 'le', 8), user.publicKey.toBuffer()],
      program.programId,
    );
  });

  it('It creates page visits tracking account!', async () => {
    // Invoke the Create Page Visits instruction from the program
    await program.methods
      .createPageVisits(seed)
      .accountsPartial({
        payer: user.publicKey,
        pageVisits: pageVisitsPDA,
      })
      .signers([user])
      .rpc();
  });

  it('Visit the page!', async () => {
    // Invoke the Increment Page Visits instruction from the program
    await program.methods
      .incrementPageVisits()
      .accountsPartial({
        payer: user.publicKey,
        pageVisits: pageVisitsPDA,
      })
      .signers([user])
      .rpc();

    // Fetch the page visits account information
    const pageVisitsInfo = await program.account.pageVisits.fetch(pageVisitsPDA);

    // Assert that the page visits count is 1
    assert.equal(pageVisitsInfo.pageVisits, 1, 'This is supposed to be the first visit, so value should be 1');

    console.log('\nNumber of page visits: ', pageVisitsInfo.pageVisits);
  });

  it('Visit the page, again!', async () => {
    await program.methods
      .incrementPageVisits()
      .accountsPartial({
        payer: user.publicKey,
        pageVisits: pageVisitsPDA,
      })
      .signers([user])
      .rpc();

    // Fetch the page visits account information
    const pageVisitsInfo = await program.account.pageVisits.fetch(pageVisitsPDA);

    // Assert that the page visits count is 2
    assert.equal(pageVisitsInfo.pageVisits, 2, 'This is supposed to be the second visit, so value should be 2');

    console.log('\nNumber of page visits: ', pageVisitsInfo.pageVisits);
  });
});
