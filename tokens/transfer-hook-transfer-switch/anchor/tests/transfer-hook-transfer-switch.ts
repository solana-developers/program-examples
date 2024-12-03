import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { TransferHook } from '../target/types/transfer_hook';

// Load program configuration
const IDL = require('../target/idl/transfer_hook.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Transfer Hook Program', async () => {
  // Initialize program context
  const context = await startAnchor('.', [], []);
  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const authority = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<TransferHook>(IDL, provider);

  // Generate keypairs
  const mintKeypair = new Keypair();
  const recipient = new Keypair();

  // Derive PDAs and token accounts
  const [stateAddress] = PublicKey.findProgramAddressSync([Buffer.from('state')], PROGRAM_ID);

  const [walletStateAddress] = PublicKey.findProgramAddressSync(
    [Buffer.from('wallet_state'), authority.publicKey.toBuffer(), mintKeypair.publicKey.toBuffer()],
    PROGRAM_ID,
  );

  // Derive token accounts
  const authorityTokenAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, authority.publicKey);

  const recipientTokenAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, recipient.publicKey);

  // Constants
  const TRANSFER_AMOUNT = new anchor.BN(100_000_000);

  it('Initialize program state', async () => {
    const transactionSignature = await program.methods
      .initialize()
      .accounts({
        state: stateAddress,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority.payer])
      .rpc();

    console.log('Program state initialized successfully');
    console.log(`   State Address: ${stateAddress}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it('Create token mint', async () => {
    const transactionSignature = await program.methods
      .createMint()
      .accounts({
        mint: mintKeypair.publicKey,
        authority: authority.publicKey,
        state: stateAddress,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair])
      .rpc();

    console.log('Token mint created successfully');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it('Set wallet state to frozen', async () => {
    const transactionSignature = await program.methods
      .setWalletState(true)
      .accounts({
        walletState: walletStateAddress,
        wallet: authority.publicKey,
        mint: mintKeypair.publicKey,
        authority: authority.publicKey,
        state: stateAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log('Wallet state set to frozen');
    console.log(`   Wallet State Address: ${walletStateAddress}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);

    const walletState = await program.account.walletState.fetch(walletStateAddress);
    console.log(`   Frozen Status: ${walletState.isFrozen}`);
  });

  it('Should fail to transfer tokens when wallet is frozen', async () => {
    try {
      await program.methods
        .transfer(TRANSFER_AMOUNT)
        .accounts({
          sourceAuthority: authority.publicKey,
          destinationAuthority: recipient.publicKey,
          walletState: walletStateAddress,
          mint: mintKeypair.publicKey,
          sourceToken: authorityTokenAddress,
          destinationToken: recipientTokenAddress,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
    } catch (error) {
      console.log('Transfer failed as expected (wallet frozen)');
      return;
    }
    throw new Error('Transfer should have failed');
  });

  it('Transfer tokens after unfreezing wallet', async () => {
    // First unfreeze the wallet
    await program.methods
      .setWalletState(false)
      .accounts({
        walletState: walletStateAddress,
        wallet: authority.publicKey,
        mint: mintKeypair.publicKey,
        authority: authority.publicKey,
        state: stateAddress,
      })
      .rpc();

    console.log('Wallet unfrozen successfully');

    // Then transfer tokens
    const transactionSignature = await program.methods
      .transfer(TRANSFER_AMOUNT)
      .accounts({
        sourceAuthority: authority.publicKey,
        destinationAuthority: recipient.publicKey,
        walletState: walletStateAddress,
        mint: mintKeypair.publicKey,
        sourceToken: authorityTokenAddress,
        destinationToken: recipientTokenAddress,
      })
      .rpc();

    console.log('Transfer completed successfully');
    console.log(`   Amount: ${TRANSFER_AMOUNT.toString()}`);
    console.log(`   From: ${authorityTokenAddress}`);
    console.log(`   To: ${recipientTokenAddress}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
