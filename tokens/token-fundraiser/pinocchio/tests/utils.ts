import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Keypair, PublicKey, type Signer, SystemProgram, Transaction } from "@solana/web3.js";
import BN from "bn.js";
import type { ProgramTestContext } from "solana-bankrun";

export const expectRevert = async (promise: Promise<unknown>) => {
  let reverted = false;
  try {
    await promise;
  } catch {
    reverted = true;
  }
  if (!reverted) {
    throw new Error("Expected a revert, but the transaction succeeded");
  }
};

// Transfers SOL from the bankrun payer to another account so it can fund account
// creation and pay fees.
export const fundAccount = async (context: ProgramTestContext, recipient: PublicKey, lamports: number) => {
  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: context.payer.publicKey,
      toPubkey: recipient,
      lamports,
    }),
  );
  transaction.recentBlockhash = context.lastBlockhash;
  transaction.sign(context.payer);
  await context.banksClient.processTransaction(transaction);
};

// Creates a mint, gives `holder` an associated token account, and mints
// `mintedAmount` whole tokens into it.
export const mintingTokens = async ({
  context,
  holder,
  mintKeypair,
  mintedAmount = 100,
  decimals = 6,
}: {
  context: ProgramTestContext;
  holder: Signer;
  mintKeypair: Keypair;
  mintedAmount?: number;
  decimals?: number;
}) => {
  const rent = await context.banksClient.getRent();
  const lamports = rent.minimumBalance(BigInt(MINT_SIZE));

  const createMintTx = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: context.payer.publicKey,
      newAccountPubkey: mintKeypair.publicKey,
      space: MINT_SIZE,
      lamports: new BN(lamports.toString()).toNumber(),
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMint2Instruction(
      mintKeypair.publicKey,
      decimals,
      context.payer.publicKey,
      context.payer.publicKey,
      TOKEN_PROGRAM_ID,
    ),
  );
  createMintTx.recentBlockhash = context.lastBlockhash;
  createMintTx.sign(context.payer, mintKeypair);
  await context.banksClient.processTransaction(createMintTx);

  const holderAta = getAssociatedTokenAddressSync(mintKeypair.publicKey, holder.publicKey, true);

  const mintToTx = new Transaction().add(
    createAssociatedTokenAccountIdempotentInstruction(
      context.payer.publicKey,
      holderAta,
      holder.publicKey,
      mintKeypair.publicKey,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    ),
    createMintToInstruction(
      mintKeypair.publicKey,
      holderAta,
      context.payer.publicKey,
      mintedAmount * 10 ** decimals,
      [],
      TOKEN_PROGRAM_ID,
    ),
  );
  mintToTx.recentBlockhash = context.lastBlockhash;
  mintToTx.sign(context.payer);
  await context.banksClient.processTransaction(mintToTx);
};
