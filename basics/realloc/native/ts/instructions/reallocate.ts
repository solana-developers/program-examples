import { Buffer } from 'node:buffer';
import { type PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { ReallocInstruction } from './instruction';

export class ReallocateWithoutZeroInit {
  instruction: ReallocInstruction;
  state: string;
  zip: number;

  constructor(props: {
    instruction: ReallocInstruction;
    state: string;
    zip: number;
  }) {
    this.instruction = props.instruction;
    this.state = props.state;
    this.zip = props.zip;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(ReallocateWithoutZeroInitSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(ReallocateWithoutZeroInitSchema, ReallocateWithoutZeroInit, buffer);
  }
}

export const ReallocateWithoutZeroInitSchema = new Map([
  [
    ReallocateWithoutZeroInit,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['state', 'string'],
        ['zip', 'u32'],
      ],
    },
  ],
]);

export function createReallocateWithoutZeroInitInstruction(
  target: PublicKey,
  payer: PublicKey,
  programId: PublicKey,
  state: string,
  zip: number,
): TransactionInstruction {
  const instructionObject = new ReallocateWithoutZeroInit({
    instruction: ReallocInstruction.ReallocateWithoutZeroInit,
    state,
    zip,
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

export class ReallocateZeroInit {
  instruction: ReallocInstruction;
  name: string;
  position: string;
  company: string;
  years_employed: number;

  constructor(props: {
    instruction: ReallocInstruction;
    name: string;
    position: string;
    company: string;
    years_employed: number;
  }) {
    this.instruction = props.instruction;
    this.name = props.name;
    this.position = props.position;
    this.company = props.company;
    this.years_employed = props.years_employed;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(ReallocateZeroInitSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(ReallocateZeroInitSchema, ReallocateZeroInit, buffer);
  }
}

export const ReallocateZeroInitSchema = new Map([
  [
    ReallocateZeroInit,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['name', 'string'],
        ['position', 'string'],
        ['company', 'string'],
        ['years_employed', 'u8'],
      ],
    },
  ],
]);

export function createReallocateZeroInitInstruction(
  target: PublicKey,
  payer: PublicKey,
  programId: PublicKey,
  name: string,
  position: string,
  company: string,
  years_employed: number,
): TransactionInstruction {
  const instructionObject = new ReallocateZeroInit({
    instruction: ReallocInstruction.ReallocateZeroInit,
    name,
    position,
    company,
    years_employed,
  });

  const ix = new TransactionInstruction({
    keys: [{ pubkey: target, isSigner: false, isWritable: true }],
    programId: programId,
    data: instructionObject.toBuffer(),
  });

  return ix;
}
