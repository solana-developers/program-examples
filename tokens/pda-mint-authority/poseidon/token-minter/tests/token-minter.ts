import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, PublicKey } from '@solana/web3.js';
import { assert } from 'chai';
import { TokenMinter } from '../target/types/token_minter';

describe('token_minter', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenMinter as Program<TokenMinter>;

  const payer = provider.wallet as anchor.Wallet;
  let mintAccount: PublicKey;
  let associatedTokenAccount: PublicKey;

  it('Creates a new token mint', async () => {
    // Generate a new Keypair for the mint account
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

    console.log('Transaction Signature:', txSignature);

    // Confirm mint account creation
    const mintInfo = await provider.connection.getAccountInfo(mintAccount);
    assert.ok(mintInfo !== null, 'Mint account was not created');
  });

  it('Mints tokens to the associated token account', async () => {
    const amount = new anchor.BN(1000 * 10 ** 9); // Mint 1000 tokens

    // Derive the associated token account for the recipient
    associatedTokenAccount = getAssociatedTokenAddressSync(mintAccount, payer.publicKey);

    // Call the `mint_token` instruction and capture the transaction signature
    const txSignature = await program.methods
      .mintToken(amount)
      .accounts({
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log('Transaction Signature:', txSignature);

    // Verify the balance of the associated token account
    const accountBalance = await provider.connection.getTokenAccountBalance(associatedTokenAccount);
    assert.equal(accountBalance.value.uiAmount, 1000, 'Minted token amount does not match');
    console.log('Associated Token Account Balance:', accountBalance.value.uiAmount);
  });
});
