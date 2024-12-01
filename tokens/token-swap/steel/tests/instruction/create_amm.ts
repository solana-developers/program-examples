import { PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { Assignable, TokenSwapInstruction } from './instruction';

class CreateAmm extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(CreateAmmSchema, this));
  }
}

const CreateAmmSchema = new Map([
  [
    CreateAmm,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['id', [32]],
        ['fee', 'u16'],
      ],
    },
  ],
]);

export function buildCreateAmmInstruction(props: {
  id: PublicKey;
  fee: number;
  amm: PublicKey;
  admin: PublicKey;
  payer: PublicKey;
  programId: PublicKey;
}) {
  const ix = new CreateAmm({
    instruction: TokenSwapInstruction.CreateAmm,
    id: props.id.toBytes(),
    fee: props.fee,
  });

  return new TransactionInstruction({
    keys: [
      {
        pubkey: props.amm,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.admin,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.payer,
        isSigner: true,
        isWritable: true,
      },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: props.programId,
    data: ix.toBuffer(),
  });
}
