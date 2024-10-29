import { Buffer } from 'node:buffer';
import { type PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { MyInstruction } from '.';

export class Create {
  instruction: MyInstruction;
  name: Uint8Array;

  constructor(props: { instruction: MyInstruction; name: Uint8Array }) {
    this.instruction = props.instruction;
    this.name = props.name;
  }

  static from(props: { instruction: MyInstruction; name: string }) {
    return new Create({
      instruction: props.instruction,
      name: Buffer.from(props.name.padEnd(48, '\0')),
    });
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
        ['name', [48]],
      ],
    },
  ],
]);

export function createCreateUserInstruction(target: PublicKey, payer: PublicKey, programId: PublicKey, name: string): TransactionInstruction {
  const instructionObject = Create.from({
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
