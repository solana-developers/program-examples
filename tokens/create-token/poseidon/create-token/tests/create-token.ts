import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { TOKEN_PROGRAM_ID, getMint } from '@solana/spl-token';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import { CreateToken } from '../target/types/create_token';

describe('create_token_program', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CreateToken as Program<CreateToken>;

  const payer = provider.wallet as anchor.Wallet;

  it('Creates a new token mint', async () => {
    // Generate a new keypair for the mint account.
    const mintKeypair = anchor.web3.Keypair.generate();

    // Set the number of decimals for the mint.
    const decimals = 9;

    // Transaction to create the mint account.
    const tx = await program.methods
      .createTokenMint(decimals)
      .accounts({
        payer: provider.wallet.publicKey,
        mint: mintKeypair.publicKey,
      })
      .signers([payer.payer, mintKeypair])
      .rpc();

    console.log('Your transaction signature', tx);

    // Fetch the mint account details to verify
    const mintInfo = await getMint(provider.connection, mintKeypair.publicKey);

    console.log('mintInfo decimals:', mintInfo.decimals);
    console.log('mintInfo mintAuthority:', mintInfo.mintAuthority.toBase58());

    // Assertions to verify the mint was created with the correct parameters
    assert.equal(mintInfo.decimals, decimals, 'Mint decimals should match the specified value');
    assert.equal(mintInfo.mintAuthority?.toBase58(), payer.publicKey.toBase58(), 'Mint authority should be the payer');

    console.log('Mint created successfully:', mintKeypair.publicKey.toBase58());
  });
});
