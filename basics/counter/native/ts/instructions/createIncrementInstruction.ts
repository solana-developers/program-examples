import { type PublicKey, TransactionInstruction } from '@solana/web3.js';
import { PROGRAM_ID as DEFAULT_PROGRAM_ID } from '../index.ts';

export type IncrementInstructionAccounts = {
  counter: PublicKey;
};

export function createIncrementInstruction(accounts: IncrementInstructionAccounts, programId: PublicKey = DEFAULT_PROGRAM_ID): TransactionInstruction {
  return new TransactionInstruction({
    programId: programId,
    keys: [
      {
        pubkey: accounts.counter,
        isSigner: false,
        isWritable: true,
      },
    ],
    data: Buffer.from([0x0]),
  });
}
