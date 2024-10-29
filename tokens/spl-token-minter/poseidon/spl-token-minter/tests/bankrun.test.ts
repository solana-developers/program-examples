import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { getAccount, getMint } from '@solana/spl-token';
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { SplTokenMinter } from '../target/types/spl_token_minter';

const IDL = require('../target/idl/spl_token_minter.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun - SPL Token Minter', async () => {
  // Start the Bankrun context
  const context = await startAnchor('', [{ name: 'spl_token_minter', programId: PROGRAM_ID }], []);

  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const program = new anchor.Program<SplTokenMinter>(IDL, provider);

  const payer = provider.wallet as anchor.Wallet;
  let mintAccount: PublicKey;
  let recipientTokenAccount: PublicKey;
  const recipient = Keypair.generate();

  it('Creates a new token mint', async () => {
    const mintAccountKeypair = Keypair.generate();
    mintAccount = mintAccountKeypair.publicKey;

    const decimals = 9;
    const freezeAuthority = payer.publicKey;

    const tx = await program.methods
      .createTokenMint(decimals, freezeAuthority)
      .accounts({
        mintAuthority: payer.publicKey,
        mintAccount: mintAccount,
      })
      .signers([payer.payer, mintAccountKeypair])
      .rpc();

    console.log('Transaction signature for createTokenMint:', tx);

    const mintInfo = await getMint(provider.connection, mintAccount);

    assert.equal(mintInfo.decimals, decimals, 'Mint account decimals should match specified value');
    assert.equal(mintInfo.mintAuthority?.toBase58(), payer.publicKey.toBase58(), 'Mint authority should be the payer');
    console.log('Mint created successfully:', mintAccount.toBase58());
  });

  it('Mints tokens to the associated token account', async () => {
    const amount = new anchor.BN(1000 * 10 ** 9);

    // Create an associated token account for the recipient
    recipientTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintAccount,
      owner: recipient.publicKey,
    });

    const tx = await program.methods
      .mint(amount)
      .accounts({
        mintAccount: mintAccount,
        mintAuthority: payer.publicKey,
        recipient: recipient.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log('Transaction signature for mint:', tx);

    const recipientTokenAccountInfo = await getAccount(provider.connection, recipientTokenAccount);
    const mintedAmount = Number(recipientTokenAccountInfo.amount) / 10 ** 9;

    assert.equal(mintedAmount, 1000, 'Minted token amount does not match');
    console.log('Recipient Token Account Balance:', mintedAmount);
  });
});
