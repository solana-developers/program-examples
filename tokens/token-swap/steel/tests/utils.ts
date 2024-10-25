import { PublicKey } from '@solana/web3.js';
import * as borsh from 'borsh';

export const instructionDiscriminators = {
  CreateAmm: Buffer.from([0]),
};

export const getCreateAmmInstructionData = (id: PublicKey, fee: number) => {
  const buffer = Buffer.alloc(2);
  buffer.writeUint16LE(fee, 0);
  return Buffer.concat([instructionDiscriminators.CreateAmm, id.toBuffer(), Buffer.from(buffer)]);
};
