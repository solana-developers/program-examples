import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { getMint } from '@solana/spl-token';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { CreateToken } from '../target/types/create_token';

const IDL = require('../target/idl/create_token.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun - create_token', async () => {
  // Start the Bankrun context
  const context = await startAnchor('', [{ name: 'create_token', programId: PROGRAM_ID }], []);

  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const program = new anchor.Program<CreateToken>(IDL, provider);

  const payer = provider.wallet as anchor.Wallet;

  it('Creates a new token mint', async () => {
    const mintKeypair = Keypair.generate();
    const decimals = 9;

    const tx = await program.methods
      .createTokenMint(decimals)
      .accounts({
        payer: payer.publicKey,
        mint: mintKeypair.publicKey,
      })
      .signers([payer.payer, mintKeypair])
      .rpc();

    console.log('Your transaction signature', tx);

    const mintInfo = await getMint(provider.connection, mintKeypair.publicKey);

    assert.equal(mintInfo.decimals, decimals, 'Mint decimals should match the specified value');
    assert.equal(mintInfo.mintAuthority?.toBase58(), payer.publicKey.toBase58(), 'Mint authority should be the payer');

    console.log('Mint created successfully:', mintKeypair.publicKey.toBase58());
  });
});
