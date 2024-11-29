import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import { Connection, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import * as dotenv from 'dotenv';
import { startAnchor } from 'solana-bankrun';
import { EscrowProgram } from '../target/types/escrow_program';

dotenv.config();

// Helper function to get a Keypair from file
async function getKeypairFromEnvFile(pathEnvVar: string): Promise<anchor.web3.Keypair> {
  const filePath = process.env[pathEnvVar];
  if (!filePath) {
    throw new Error(`Environment variable ${pathEnvVar} is not set`);
  }
  return anchor.web3.Keypair.fromSecretKey(new Uint8Array(JSON.parse(require('node:fs').readFileSync(filePath, 'utf8'))));
}

const IDL = require('../target/idl/escrow_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('escrow_program_bankrun', async () => {
  // Configure the client to use bankrun
  const context = await startAnchor('', [{ name: 'escrow_program', programId: new PublicKey(PROGRAM_ID) }], []);
  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);

  const program = new anchor.Program<EscrowProgram>(require('../target/idl/escrow_program.json'), provider);
  const connection = new Connection(process.env.SOLANA_RPC_URL || 'http://127.0.0.1:8899', 'confirmed');

  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerTokenAccount = null;
  let takerTokenAccount = null;
  const vault = null;
  let maker: anchor.web3.Keypair;
  let taker: anchor.web3.Keypair;

  const depositAmount = 1000;
  const offerAmount = 500;
  const seed = new anchor.BN(123);

  beforeEach('Setup accounts and mint tokens', async () => {
    // Get maker and taker keypairs from environment variables
    maker = await getKeypairFromEnvFile('MAKER_KEYPAIR_PATH');
    taker = await getKeypairFromEnvFile('TAKER_KEYPAIR_PATH');

    // Get mint addresses from environment variables
    const mintAAddress = process.env.MINT_A_PUBLIC_KEY;
    const mintBAddress = process.env.MINT_B_PUBLIC_KEY;

    if (!mintAAddress || !mintBAddress) {
      throw new Error('MINT_A_ADDRESS and MINT_B_ADDRESS environment variables must be set');
    }

    mintA = new PublicKey(mintAAddress);
    mintB = new PublicKey(mintBAddress);

    // Create token accounts for the maker and taker
    makerTokenAccount = await getOrCreateAssociatedTokenAccount(connection, maker, mintA, maker.publicKey, true);

    takerTokenAccount = await getOrCreateAssociatedTokenAccount(provider.connection, maker, mintB, taker.publicKey, true);

    await mintTo(connection, maker, mintA, makerTokenAccount.address, maker.publicKey, depositAmount);
  });

  it('Initialize escrow', async () => {
    const [escrowPda, escrowBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('escrow'), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, 'le', 8)],
      program.programId,
    );

    const [vaultPda, vaultBump] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('vault'), escrowPda.toBuffer()], program.programId);

    const [authPda, authBump] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('auth')], program.programId);

    await program.methods
      .make(new anchor.BN(depositAmount), new anchor.BN(offerAmount), seed)
      .accounts({
        maker: maker.publicKey,
        makerMint: mintA,
        takerMint: mintB,
      })
      .signers([maker])
      .rpc();

    const escrowState = await program.account.escrowState.fetch(escrowPda);

    assert.ok(escrowState.maker.equals(maker.publicKey));
    assert.ok(escrowState.amount.toNumber() === offerAmount);
  });

  it('Take escrow', async () => {
    const [escrowPda, escrowBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('escrow'), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, 'le', 8)],
      program.programId,
    );

    const [vaultPda, vaultBump] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('vault'), escrowPda.toBuffer()], program.programId);

    const [authPda, authBump] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('auth')], program.programId);

    const takerReceiveAta = await getOrCreateAssociatedTokenAccount(provider.connection, taker, mintA, taker.publicKey);

    await program.methods
      .take()
      .accounts({
        taker: taker.publicKey,
      })
      .signers([taker])
      .rpc();

    const takerBalance = await provider.connection.getTokenAccountBalance(takerReceiveAta.address);

    assert.ok(takerBalance.value.uiAmount === offerAmount / 1e9); // Adjusting for decimals
  });

  it('Refund escrow', async () => {
    const [escrowPda, escrowBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('escrow'), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, 'le', 8)],
      program.programId,
    );

    const [vaultPda, vaultBump] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('vault'), escrowPda.toBuffer()], program.programId);

    const [authPda, authBump] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('auth')], program.programId);

    await program.methods
      .refund()
      .accounts({
        makerMint: mintA,
      })
      .signers([maker])
      .rpc();

    const makerBalance = await provider.connection.getTokenAccountBalance(makerTokenAccount.address);

    assert.ok(makerBalance.value.uiAmount === depositAmount / 1e9); // Adjusting for decimals
  });
});
