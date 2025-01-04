import { before, describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { getAssociatedTokenAddressSync, getMint } from '@solana/spl-token';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { TransferTokensProgram } from '../target/types/transfer_tokens_program';

const IDL = require('../target/idl/transfer_tokens_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun - Transfer Tokens Program', async () => {
  // Start the Bankrun context
  const context = await startAnchor('', [{ name: 'transfer_tokens_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);

  const program = new anchor.Program<TransferTokensProgram>(IDL, provider);

  const payer = provider.wallet as anchor.Wallet;
  const mintKeypair = Keypair.generate();
  const recipientKeypair = Keypair.generate();

  const DECIMALS = 9;
  let senderTokenAccount: PublicKey;
  let recipientTokenAccount: PublicKey;

  before(async () => {
    // Derive associated token account addresses
    senderTokenAccount = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);
    recipientTokenAccount = getAssociatedTokenAddressSync(mintKeypair.publicKey, recipientKeypair.publicKey);
  });

  it('Creates a new SPL Token', async () => {
    const txSig = await program.methods
      .createToken(DECIMALS, payer.publicKey)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([payer.payer, mintKeypair])
      .rpc();

    console.log(`Transaction Signature: ${txSig}`);
    const mintInfo = await getMint(provider.connection, mintKeypair.publicKey);

    assert.equal(mintInfo.decimals, DECIMALS, 'Mint decimals should match the specified value');
    assert.equal(mintInfo.mintAuthority?.toBase58(), payer.publicKey.toBase58(), 'Mint authority should be the payer');

    console.log('Mint created successfully:', mintKeypair.publicKey.toBase58());
  });

  it("Mints tokens to sender's account", async () => {
    const mintAmount = new anchor.BN(1_000_000_000); // Mint 1 token with 9 decimals

    const txSig = await program.methods
      .mint(mintAmount)
      .accounts({
        mintAccount: mintKeypair.publicKey,
        mintAuthority: payer.publicKey,
        recipient: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    const mintInfo = await getMint(provider.connection, mintKeypair.publicKey);

    assert.equal(mintInfo.supply.toString(), mintAmount.toString(), 'Minted amount should match the specified value');
    console.log(`Minted ${mintAmount.toString()} tokens to ${senderTokenAccount}`);
    console.log(`Transaction Signature: ${txSig}`);
  });

  it('Transfers tokens from sender to recipient', async () => {
    const transferAmount = new anchor.BN(500_000_000); // Transfer 0.5 tokens

    const txSig = await program.methods
      .transferTokens(transferAmount)
      .accounts({
        sender: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
        recipient: recipientKeypair.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log(`Transferred ${transferAmount.toString()} tokens to ${recipientTokenAccount}`);
    console.log(`Transaction Signature: ${txSig}`);
  });
});
