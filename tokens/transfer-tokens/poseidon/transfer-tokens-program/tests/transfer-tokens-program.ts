import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import { Keypair, PublicKey } from '@solana/web3.js';
import { assert } from 'chai';
import { TransferTokensProgram } from '../target/types/transfer_tokens_program';

describe('transfer_tokens_program', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TransferTokensProgram as Program<TransferTokensProgram>;

  const payer = provider.wallet as anchor.Wallet;

  // Keypair for mint and recipient
  const mintKeypair = Keypair.generate();
  const recipientKeypair = Keypair.generate();

  let mintAccount: PublicKey;
  let senderTokenAccount: PublicKey;
  let recipientTokenAccount: PublicKey;

  before(async () => {
    // Create an SPL token mint.
    mintAccount = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      9, // Decimals
    );

    // Get or create associated token accounts for sender and recipient.
    senderTokenAccount = getAssociatedTokenAddressSync(mintAccount, payer.publicKey);
    recipientTokenAccount = getAssociatedTokenAddressSync(mintAccount, recipientKeypair.publicKey);

    // Create the associated token account for the sender and recipient.
    await getOrCreateAssociatedTokenAccount(provider.connection, payer.payer, mintAccount, payer.publicKey);
    await getOrCreateAssociatedTokenAccount(provider.connection, payer.payer, mintAccount, recipientKeypair.publicKey);

    // Mint some tokens to the sender's associated token account.
    await mintTo(
      provider.connection,
      payer.payer,
      mintAccount,
      senderTokenAccount,
      payer.payer,
      100 * 10 ** 9, // 100 tokens
    );
  });

  it('Transfers tokens between accounts', async () => {
    const transferAmount = new anchor.BN(50 * 10 ** 9);

    // Call the transfer_tokens function of the program.
    const txSignature = await program.methods
      .transferTokens(transferAmount)
      .accounts({
        owner: payer.publicKey,
        mint: mintAccount,
        destination: recipientKeypair.publicKey,
      })
      .rpc();

    console.log('Transfer transaction signature:', txSignature);

    // Fetch the balances of the associated token accounts.
    const senderAccountInfo = await provider.connection.getTokenAccountBalance(senderTokenAccount);
    const recipientAccountInfo = await provider.connection.getTokenAccountBalance(recipientTokenAccount);

    console.log('Sender account info:', senderAccountInfo.value.uiAmount);

    // Verify that the transfer occurred correctly.
    assert.equal(senderAccountInfo.value.uiAmount, 50, 'Sender should have 50 tokens left');
    assert.equal(recipientAccountInfo.value.uiAmount, 50, 'Recipient should have received 50 tokens');
  });
});
