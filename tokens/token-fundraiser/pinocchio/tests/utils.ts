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

export const DECIMALS = 6;

export const expectRevert = async (promise: Promise<unknown>) => {
  try {
    await promise;
    throw new Error("Expected a revert");
  } catch {
    return;
  }
};

export const mintingTokens = async ({
  context,
  holder,
  mintKeypair,
  mintedAmount = 100,
  decimals = DECIMALS,
}: {
  context: ProgramTestContext;
  holder: Signer;
  mintKeypair: Keypair;
  mintedAmount?: number;
  decimals?: number;
}) => {
  async function createMint(context: ProgramTestContext, mint: Keypair, decimals: number) {
    const rent = await context.banksClient.getRent();

    const lamports = rent.minimumBalance(BigInt(MINT_SIZE));

    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: context.payer.publicKey,
        newAccountPubkey: mint.publicKey,
        space: MINT_SIZE,
        lamports: new BN(lamports.toString()).toNumber(),
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeMint2Instruction(
        mint.publicKey,
        decimals,
        context.payer.publicKey,
        context.payer.publicKey,
        TOKEN_PROGRAM_ID,
      ),
    );
    transaction.recentBlockhash = context.lastBlockhash;
    transaction.sign(context.payer, mint);

    await context.banksClient.processTransaction(transaction);
  }

  async function createAssociatedTokenAccountIfNeeded(context: ProgramTestContext, mint: PublicKey, owner: PublicKey) {
    const associatedToken = getAssociatedTokenAddressSync(mint, owner, true);

    const transaction = new Transaction().add(
      createAssociatedTokenAccountIdempotentInstruction(
        context.payer.publicKey,
        associatedToken,
        owner,
        mint,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID,
      ),
    );
    transaction.recentBlockhash = context.lastBlockhash;
    transaction.sign(context.payer);

    await context.banksClient.processTransaction(transaction);
  }

  async function mintTo(context: ProgramTestContext, mint: PublicKey, destination: PublicKey, amount: number | bigint) {
    const transaction = new Transaction().add(
      createMintToInstruction(mint, destination, context.payer.publicKey, amount, [], TOKEN_PROGRAM_ID),
    );
    transaction.recentBlockhash = context.lastBlockhash;
    transaction.sign(context.payer);

    await context.banksClient.processTransaction(transaction);
  }

  // creator creates the mint
  await createMint(context, mintKeypair, decimals);

  // create holder token account
  await createAssociatedTokenAccountIfNeeded(context, mintKeypair.publicKey, holder.publicKey);

  // mint to holders token account
  await mintTo(
    context,
    mintKeypair.publicKey,
    getAssociatedTokenAddressSync(mintKeypair.publicKey, holder.publicKey, true),
    mintedAmount * 10 ** decimals,
  );
};

export interface TestValues {
  programId: PublicKey;
  maker: Keypair;
  mintKeypair: Keypair;
  amountToRaise: BN;
  duration: number;
  fundraiser: PublicKey;
  fundraiserBump: number;
  vault: PublicKey;
  makerAta: PublicKey;
}

export function createValues(): TestValues {
  const programId = PublicKey.unique();
  const maker = Keypair.generate();
  const mintKeypair = Keypair.generate();

  const [fundraiser, fundraiserBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("fundraiser"), maker.publicKey.toBuffer()],
    programId,
  );

  return {
    programId,
    maker,
    mintKeypair,
    // 30 tokens target; well above the 3^decimals minimum.
    amountToRaise: new BN(30 * 10 ** DECIMALS),
    duration: 0,
    fundraiser,
    fundraiserBump,
    vault: getAssociatedTokenAddressSync(mintKeypair.publicKey, fundraiser, true),
    makerAta: getAssociatedTokenAddressSync(mintKeypair.publicKey, maker.publicKey, true),
  };
}
