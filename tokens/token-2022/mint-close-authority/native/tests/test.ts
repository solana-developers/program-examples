import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { TOKEN_2022_PROGRAM_ID } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction, LAMPORTS_PER_SOL} from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { LiteSVM } from 'litesvm';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

class CreateTokenArgs extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(CreateTokenArgsSchema, this));
  }
}
const CreateTokenArgsSchema = new Map([
  [
    CreateTokenArgs,
    {
      kind: 'struct',
      fields: [['token_decimals', 'u8']],
    },
  ],
]);

describe('Create Token', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/token_2022_mint_close_authority_program.so');
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  test('Create a Token-22 SPL-Token !', async () => {
    const mintKeypair: Keypair = Keypair.generate();

    const instructionData = new CreateTokenArgs({
      token_decimals: 9,
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true }, // Mint account
        { pubkey: payer.publicKey, isSigner: false, isWritable: true }, // Mint authority account
        { pubkey: payer.publicKey, isSigner: false, isWritable: true }, // Mint close authority account
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Transaction Payer
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // Rent account
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
        { pubkey: TOKEN_2022_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
      ],
      programId: PROGRAM_ID,
      data: instructionData.toBuffer(),
    });
    const blockhash = svm.latestBlockhash();

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, mintKeypair);

    const transaction = svm.sendTransaction(tx);

    assert(transaction.logs()[0].startsWith(`Program ${PROGRAM_ID}`));
    console.log('Token Mint Address: ', mintKeypair.publicKey.toBase58());
  });
});
