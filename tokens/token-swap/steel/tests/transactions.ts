import { BN } from '@coral-xyz/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY as SYS_VAR_RENT_ID, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { serialize } from 'borsh';
import { ProgramTestContext } from 'solana-bankrun';
import { PROGRAM_ID, TestValues } from './utils';

export const createAmmTransactionInstruction = (values: TestValues, payer: Keypair, context: ProgramTestContext, shouldFail = false): Transaction => {
  const data = serialize(
    {
      struct: {
        discriminator: 'u8',
        id: { array: { type: 'u8', len: 32 } },
        fee: 'u16',
      },
    },
    {
      discriminator: 0,
      id: values.id.toBytes(),
      fee: shouldFail ? 2000 : values.fee,
    },
  );

  const ix = new TransactionInstruction({
    data: Buffer.from(data),
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      { pubkey: values.ammKey, isSigner: false, isWritable: true },
      {
        pubkey: values.admin.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: PROGRAM_ID,
  });

  const blockhash = context.lastBlockhash;
  const tx = new Transaction();
  tx.recentBlockhash = blockhash;
  tx.add(ix);
  tx.sign(payer);
  return tx;
};

export const createPoolInstruction = (values: TestValues, payer: Keypair, context: ProgramTestContext, shouldFail = false): Transaction => {
  const data = serialize(
    {
      struct: {
        discriminator: 'u8',
      },
    },
    {
      discriminator: 1,
    },
  );

  const ix = new TransactionInstruction({
    data: Buffer.from(data),
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      { pubkey: values.ammKey, isSigner: false, isWritable: false },
      {
        pubkey: values.poolKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAuthority,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: values.mintLiquidity,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.mintAKeypair.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: shouldFail ? values.mintAKeypair.publicKey : values.mintBKeypair.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAccountB,
        isSigner: false,
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
      {
        pubkey: SYS_VAR_RENT_ID,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: PROGRAM_ID,
  });

  const blockhash = context.lastBlockhash;
  const tx = new Transaction();
  tx.recentBlockhash = blockhash;
  tx.add(ix);
  tx.sign(payer);
  return tx;
};

export const createDepositInstruction = (
  values: TestValues,
  payer: Keypair,
  context: ProgramTestContext,
  sameAmount = false,
  shouldFail = false,
): Transaction => {
  const data = serialize(
    {
      struct: {
        discriminator: 'u8',
        amount_a: 'u64',
        amount_b: 'u64',
      },
    },
    {
      discriminator: 2,
      amount_a: values.depositAmountA,
      amount_b: sameAmount ? values.depositAmountA : values.depositAmountB,
    },
  );

  const ix = new TransactionInstruction({
    data: Buffer.from(data),
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      { pubkey: values.admin.publicKey, isSigner: !false, isWritable: true },
      {
        pubkey: values.poolKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: values.poolAuthority,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: values.mintLiquidity,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.mintAKeypair.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.mintBKeypair.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAccountB,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.liquidityAccount,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.holderAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.holderAccountB,
        isSigner: false,
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
      // {
      //   pubkey: SYS_VAR_RENT_ID,
      //   isSigner: false,
      //   isWritable: false,
      // },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: PROGRAM_ID,
  });

  const blockhash = context.lastBlockhash;
  const tx = new Transaction();
  tx.recentBlockhash = blockhash;
  tx.add(ix);
  tx.sign(payer, values.admin);
  return tx;
};

export const createSwapInstruction = (
  values: TestValues,
  payer: Keypair,
  context: ProgramTestContext,
  input: BN = new BN(10 ** 6),
  output: BN = new BN(100),
  shouldFail = false,
): Transaction => {
  const data = serialize(
    {
      struct: {
        discriminator: 'u8',
        swap_a: 'u8',
        input: 'u64',
        min_output_amount: 'u64',
      },
    },
    {
      discriminator: 3,
      swap_a: 1,
      input: input,
      min_output_amount: output,
    },
  );

  const ix = new TransactionInstruction({
    data: Buffer.from(data),
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      { pubkey: values.admin.publicKey, isSigner: true, isWritable: true },
      {
        pubkey: values.ammKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: values.poolKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: values.poolAuthority,
        isSigner: false,
        isWritable: false,
      },

      {
        pubkey: values.mintAKeypair.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.mintBKeypair.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAccountB,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.holderAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.holderAccountB,
        isSigner: false,
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
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: PROGRAM_ID,
  });

  const blockhash = context.lastBlockhash;
  const tx = new Transaction();
  tx.recentBlockhash = blockhash;
  tx.add(ix);
  tx.sign(payer, values.admin);
  return tx;
};

export const createWithdrawInstruction = (
  values: TestValues,
  payer: Keypair,
  context: ProgramTestContext,
  amount: BN = values.depositAmountA.sub(values.minimumLiquidity),
  shouldFail = false,
): Transaction => {
  const data = serialize(
    {
      struct: {
        discriminator: 'u8',
        amount: 'u64',
      },
    },
    {
      discriminator: 4,
      amount,
    },
  );

  const ix = new TransactionInstruction({
    data: Buffer.from(data),
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      { pubkey: values.admin.publicKey, isSigner: !false, isWritable: true },
      {
        pubkey: values.ammKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: values.poolAuthority,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: values.mintLiquidity,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.mintAKeypair.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.mintBKeypair.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.poolAccountB,
        isSigner: false,
        isWritable: true,
      },

      {
        pubkey: values.liquidityAccount,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.holderAccountA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: values.holderAccountB,
        isSigner: false,
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
      // {
      //   pubkey: SYS_VAR_RENT_ID,
      //   isSigner: false,
      //   isWritable: false,
      // },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: PROGRAM_ID,
  });

  const blockhash = context.lastBlockhash;
  const tx = new Transaction();
  tx.recentBlockhash = blockhash;
  tx.add(ix);
  tx.sign(payer, values.admin);
  return tx;
};
