import { Buffer } from 'node:buffer';
import { type PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { ReallocInstruction } from './instruction';

export class Create {
  instruction: ReallocInstruction;
  name: string;
  house_number: number;
  street: string;
  city: string;

  constructor(props: {
    instruction: ReallocInstruction;
    name: string;
    house_number: number;
    street: string;
    city: string;
  }) {
    this.instruction = props.instruction;
    this.name = props.name;
    this.house_number = props.house_number;
    this.street = props.street;
    this.city = props.city;
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
        ['house_number', 'u8'],
        ['street', 'string'],
        ['city', 'string'],
      ],
    },
  ],
]);

export function createCreateInstruction(
  target: PublicKey,
  payer: PublicKey,
  programId: PublicKey,
  name: string,
  house_number: number,
  street: string,
  city: string,
): TransactionInstruction {
  const instructionObject = new Create({
    instruction: ReallocInstruction.Create,
    name,
    house_number,
    street,
    city,
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
