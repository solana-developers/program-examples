import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { type PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";
import type BN from "bn.js";
import * as borsh from "borsh";

enum FundraiserInstruction {
  Initialize = 0,
  Contribute = 1,
  CheckContributions = 2,
  Refund = 3,
}

// The Pinocchio program receives PDA bumps in the instruction data (and stores
// the fundraiser bump) instead of deriving them on-chain.
const InitializeSchema = {
  struct: {
    instruction: "u8",
    amount: "u64",
    duration: "u16",
    bump: "u8",
  },
};

const ContributeSchema = {
  struct: {
    instruction: "u8",
    amount: "u64",
    contributor_bump: "u8",
  },
};

const CheckContributionsSchema = {
  struct: {
    instruction: "u8",
  },
};

const RefundSchema = {
  struct: {
    instruction: "u8",
    contributor_bump: "u8",
  },
};

function borshSerialize(schema: borsh.Schema, data: object): Buffer {
  return Buffer.from(borsh.serialize(schema, data));
}

export function buildInitialize(props: {
  amount: BN;
  duration: number;
  bump: number;
  maker: PublicKey;
  mint: PublicKey;
  fundraiser: PublicKey;
  vault: PublicKey;
  programId: PublicKey;
}) {
  const data = borshSerialize(InitializeSchema, {
    instruction: FundraiserInstruction.Initialize,
    amount: props.amount,
    duration: props.duration,
    bump: props.bump,
  });

  return new TransactionInstruction({
    keys: [
      { pubkey: props.maker, isSigner: true, isWritable: true },
      { pubkey: props.mint, isSigner: false, isWritable: false },
      { pubkey: props.fundraiser, isSigner: false, isWritable: true },
      { pubkey: props.vault, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: props.programId,
    data,
  });
}

export function buildContribute(props: {
  amount: BN;
  contributor_bump: number;
  contributor: PublicKey;
  mint: PublicKey;
  fundraiser: PublicKey;
  contributorAccount: PublicKey;
  contributorAta: PublicKey;
  vault: PublicKey;
  programId: PublicKey;
}) {
  const data = borshSerialize(ContributeSchema, {
    instruction: FundraiserInstruction.Contribute,
    amount: props.amount,
    contributor_bump: props.contributor_bump,
  });

  return new TransactionInstruction({
    keys: [
      { pubkey: props.contributor, isSigner: true, isWritable: true },
      { pubkey: props.mint, isSigner: false, isWritable: false },
      { pubkey: props.fundraiser, isSigner: false, isWritable: true },
      { pubkey: props.contributorAccount, isSigner: false, isWritable: true },
      { pubkey: props.contributorAta, isSigner: false, isWritable: true },
      { pubkey: props.vault, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: props.programId,
    data,
  });
}

export function buildCheckContributions(props: {
  maker: PublicKey;
  mint: PublicKey;
  fundraiser: PublicKey;
  vault: PublicKey;
  makerAta: PublicKey;
  programId: PublicKey;
}) {
  const data = borshSerialize(CheckContributionsSchema, {
    instruction: FundraiserInstruction.CheckContributions,
  });

  return new TransactionInstruction({
    keys: [
      { pubkey: props.maker, isSigner: true, isWritable: true },
      { pubkey: props.mint, isSigner: false, isWritable: false },
      { pubkey: props.fundraiser, isSigner: false, isWritable: true },
      { pubkey: props.vault, isSigner: false, isWritable: true },
      { pubkey: props.makerAta, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: props.programId,
    data,
  });
}

export function buildRefund(props: {
  contributor_bump: number;
  contributor: PublicKey;
  maker: PublicKey;
  mint: PublicKey;
  fundraiser: PublicKey;
  contributorAccount: PublicKey;
  contributorAta: PublicKey;
  vault: PublicKey;
  programId: PublicKey;
}) {
  const data = borshSerialize(RefundSchema, {
    instruction: FundraiserInstruction.Refund,
    contributor_bump: props.contributor_bump,
  });

  return new TransactionInstruction({
    keys: [
      { pubkey: props.contributor, isSigner: true, isWritable: true },
      { pubkey: props.maker, isSigner: false, isWritable: false },
      { pubkey: props.mint, isSigner: false, isWritable: false },
      { pubkey: props.fundraiser, isSigner: false, isWritable: true },
      { pubkey: props.contributorAccount, isSigner: false, isWritable: true },
      { pubkey: props.contributorAta, isSigner: false, isWritable: true },
      { pubkey: props.vault, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: props.programId,
    data,
  });
}
