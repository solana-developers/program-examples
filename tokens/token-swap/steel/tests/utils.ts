import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { ProgramTestContext } from "solana-bankrun";

export const instructionDiscriminators = {
  CreateAmm: Buffer.from([0]),
};

export const getCreateAmmInstructionData = (id: PublicKey, fee: number) => {
  const buffer = Buffer.alloc(2);
  buffer.writeUint16LE(fee, 0);
  return Buffer.concat([
    instructionDiscriminators.CreateAmm,
    id.toBuffer(),
    Buffer.from(buffer),
  ]);
};

export const createAMint = async (
  context: ProgramTestContext,
  payer: Keypair,
  mint: Keypair
) => {
  const tx = new Transaction();
  tx.add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint.publicKey,
      // the `space` required for a token mint is accessible in the `@solana/spl-token` sdk
      space: MINT_SIZE,
      // store enough lamports needed for our `space` to be rent exempt
      lamports: Number(
        (await context.banksClient.getRent()).minimumBalance(BigInt(MINT_SIZE))
      ),
      // tokens are owned by the "token program"
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMint2Instruction(
      mint.publicKey,
      9,
      payer.publicKey,
      payer.publicKey
    )
  );
  tx.recentBlockhash = context.lastBlockhash;
  tx.sign(payer, mint);

  // process the transaction
  await context.banksClient.processTransaction(tx);
};

export const mintTo = async (
  context: ProgramTestContext,
  payer: Keypair,
  owner: PublicKey,
  mint: PublicKey
) => {
  const tokenAccount = getAssociatedTokenAddressSync(mint, owner, false);
  const tx = new Transaction();
  tx.add(
    createAssociatedTokenAccountInstruction(
      payer.publicKey,
      tokenAccount,
      owner,
      mint
    ),
    createMintToInstruction(mint, tokenAccount, payer.publicKey, 1_000_000)
  );
  tx.recentBlockhash = context.lastBlockhash;
  tx.sign(payer);

  // process the transaction
  await context.banksClient.processTransaction(tx);
  return tokenAccount;
};
