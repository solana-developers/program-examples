import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  mintTo,
} from '@solana/spl-token';
import { Keypair, PublicKey, type Signer, SystemProgram, Transaction } from '@solana/web3.js';
import BN from 'bn.js';
import { ProgramTestContext } from 'solana-bankrun';

export async function sleep(seconds: number) {
  new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

export const expectRevert = async (promise: Promise<any>) => {
  try {
    await promise;
    throw new Error('Expected a revert');
  } catch {
    return;
  }
};

export const mintingTokens = async ({
  context,
  creator,
  holder = creator,
  mintAKeypair,
  mintBKeypair,
  mintedAmount = 100,
  decimals = 6,
}: {
  context: ProgramTestContext;
  creator: Signer;
  holder?: Signer;
  mintAKeypair: Keypair;
  mintBKeypair: Keypair;
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
      createInitializeMint2Instruction(mint.publicKey, decimals, context.payer.publicKey, context.payer.publicKey, TOKEN_PROGRAM_ID),
    );
    transaction.recentBlockhash = context.lastBlockhash;
    transaction.sign(context.payer, mint);

    await context.banksClient.processTransaction(transaction);
  }

  async function createAssociatedTokenAccountIfNeeded(context: ProgramTestContext, mint: PublicKey, owner: PublicKey) {
    const associatedToken = getAssociatedTokenAddressSync(mint, owner, true);

    const rent = await context.banksClient.getRent();

    rent.minimumBalance(BigInt(MINT_SIZE));

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
    const transaction = new Transaction().add(createMintToInstruction(mint, destination, context.payer.publicKey, amount, [], TOKEN_PROGRAM_ID));
    transaction.recentBlockhash = context.lastBlockhash;
    transaction.sign(context.payer);

    await context.banksClient.processTransaction(transaction);
  }

  // creator creates the mint
  await createMint(context, mintAKeypair, decimals);
  await createMint(context, mintBKeypair, decimals);

  // create holder token account
  await createAssociatedTokenAccountIfNeeded(context, mintAKeypair.publicKey, holder.publicKey);
  await createAssociatedTokenAccountIfNeeded(context, mintBKeypair.publicKey, holder.publicKey);

  // mint to holders token account
  await mintTo(
    context,
    mintAKeypair.publicKey,
    getAssociatedTokenAddressSync(mintAKeypair.publicKey, holder.publicKey, true),
    mintedAmount * 10 ** decimals,
  );
  await mintTo(
    context,
    mintBKeypair.publicKey,
    getAssociatedTokenAddressSync(mintBKeypair.publicKey, holder.publicKey, true),
    mintedAmount * 10 ** decimals,
  );
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
  programId: PublicKey;
}

type TestValuesDefaults = {
  [K in keyof TestValues]+?: TestValues[K];
};
export function createValues(defaults?: TestValuesDefaults): TestValues {
  const programId = PublicKey.unique();
  const id = defaults?.id || Keypair.generate().publicKey;
  const admin = Keypair.generate();
  const ammKey = PublicKey.findProgramAddressSync([id.toBuffer()], programId)[0];

  // Making sure tokens are in the right order
  const mintAKeypair = Keypair.generate();
  let mintBKeypair = Keypair.generate();
  while (new BN(mintBKeypair.publicKey.toBytes()).lt(new BN(mintAKeypair.publicKey.toBytes()))) {
    mintBKeypair = Keypair.generate();
  }

  const poolAuthority = PublicKey.findProgramAddressSync(
    [ammKey.toBuffer(), mintAKeypair.publicKey.toBuffer(), mintBKeypair.publicKey.toBuffer(), Buffer.from('authority')],
    programId,
  )[0];
  const mintLiquidity = PublicKey.findProgramAddressSync(
    [ammKey.toBuffer(), mintAKeypair.publicKey.toBuffer(), mintBKeypair.publicKey.toBuffer(), Buffer.from('liquidity')],
    programId,
  )[0];
  const poolKey = PublicKey.findProgramAddressSync(
    [ammKey.toBuffer(), mintAKeypair.publicKey.toBuffer(), mintBKeypair.publicKey.toBuffer()],
    programId,
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
    programId,
  };
}
