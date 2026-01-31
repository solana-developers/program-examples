import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PublicKey, Transaction, TransactionInstruction, LAMPORTS_PER_SOL, Keypair} from '@solana/web3.js';
import * as borsh from 'borsh';
import { LiteSVM } from 'litesvm';

describe('custom-instruction-data', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/processing_instructions_program.so');
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  class Assignable {
    constructor(properties) {
      for (const [key, value] of Object.entries(properties)) {
        this[key] = value;
      }
    }
  }

  class InstructionData extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(InstructionDataSchema, this));
    }
  }

  const InstructionDataSchema = new Map([
    [
      InstructionData,
      {
        kind: 'struct',
        fields: [
          ['name', 'string'],
          ['height', 'u32'],
        ],
      },
    ],
  ]);

  test('Go to the park!', async () => {
    const blockhash = svm.latestBlockhash();

    const jimmy = new InstructionData({
      name: 'Jimmy',
      height: 3,
    });

    const mary = new InstructionData({
      name: 'Mary',
      height: 10,
    });

    const ix1 = new TransactionInstruction({
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
      programId: PROGRAM_ID,
      data: jimmy.toBuffer(),
    });

    const ix2 = new TransactionInstruction({
      ...ix1,
      data: mary.toBuffer(),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix1).add(ix2).sign(payer);

    svm.sendTransaction(tx);
  });
});
