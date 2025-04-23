import { Buffer } from 'node:buffer';
import { PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import { MyInstruction, closeAccountSchema } from '.';

export class Create {
  instruction: MyInstruction;
  name: string;

  constructor(props: { instruction: MyInstruction; name: string }) {
    this.instruction = props.instruction;
    this.name = props.name;
  }

  toBuffer() {
    const textBuffer = Buffer.alloc(64);
    const buffer = Buffer.alloc(1000);

    textBuffer.write('foobarbaz', 0, 'utf-8');

    closeAccountSchema.encode(
      {
        CreateUser: Array.from(textBuffer),
      },
      buffer,
    );

    return buffer.subarray(0, closeAccountSchema.getSpan(buffer));
  }
}

export function createCreateUserInstruction(payer: PublicKey, target: PublicKey, programId: PublicKey, name: string): TransactionInstruction {
  const instructionObject = new Create({
    instruction: MyInstruction.CreateUser,
    name,
  });

  const seed = 'USER';
  const seedBytes = new Uint8Array(seed.split('').map((char) => char.charCodeAt(0)));

  const [pda, _] = PublicKey.findProgramAddressSync([seedBytes, payer.toBuffer()], programId);

  const data = instructionObject.toBuffer();

  const ix = new TransactionInstruction({
    keys: [
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: pda, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data,
  });

  return ix;
}
