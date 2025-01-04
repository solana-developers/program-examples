import { Buffer } from 'node:buffer';
import { type PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';

export enum InstructionType {
  CpiTransfer = 0,
  ProgramTransfer = 1,
}

export class TransferInstruction {
  instruction: InstructionType;
  amount: number;

  constructor(props: { instruction: InstructionType; amount: number }) {
    this.instruction = props.instruction;
    this.amount = props.amount;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(TransferInstructionSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(TransferInstructionSchema, TransferInstruction, buffer);
  }
}

export const TransferInstructionSchema = new Map([
  [
    TransferInstruction,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u64'],
      ],
    },
  ],
]);

export function createTransferInstruction(
  payerPubkey: PublicKey,
  recipientPubkey: PublicKey,
  programId: PublicKey,
  instruction: InstructionType,
  amount: number,
  systemProgramId?: boolean,
): TransactionInstruction {
  const instructionObject = new TransferInstruction({
    instruction,
    amount,
  });

  const keys = [
    { pubkey: payerPubkey, isSigner: true, isWritable: true },
    { pubkey: recipientPubkey, isSigner: false, isWritable: true },
  ];

  if (systemProgramId) {
    keys.push({
      pubkey: SystemProgram.programId,
      isSigner: false,
      isWritable: false,
    });
  }

  const ix = new TransactionInstruction({
    keys: keys,
    programId,
    data: instructionObject.toBuffer(),
  });

  return ix;
}
