import { Buffer } from 'node:buffer';
import { type PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { MyInstruction, type MyInstructionType } from '.';

export class Create {
  instruction: MyInstructionType;
  name: string;

  constructor(props: { instruction: MyInstructionType; name: string }) {
    this.instruction = props.instruction;
    this.name = props.name;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(CreateSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(CreateSchema, Create, buffer);
  }
}

export const CreateSchema = new Map([
  [
    Create,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['name', 'string'],
      ],
    },
  ],
]);

export function createCreateUserInstruction(target: PublicKey, payer: PublicKey, programId: PublicKey, name: string): TransactionInstruction {
  const instructionObject = new Create({
    instruction: MyInstruction.CreateUser,
    name,
  });

  const ix = new TransactionInstruction({
    keys: [
      { pubkey: target, isSigner: false, isWritable: true },
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data: instructionObject.toBuffer(),
  });

  return ix;
}
