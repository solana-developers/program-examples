import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { getAccount, getAssociatedTokenAddressSync, getMint } from '@solana/spl-token';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { TokenMinter } from '../target/types/token_minter';

const IDL = require('../target/idl/token_minter.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun - Token Minter', async () => {
  // Start the Bankrun context
  const context = await startAnchor('', [{ name: 'token_minter', programId: PROGRAM_ID }], []);

  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const program = new anchor.Program<TokenMinter>(IDL, provider);

  const payer = provider.wallet as anchor.Wallet;
  let mintAccount: PublicKey;
  let associatedTokenAccount: PublicKey;

  it('Creates a new token mint', async () => {
    const [mintAccountPda, mintBump] = PublicKey.findProgramAddressSync([Buffer.from('mint')], program.programId);
    mintAccount = mintAccountPda;

    // Call the `create_token` instruction and capture the transaction signature
    const txSignature = await program.methods
      .createToken(9) // 9 decimals
      .accounts({
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log('Transaction Signature for createTokenMint:', txSignature);

    // Confirm mint account creation
    const mintInfo = await getMint(provider.connection, mintAccount);
    assert.equal(mintInfo.decimals, 9, 'Mint account decimals should match specified value');
    console.log('Mint created successfully:', mintAccount.toBase58());
  });

  it('Mints tokens to the associated token account', async () => {
    const amount = new anchor.BN(1000 * 10 ** 9); // Mint 1000 tokens

    // Derive the associated token account for the payer
    associatedTokenAccount = getAssociatedTokenAddressSync(mintAccount, payer.publicKey);

    const txSignature = await program.methods
      .mintToken(amount)
      .accounts({
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log('Transaction Signature for mint:', txSignature);

    // Verify the balance of the associated token account
    const associatedTokenAccountInfo = await getAccount(provider.connection, associatedTokenAccount);
    const accountBalance = Number(associatedTokenAccountInfo.amount) / 10 ** 9;
    assert.equal(accountBalance, 1000, 'Minted token amount does not match');
    console.log('Associated Token Account Balance:', accountBalance);
  });
});
