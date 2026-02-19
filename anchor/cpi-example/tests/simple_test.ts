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

  it('Verify program deployment', async () => {
    const programId = program.programId;
    const accountInfo = await provider.connection.getAccountInfo(programId);
    
    assert(accountInfo !== null, 'Program should be deployed');
    assert(accountInfo.executable, 'Program should be executable');
    
    console.log('✅ Program is properly deployed and executable');
    console.log('Program ID:', programId.toBase58());
  });

  it('Initialize the CPI example', async () => {
    const cpiExampleKeypair = new Keypair();

    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          cpiExample: cpiExampleKeypair.publicKey,
          authority: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([cpiExampleKeypair])
        .rpc();

      console.log('✅ CPI Example initialized successfully!');
      console.log('Transaction signature:', tx);
    } catch (error) {
      console.log('❌ Initialize failed:', error);
      throw error;
    }
  });
});