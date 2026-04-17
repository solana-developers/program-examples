import { Buffer } from "node:buffer";
import { type PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";

export enum InstructionType {
  CpiTransfer = 0,
  ProgramTransfer = 1,
}

export function createTransferInstruction(
  payerPubkey: PublicKey,
  recipientPubkey: PublicKey,
  programId: PublicKey,
  instruction: InstructionType,
  amount: number,
): TransactionInstruction {
  const data = Buffer.alloc(9);
  data.writeUInt8(instruction, 0);
  data.writeBigUInt64LE(BigInt(amount), 1);

  return new TransactionInstruction({
    keys: [
      { pubkey: payerPubkey, isSigner: true, isWritable: true },
      { pubkey: recipientPubkey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId,
    data,
  });
}
