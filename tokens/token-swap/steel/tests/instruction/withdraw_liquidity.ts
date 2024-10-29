import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import BN from 'bn.js';
import * as borsh from 'borsh';
import { Assignable, TokenSwapInstruction } from './instruction';

export class WithdrawLiquidity extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(WithdrawLiquiditySchema, this));
  }
}
const WithdrawLiquiditySchema = new Map([
  [
    WithdrawLiquidity,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u64'],
      ],
    },
  ],
]);

export function buildWithdrawLiquidityInstruction(props: {
  amount: BN;
  amm: PublicKey;
  pool: PublicKey;
  poolAuthority: PublicKey;
  depositor: PublicKey;
  mintLiquidity: PublicKey;
  mintA: PublicKey;
  mintB: PublicKey;
  poolTokenAccountA: PublicKey;
  poolTokenAccountB: PublicKey;
  depositorTokenAccountLiquidity: PublicKey;
  depositorTokenAccountA: PublicKey;
  depositorTokenAccountB: PublicKey;
  payer: PublicKey;
  programId: PublicKey;
}) {
  const ix = new WithdrawLiquidity({
    instruction: TokenSwapInstruction.WithdrawLiquidity,
    amount: props.amount,
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
        pubkey: props.depositor,
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
        pubkey: props.depositorTokenAccountLiquidity,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.depositorTokenAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: props.depositorTokenAccountB,
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
