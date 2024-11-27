import { Buffer } from 'node:buffer';
import { type PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import { closeAccountSchema, MyInstruction } from '.';

export class Close {
  instruction: MyInstruction;

  constructor(props: { instruction: MyInstruction }) {
    this.instruction = props.instruction;
  }

  toBuffer() {
    const buffer = Buffer.alloc(1000);

    closeAccountSchema.encode(
      {
        CloseUser: '',
      },
      buffer,
    );

    return buffer.subarray(0, closeAccountSchema.getSpan(buffer));
  }
}

export function createCloseUserInstruction(payer: PublicKey, target: PublicKey, programId: PublicKey): TransactionInstruction {
  const instructionObject = new Close({
    instruction: MyInstruction.CloseUser,
  });

  const ix = new TransactionInstruction({
    keys: [
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: target, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data: instructionObject.toBuffer(),
  });

  return ix;
}
