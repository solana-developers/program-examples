import { BN } from '@coral-xyz/anchor';
import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID as TOKEN_PROGRAM,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
} from '@solana/spl-token';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, type Signer, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { ProgramTestContext } from 'solana-bankrun';

export const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

export async function sleep(seconds: number) {
  new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

export const expectRevert = async (promise: Promise<any>) => {
  try {
    await promise;
    return false;
  } catch {
    return true;
  }
};

export const mintingTokens = async ({
  provider,
  creator,
  context,
  holder = creator,
  mintAKeypair,
  mintBKeypair,
  mintedAmount = 100,
  decimals = 6,
}: {
  provider: BankrunProvider;
  creator: Signer;
  context: ProgramTestContext;
  holder?: Signer;
  mintAKeypair: Keypair;
  mintBKeypair: Keypair;
  mintedAmount?: number;
  decimals?: number;
}) => {
  const minimumLamports = await getMinimumBalanceForRentExemptMint(provider.connection);
  const sendSolToAdminForMinting: Array<TransactionInstruction> = [creator].map((account) =>
    SystemProgram.transfer({
      fromPubkey: provider.publicKey,
      toPubkey: account.publicKey,
      lamports: 10 * LAMPORTS_PER_SOL,
    }),
  );

  const createMintAccounts: Array<TransactionInstruction> = [mintAKeypair, mintBKeypair].map((mint) =>
    SystemProgram.createAccount({
      fromPubkey: provider.publicKey,
      newAccountPubkey: mint.publicKey,
      lamports: minimumLamports,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM,
    }),
  );

  const mintTokensInstructions: Array<TransactionInstruction> = [
    {
      mint: mintAKeypair.publicKey,
      authority: creator.publicKey,

      ata: getAssociatedTokenAddressSync(mintAKeypair.publicKey, holder.publicKey, true),
    },
    {
      mint: mintBKeypair.publicKey,
      authority: creator.publicKey,
      ata: getAssociatedTokenAddressSync(mintBKeypair.publicKey, holder.publicKey, true),
    },
  ].flatMap((mintDetails) => [
    createInitializeMint2Instruction(mintDetails.mint, 6, mintDetails.authority, null, TOKEN_PROGRAM),
    createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, mintDetails.ata, mintDetails.authority, mintDetails.mint, TOKEN_PROGRAM),
    createMintToInstruction(mintDetails.mint, mintDetails.ata, mintDetails.authority, mintedAmount * 10 ** decimals, [], TOKEN_PROGRAM),
  ]);

  // Add all these instructions to our transaction
  const tx = new Transaction();
  tx.instructions = [...sendSolToAdminForMinting, ...createMintAccounts, ...mintTokensInstructions];
  const blockhash = context.lastBlockhash;

  tx.recentBlockhash = blockhash;
  tx.sign(provider.context.payer, creator, mintAKeypair, mintBKeypair);
  await provider.context.banksClient.processTransaction(tx);
};

export interface TestValues {
  id: PublicKey;
  fee: number;
  admin: Keypair;
  mintAKeypair: Keypair;
  mintBKeypair: Keypair;
  defaultSupply: BN;
  ammKey: PublicKey;
  minimumLiquidity: BN;
  poolKey: PublicKey;
  poolAuthority: PublicKey;
  mintLiquidity: PublicKey;
  depositAmountA: BN;
  depositAmountB: BN;
  liquidityAccount: PublicKey;
  poolAccountA: PublicKey;
  poolAccountB: PublicKey;
  holderAccountA: PublicKey;
  holderAccountB: PublicKey;
}

type TestValuesDefaults = {
  [K in keyof TestValues]+?: TestValues[K];
};
export function createValues(defaults?: TestValuesDefaults): TestValues {
  const id = defaults?.id || Keypair.generate().publicKey;
  const admin = Keypair.generate();
  const ammKey = PublicKey.findProgramAddressSync([id.toBuffer()], PROGRAM_ID)[0];

  // Making sure tokens are in the right order
  const mintAKeypair = Keypair.generate();
  let mintBKeypair = Keypair.generate();
  while (new BN(mintBKeypair.publicKey.toBytes()).lt(new BN(mintAKeypair.publicKey.toBytes()))) {
    mintBKeypair = Keypair.generate();
  }

  const poolAuthority = PublicKey.findProgramAddressSync(
    [ammKey.toBuffer(), mintAKeypair.publicKey.toBuffer(), mintBKeypair.publicKey.toBuffer(), Buffer.from('authority')],
    PROGRAM_ID,
  )[0];
  const mintLiquidity = PublicKey.findProgramAddressSync(
    [ammKey.toBuffer(), mintAKeypair.publicKey.toBuffer(), mintBKeypair.publicKey.toBuffer(), Buffer.from('liquidity')],
    PROGRAM_ID,
  )[0];
  const poolKey = PublicKey.findProgramAddressSync(
    [ammKey.toBuffer(), mintAKeypair.publicKey.toBuffer(), mintBKeypair.publicKey.toBuffer()],
    PROGRAM_ID,
  )[0];
  return {
    id,
    fee: 500,
    admin,
    ammKey,
    mintAKeypair,
    mintBKeypair,
    mintLiquidity,
    poolKey,
    poolAuthority,
    poolAccountA: getAssociatedTokenAddressSync(mintAKeypair.publicKey, poolAuthority, true),
    poolAccountB: getAssociatedTokenAddressSync(mintBKeypair.publicKey, poolAuthority, true),
    liquidityAccount: getAssociatedTokenAddressSync(mintLiquidity, admin.publicKey, true),
    holderAccountA: getAssociatedTokenAddressSync(mintAKeypair.publicKey, admin.publicKey, true),
    holderAccountB: getAssociatedTokenAddressSync(mintBKeypair.publicKey, admin.publicKey, true),
    depositAmountA: new BN(4 * 10 ** 6),
    depositAmountB: new BN(1 * 10 ** 6),
    minimumLiquidity: new BN(100),
    defaultSupply: new BN(100 * 10 ** 6),
  };
}
