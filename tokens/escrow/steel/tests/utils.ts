import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
} from '@solana/spl-token';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { ProgramTestContext } from 'solana-bankrun';

export const instructionDiscriminators = {
  MakeOffer: Buffer.from([0]),
  TakeOffer: Buffer.from([1]),
};

export const getMakeOfferInstructionData = (id: bigint, token_a_offered_amount: bigint, token_b_wanted_amount: bigint) => {
  return Buffer.concat([
    instructionDiscriminators.MakeOffer,
    encodeBigint(id),
    encodeBigint(token_a_offered_amount),
    encodeBigint(token_b_wanted_amount),
  ]);
};

export const getTakeOfferInstructionData = () => {
  return Buffer.concat([instructionDiscriminators.TakeOffer]);
};

export const createAMint = async (context: ProgramTestContext, payer: Keypair, mint: Keypair) => {
  const tx = new Transaction();
  tx.add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint.publicKey,
      // the `space` required for a token mint is accessible in the `@solana/spl-token` sdk
      space: MINT_SIZE,
      // store enough lamports needed for our `space` to be rent exempt
      lamports: Number((await context.banksClient.getRent()).minimumBalance(BigInt(MINT_SIZE))),
      // tokens are owned by the "token program"
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMint2Instruction(mint.publicKey, 9, payer.publicKey, payer.publicKey),
  );
  tx.recentBlockhash = context.lastBlockhash;
  tx.sign(payer, mint);

  // process the transaction
  await context.banksClient.processTransaction(tx);
};

export const mintTo = async (context: ProgramTestContext, payer: Keypair, owner: PublicKey, mint: PublicKey) => {
  const tokenAccount = getAssociatedTokenAddressSync(mint, owner, false);
  const tx = new Transaction();
  tx.add(
    createAssociatedTokenAccountInstruction(payer.publicKey, tokenAccount, owner, mint),
    createMintToInstruction(mint, tokenAccount, payer.publicKey, 1_000 * LAMPORTS_PER_SOL),
  );
  tx.recentBlockhash = context.lastBlockhash;
  tx.sign(payer);

  // process the transaction
  await context.banksClient.processTransaction(tx);
  return tokenAccount;
};

export const encodeBigint = (value: bigint) => {
  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(value);
  return Uint8Array.from(buffer);
};

export type OfferAccount = {
  id: number;
  maker: PublicKey;
  token_mint_a: PublicKey;
  token_mint_b: PublicKey;
  token_b_wanted_amount: number;
  bump: number;
};

// Define DataAccountRaw type for deserialization
export type OfferAccountRaw = {
  id: number;
  maker: Uint8Array;
  token_mint_a: Uint8Array;
  token_mint_b: Uint8Array;
  token_b_wanted_amount: number;
  bump: number;
};

// Define the schema for the account data
export const offerAccountSchema: borsh.Schema = {
  struct: {
    discriminator: 'u64',
    id: 'u64',
    maker: { array: { type: 'u8', len: 32 } },
    token_mint_a: { array: { type: 'u8', len: 32 } },
    token_mint_b: { array: { type: 'u8', len: 32 } },
    token_b_wanted_amount: 'u64',
    bump: 'u8',
  },
};

export const deserializeOfferAccount = (data: Uint8Array): OfferAccount => {
  const account = borsh.deserialize(offerAccountSchema, data) as OfferAccountRaw;
  return {
    id: account.id,
    maker: new PublicKey(account.maker),
    token_mint_a: new PublicKey(account.token_mint_a),
    token_mint_b: new PublicKey(account.token_mint_b),
    token_b_wanted_amount: account.token_b_wanted_amount,
    bump: account.bump,
  };
};
