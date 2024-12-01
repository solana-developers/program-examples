import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { Assignable, TokenSwapInstruction } from './instruction';

export class CreatePool extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(CreatePoolSchema, this));
  }
}
const CreatePoolSchema = new Map([
  [
    CreatePool,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        // ["mint_decimals", "u8"],
      ],
    },
  ],
]);

export function buildCreatePoolInstruction(props: {
  amm: PublicKey;
  pool: PublicKey;
  poolAuthority: PublicKey;
  mintLiquidity: PublicKey;
  mintA: PublicKey;
  mintB: PublicKey;
  poolTokenAccountA: PublicKey;
  poolTokenAccountB: PublicKey;
  payer: PublicKey;
  programId: PublicKey;
}) {
  const ix = new CreatePool({
    instruction: TokenSwapInstruction.CreatePool,
  });

  return new TransactionInstruction({
    keys: [
      {
        pubkey: props.amm,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.pool,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.poolAuthority,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.mintLiquidity,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.mintA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.mintB,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.poolTokenAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.poolTokenAccountB,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.payer,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      {
        pubkey: SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: props.programId,
    data: ix.toBuffer(),
  });
}
