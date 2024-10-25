import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { TOKEN_PROGRAM_ID, createMint, getAccount, getMint, getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { PublicKey } from '@solana/web3.js';
import { assert } from 'chai';
import { SplTokenMinter } from '../target/types/spl_token_minter';

describe('spl_token_minter', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SplTokenMinter as Program<SplTokenMinter>;
  const wallet = provider.wallet as anchor.Wallet;
  const payer = wallet.payer;
  let mint: PublicKey;

  it('Creates a new token mint', async () => {
    const decimals = 9;
    mint = await createMint(provider.connection, payer, payer.publicKey, payer.publicKey, decimals, undefined, undefined, TOKEN_PROGRAM_ID);
    await program.methods
      .createToken(decimals, wallet.publicKey)
      .accounts({
        payer: payer.publicKey,
        mint: mint,
      })
      .rpc();

    // Fetch the mint account and verify that it was created
    const mintAccount = await getMint(provider.connection, mint);
    assert.equal(mintAccount.decimals, decimals, 'Decimals should match the input');
    console.log('Mint Account', mintAccount);
  });

  it('Mints tokens to the associated token account', async () => {
    const amount = new anchor.BN(1000);
    // Create or get the associated token account for the user
    const userAssociatedTokenAccount = await getOrCreateAssociatedTokenAccount(provider.connection, payer, mint, payer.publicKey);
    console.log('Associated Account', userAssociatedTokenAccount);
    await program.methods
      .mint(amount)
      .accounts({
        mintAccount: mint,
        signer: payer.publicKey,
        toAccount: userAssociatedTokenAccount.address,
      })
      .signers([payer])
      .rpc();

    // Fetch the token account to verify the balance
    const tokenAccountInfo = await getAccount(provider.connection, userAssociatedTokenAccount.address);
    console.log('Token Account Info', tokenAccountInfo);
    assert.equal(tokenAccountInfo.amount, BigInt(amount.toString()), 'Balance should be minted');
  });
});