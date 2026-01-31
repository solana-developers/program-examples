import { describe, it } from 'node:test';
import { PublicKey, Transaction, TransactionInstruction, LAMPORTS_PER_SOL, Keypair } from '@solana/web3.js';
import { assert } from 'chai';
import { LiteSVM } from 'litesvm';

describe('hello-solana', () => {
  const PROGRAM_ID = PublicKey.unique();

  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/hello_solana_program_pinocchio.so');
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  it('Say hello!', () => {
    const blockhash = svm.latestBlockhash();
    // We set up our instruction first.
    const ix = new TransactionInstruction({
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
      programId: PROGRAM_ID,
      data: Buffer.from([]), // No data
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    // Now we process the transaction
    const transaction = svm.sendTransaction(tx);

    assert(transaction.logs()[0].startsWith(`Program ${PROGRAM_ID}`));
    assert(transaction.logs()[1] === 'Program log: Hello, Solana!');
    assert(transaction.logs()[2] === `Program log: [${Array.from(PROGRAM_ID.toBytes()).join(', ')}]`);
    assert(transaction.logs()[3].startsWith(`Program ${PROGRAM_ID} consumed`));
    assert(transaction.logs()[4] === `Program ${PROGRAM_ID} success`);
    assert(transaction.logs().length === 5);
  });
});
