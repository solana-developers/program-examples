import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair } from '@solana/web3.js';
import { SplMintProgram } from '../target/types/spl_mint_program';

describe('spl-token-minter', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.SplTokenMinter as Program<SplMintProgram>;
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const mintKeypair = new Keypair();

  it('Create Token', async () => {
    // Add your test here.
    const tx = await program.methods
      .create()
      .accounts({
        maker: payer.publicKey,
        mint: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();
    console.log('Your transaction signature', tx);
  });

  it('Mint Token', async () => {
    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);
    // Amount of tokens to mint.
    const amount = new anchor.BN(100);

    // Add your test here.
    const tx = await program.methods
      .mint(amount)
      .accounts({
        auth: payer.publicKey,
        to: associatedTokenAccountAddress,
      })
      .signers([mintKeypair])
      .rpc();
    console.log('Your transaction signature', tx);
  });
});
