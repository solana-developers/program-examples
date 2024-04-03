import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { Journal } from '../target/types/journal';

describe('journal', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.Journal as Program<Journal>;

  const journalKeypair = Keypair.generate();

  it('Initialize Journal', async () => {
    await program.methods
      .initialize()
      .accounts({
        journal: journalKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([journalKeypair])
      .rpc();

    const currentCount = await program.account.journal.fetch(
      journalKeypair.publicKey
    );

    expect(currentCount.count).toEqual(0);
  });

  it('Increment Journal', async () => {
    await program.methods
      .increment()
      .accounts({ journal: journalKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.journal.fetch(
      journalKeypair.publicKey
    );

    expect(currentCount.count).toEqual(1);
  });

  it('Increment Journal Again', async () => {
    await program.methods
      .increment()
      .accounts({ journal: journalKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.journal.fetch(
      journalKeypair.publicKey
    );

    expect(currentCount.count).toEqual(2);
  });

  it('Decrement Journal', async () => {
    await program.methods
      .decrement()
      .accounts({ journal: journalKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.journal.fetch(
      journalKeypair.publicKey
    );

    expect(currentCount.count).toEqual(1);
  });

  it('Set journal value', async () => {
    await program.methods
      .set(42)
      .accounts({ journal: journalKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.journal.fetch(
      journalKeypair.publicKey
    );

    expect(currentCount.count).toEqual(42);
  });

  it('Set close the journal account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        journal: journalKeypair.publicKey,
      })
      .rpc();

    // The account should no longer exist, returning null.
    const userAccount = await program.account.journal.fetchNullable(
      journalKeypair.publicKey
    );
    expect(userAccount).toBeNull();
  });
});
