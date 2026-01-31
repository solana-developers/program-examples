import { Buffer } from 'node:buffer';
import { after, describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction, LAMPORTS_PER_SOL} from '@solana/web3.js';
import * as borsh from 'borsh';
import { LiteSVM } from 'litesvm';

describe('Create a system account', async () => {
  const PROGRAM_ID = PublicKey.unique();

  after(() => {
    process.exit(0);
  });
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/program.so');
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  class Assignable {
    constructor(properties) {
      for (const [key, value] of Object.entries(properties)) {
        this[key] = value;
      }
    }
  }

  class AddressData extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(AddressDataSchema, this));
    }
  }

  const AddressDataSchema = new Map([
    [
      AddressData,
      {
        kind: 'struct',
        fields: [
          ['name', 'string'],
          ['address', 'string'],
        ],
      },
    ],
  ]);

  test('Create the account', async () => {
    const newKeypair = Keypair.generate();

    const addressData = new AddressData({
      name: 'Marcus',
      address: '123 Main St. San Francisco, CA',
    });

    // We're just going to serialize our object here so we can check
    // the size on the client side against the program logs
    const addressDataBuffer = addressData.toBuffer();
    console.log(`Address data buffer length: ${addressDataBuffer.length}`);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: newKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: addressDataBuffer,
    });

    const tx = new Transaction();
    const blockhash = svm.latestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, newKeypair);

    svm.sendTransaction(tx);
  });
});
