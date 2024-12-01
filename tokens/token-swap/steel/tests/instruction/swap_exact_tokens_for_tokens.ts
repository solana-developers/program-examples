import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import BN from 'bn.js';
import * as borsh from 'borsh';
import { Assignable, TokenSwapInstruction } from './instruction';

export class SwapExactTokensForTokens extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(SwapExactTokensForTokensSchema, this));
  }
}
const SwapExactTokensForTokensSchema = new Map([
  [
    SwapExactTokensForTokens,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['swap_a', 'u8'],
        ['input_amount', 'u64'],
        ['min_output_amount', 'u64'],
      ],
    },
  ],
]);

export function buildSwapExactTokensForTokensInstruction(props: {
  swap_a: boolean;
  input_amount: BN;
  min_output_amount: BN;
  amm: PublicKey;
  pool: PublicKey;
  poolAuthority: PublicKey;
  trader: PublicKey;
  mintLiquidity: PublicKey;
  mintA: PublicKey;
  mintB: PublicKey;
  poolTokenAccountA: PublicKey;
  poolTokenAccountB: PublicKey;
  traderTokenAccountLiquidity: PublicKey;
  traderTokenAccountA: PublicKey;
  traderTokenAccountB: PublicKey;
  payer: PublicKey;
  programId: PublicKey;
}) {
  const ix = new SwapExactTokensForTokens({
    instruction: TokenSwapInstruction.SwapExactTokensForTokens,
    swap_a: props.swap_a,
    input_amount: props.input_amount,
    min_output_amount: props.min_output_amount,
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
        pubkey: props.trader,
        isSigner: true,
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
        pubkey: props.traderTokenAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.traderTokenAccountB,
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
