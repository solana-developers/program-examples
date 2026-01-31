import { after, describe, test } from 'node:test';
import { PublicKey, Transaction, TransactionInstruction, LAMPORTS_PER_SOL, Keypair} from '@solana/web3.js';
import { assert } from 'chai';
import { LiteSVM, TransactionMetadata, FailedTransactionMetadata } from 'litesvm';

describe('hello-solana', async () => {
  // load program in litesvm
  const PROGRAM_ID = PublicKey.unique();
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/hello_solana_program.so');

  after(() => {
    process.exit(0);
  });
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  test('Say hello!', async () => {
    const blockhash = svm.latestBlockhash();
    // We set up our instruction first.
    const ix = new TransactionInstruction({
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
      programId: PROGRAM_ID,
      data: Buffer.alloc(0), // No data
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    // Now we process the transaction
    const txResult = svm.sendTransaction(tx);

    // sendTransaction returns TransactionMetadata | FailedTransactionMetadata
    // On success it's TransactionMetadata which has logs() directly
    const meta = txResult instanceof TransactionMetadata ? txResult : (txResult as FailedTransactionMetadata).meta();
    const logs = meta.logs();
    assert(logs[0].startsWith(`Program ${PROGRAM_ID}`));
    assert(logs[1] === 'Program log: Hello, Solana!');
    assert(logs[2] === `Program log: Our program's Program ID: ${PROGRAM_ID}`);
    assert(logs[3].startsWith(`Program ${PROGRAM_ID} consumed`));
    assert(logs[4] === `Program ${PROGRAM_ID} success`);
    assert(logs.length === 5);
  });
});
