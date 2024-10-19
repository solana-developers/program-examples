import { describe, test } from 'node:test';
import { Connection, Keypair, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';

describe('favorites', async () => {
  const payer = Keypair.generate();
  const programId = PublicKey.unique();
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  test('should initialize correctly', async () => {
    const result = await start([{ name: 'favorites_program', programId }], []);
    assert.isNotNull(result, 'Initialization failed');
  });

  test('should create a new favorite', async () => {
    const result = await start([{ name: 'favorites_program', programId }], []);
    const client = result.banksClient;
    const favorite = Keypair.generate();
    const tx = new Transaction().add(
      new TransactionInstruction({
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: favorite.publicKey, isSigner: true, isWritable: true },
        ],
        programId,
        data: Buffer.alloc(0),
      }),
    );
    tx.recentBlockhash = (await connection.getRecentBlockhash()).blockhash;
    tx.sign(payer, favorite);
    await client.processTransaction(tx);
  });

  test('should delete a favorite', async () => {
    const result = await start([{ name: 'favorites_program', programId }], []);
    const client = result.banksClient;
    const favorite = Keypair.generate();
    const tx = new Transaction().add(
      new TransactionInstruction({
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: favorite.publicKey, isSigner: true, isWritable: true },
        ],
        programId,
        data: Buffer.alloc(0),
      }),
    );
    tx.recentBlockhash = (await connection.getRecentBlockhash()).blockhash;
    tx.sign(payer, favorite);
    await client.processTransaction(tx);
  });
});
