import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import { SplTokenMinter } from '../target/types/spl_token_minter'; // Adjust path if necessary

describe('spl_token_minter', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.SplTokenMinter as Program<SplTokenMinter>;

  const mintAuthority = provider.wallet as anchor.Wallet;
  let mintAccount: PublicKey;
  let recipientTokenAccount: PublicKey;
  const recipient = Keypair.generate();

  it('Creates a new token mint', async () => {
    // Generate a new Keypair for the mint account
    const mintAccountKeypair = Keypair.generate();
    mintAccount = mintAccountKeypair.publicKey;

    // Define decimals and freeze authority for the mint
    const decimals = 9;
    const freezeAuthority = mintAuthority.publicKey;

    // Call the `create_token_mint` instruction
    await program.methods
      .createTokenMint(decimals, freezeAuthority)
      .accounts({
        mintAuthority: mintAuthority.publicKey,
        mintAccount: mintAccount,
      })
      .signers([mintAuthority.payer, mintAccountKeypair])
      .rpc();
  });

  it('Mints tokens to the associated token account', async () => {
    const amount = new anchor.BN(1000 * 10 ** 9); // Mint 1000 tokens (adjust for decimals)

    recipientTokenAccount = await getAssociatedTokenAddressSync(mintAccount, recipient.publicKey);

    // Mint tokens to the recipient's associated token account
    await program.methods
      .mint(amount)
      .accounts({
        mintAccount: mintAccount,
        mintAuthority: mintAuthority.publicKey,
        recipient: recipient.publicKey,
      })
      .signers([mintAuthority.payer])
      .rpc();

    // Verify the balance of the associated token account
    const recipientTokenAccountInfo = await program.provider.connection.getTokenAccountBalance(recipientTokenAccount);
    console.log('Recipient Token Account Balance:', recipientTokenAccountInfo.value.uiAmount);

    assert.equal(recipientTokenAccountInfo.value.uiAmount, 1000, 'Minted token amount does not match');
  });
});
