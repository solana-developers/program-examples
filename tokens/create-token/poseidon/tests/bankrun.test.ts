import { before, describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { BanksClient, BanksTransactionResultWithMeta, startAnchor } from 'solana-bankrun';
import type { CreateToken } from '../target/types/create_token';

const IDL = require('../target/idl/create_token.json');
const PROGRAM_ID = new PublicKey(IDL.address);
const METADATA_PROGRAM_ID = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');
async function createAndProcessTransaction(
  client: BanksClient,
  payer: Keypair,
  instruction: TransactionInstruction,
  additionalSigners: Keypair[] = [],
): Promise<BanksTransactionResultWithMeta> {
  const tx = new Transaction();
  // Get the latest blockhash
  const [latestBlockhash] = await client.getLatestBlockhash();
  tx.recentBlockhash = latestBlockhash;
  // Add transaction instructions
  tx.add(instruction);
  tx.feePayer = payer.publicKey;
  //Add signers
  tx.sign(payer, ...additionalSigners);
  // Process transaction
  const result = await client.tryProcessTransaction(tx);
  return result;
}

describe('Bankrun example', async () => {
  const context = await startAnchor(
    '',
    [
      { name: 'create_tokens', programId: PROGRAM_ID },
      { name: 'mpl_token_metadata', programId: METADATA_PROGRAM_ID },
    ],
    [],
  );
  const provider = new BankrunProvider(context);
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CreateToken>(IDL, provider);
  // Generate new keypairs to use as addresses for mint accounts.
  const mintKeypair1 = Keypair.generate();
  const mintKeypair2 = Keypair.generate();

  const metadata = {
    name: 'Solana Gold',
    symbol: 'GOLDSOL',
    uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
  };
  before(async () => {
    //Transfer SOL to the mint account to cover rent
    const transferInstruction1 = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: mintKeypair1.publicKey,
      lamports: 2 * LAMPORTS_PER_SOL,
    });

    await createAndProcessTransaction(context.banksClient, payer.payer, transferInstruction1, [payer.payer]);
    const userBalance1 = await context.banksClient.getBalance(mintKeypair1.publicKey);
    console.log(`mintKeypair1 balance after funding: ${userBalance1}`);

    //Transfer SOL to the mint account to cover rent
    const transferInstruction2 = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: mintKeypair2.publicKey,
      lamports: 2 * LAMPORTS_PER_SOL,
    });

    await createAndProcessTransaction(context.banksClient, payer.payer, transferInstruction2, [payer.payer]);
    const userBalance2 = await context.banksClient.getBalance(mintKeypair2.publicKey);
    console.log(`mintKeypair2 balance after funding: ${userBalance2}`);
  });

  it('Create an SPL Token!', async () => {
    // Generate new keypair to use as address for mint account.

    // SPL Token default = 9 decimals
    const transactionSignature = await program.methods
      .createTokenMint(mintKeypair1.publicKey, 0, metadata.name, metadata.symbol, metadata.uri, 9)
      .accounts({
        mintAccount: mintKeypair1.publicKey,
      })
      .signers([mintKeypair1])
      .rpc();

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair1.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it('Creates an NFT!', async () => {
    // NFT default = 0 decimals
    const transactionSignature = await program.methods
      .createTokenMint(mintKeypair2.publicKey, 0, metadata.name, metadata.symbol, metadata.uri, 0)
      .accounts({
        mintAccount: mintKeypair2.publicKey,
      })
      .signers([mintKeypair2])
      .rpc();

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair2.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});

//for the test to work you first have to run
// solana program dump metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s mpl_token_metadata.so
//to dump a copy of the metadata program locally in your root directory
