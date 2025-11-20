import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, SystemProgram, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { assert } from 'chai';

describe('cpi_example', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CpiExample as Program<any>;
  const payer = provider.wallet as anchor.Wallet;

  it('Initialize the CPI example', async () => {
    const cpiExampleKeypair = new Keypair();

    try {
      await program.methods
        .initialize()
        .accounts({
          cpiExample: cpiExampleKeypair.publicKey,
          authority: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([cpiExampleKeypair])
        .rpc();

      console.log('✅ CPI Example initialized successfully!');
    } catch (error) {
      console.log('❌ Initialize failed:', error);
      throw error;
    }
  });

  it('Transfer SOL via CPI', async () => {
    const cpiExampleKeypair = new Keypair();
    const fromAccountKeypair = new Keypair();
    const toAccountKeypair = new Keypair();

    // Initialize the CPI example first
    await program.methods
      .initialize()
      .accounts({
        cpiExample: cpiExampleKeypair.publicKey,
        authority: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([cpiExampleKeypair])
      .rpc();

    // Fund the from account
    await provider.connection.requestAirdrop(fromAccountKeypair.publicKey, LAMPORTS_PER_SOL);

    const transferAmount = 0.1 * LAMPORTS_PER_SOL; // 0.1 SOL

    try {
      await program.methods
        .transferSolViaCpi(new anchor.BN(transferAmount))
        .accounts({
          cpiExample: cpiExampleKeypair.publicKey,
          fromAccount: fromAccountKeypair.publicKey,
          toAccount: toAccountKeypair.publicKey,
          authority: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([fromAccountKeypair, toAccountKeypair])
        .rpc();

      console.log('✅ SOL transfer via CPI successful!');
    } catch (error) {
      console.log('❌ SOL transfer failed:', error);
      throw error;
    }
  });

  it('Verify program deployment', async () => {
    const programId = program.programId;
    const accountInfo = await provider.connection.getAccountInfo(programId);
    
    assert(accountInfo !== null, 'Program should be deployed');
    assert(accountInfo.executable, 'Program should be executable');
    
    console.log('✅ Program is properly deployed and executable');
  });
});
