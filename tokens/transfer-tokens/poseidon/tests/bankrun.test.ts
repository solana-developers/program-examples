import { before, describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, LAMPORTS_PER_SOL, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { BanksClient, BanksTransactionResultWithMeta, startAnchor } from 'solana-bankrun';
import type { TransferTokensProgram } from '../target/types/transfer_tokens_program';

const IDL = require('../target/idl/transfer_tokens_program.json');
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
      { name: 'transfer_tokens', programId: PROGRAM_ID },
      { name: 'mpl_token_metadata', programId: METADATA_PROGRAM_ID },
    ],
    [],
  );
  const provider = new BankrunProvider(context);
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<TransferTokensProgram>(IDL, provider);
  // Generate new keypair to use as address for mint account.
  const mintKeypair = new Keypair();

  // Generate new keypair to use as address for recipient wallet.
  const recipient = new Keypair();

  // Derive the associated token address account for the mint and payer.
  const senderTokenAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);

  // Derive the associated token address account for the mint and recipient.
  const recipientTokenAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, recipient.publicKey);
  const metadata = {
    name: 'Solana Gold',
    symbol: 'GOLDSOL',
    uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
  };
  before(async () => {
    //Transfer SOL to the mint account to cover rent
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: mintKeypair.publicKey,
      lamports: 2 * LAMPORTS_PER_SOL,
    });

    await createAndProcessTransaction(context.banksClient, payer.payer, transferInstruction, [payer.payer]);
    const userBalance = await context.banksClient.getBalance(mintKeypair.publicKey);
    console.log(`User balance after funding: ${userBalance}`);
  });

  it('Create an SPL Token!', async () => {
    // Generate new keypair to use as address for mint account.

    // SPL Token default = 9 decimals
    const transactionSignature = await program.methods
      .createTokenMint(mintKeypair.publicKey, 0, metadata.name, metadata.symbol, metadata.uri, 9)
      .accounts({
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it('Mint tokens!', async () => {
    // Amount of tokens to mint.
    const amount = new anchor.BN(100);

    // Mint the tokens to the associated token account.
    const transactionSignature = await program.methods
      .mintToken(amount)
      .accounts({
        mintAuthority: payer.publicKey,
        recipient: senderTokenAddress,
        mintAccount: mintKeypair.publicKey,
      })
      .rpc();

    console.log('Success!');
    console.log(`   Associated Token Account Address: ${senderTokenAddress}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it('Transfer tokens!', async () => {
    // Amount of tokens to transfer.
    const amount = new anchor.BN(50);

    const transactionSignature = await program.methods
      .transferTokens(amount)
      .accounts({
        sender: senderTokenAddress,
        recipient: recipientTokenAddress,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([])
      .rpc();

    console.log('Success!');
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});

//run dump.sh before you run this test
