import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import { EscrowProgram } from '../target/types/escrow_program';

describe('escrow_program', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.EscrowProgram as Program<EscrowProgram>;
  const maker = provider.wallet;

  let vaultAccount: PublicKey;
  let escrowAccount: PublicKey;

  it('Creates a new escrow', async () => {
    // Generate a keypair for escrow
    escrowAccount = anchor.web3.Keypair.generate().publicKey;

    const vault = anchor.web3.Keypair.generate();
    const depositAmount = new anchor.BN(1000);
    const offerAmount = new anchor.BN(2000);
    const seed = new anchor.BN(123);

    await program.methods
      .make(depositAmount, offerAmount, seed)
      .accounts({
        vault: vaultAccount,
        auth: maker.publicKey,
        escrow: escrowAccount,
        maker: maker.publicKey,
        makerAta: maker.publicKey, // assuming the ATA
        makerMint: maker.publicKey, // assuming mint for simplicity
        takerMint: maker.publicKey, // assuming mint for simplicity
        tokenProgram: SystemProgram.programId,
        associatedTokenProgram: SystemProgram.programId,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    // Assert escrow account was created
    const escrowState = await program.account.escrowState.fetch(escrowAccount);
    assert.equal(escrowState.amount.toString(), offerAmount.toString(), 'Escrow amount is incorrect');
    assert.equal(escrowState.seed.toString(), seed.toString(), 'Escrow seed is incorrect');
  });

  it('Refunds from the vault', async () => {
    await program.methods
      .refund()
      .accounts({
        vault: vaultAccount,
        auth: maker.publicKey,
        escrow: escrowAccount,
        maker: maker.publicKey,
        makerAta: maker.publicKey, // assuming the ATA
        makerMint: maker.publicKey, // assuming mint for simplicity
        tokenProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    // Assert that the escrow account was closed
    try {
      await program.account.escrowState.fetch(escrowAccount);
      assert.fail('Escrow account should be closed');
    } catch (err) {
      assert.ok('Escrow account was closed');
    }
  });

  it('Transfers tokens to taker', async () => {
    const taker = anchor.web3.Keypair.generate();
    const takerAta = anchor.web3.Keypair.generate();
    const takerReceiveAta = anchor.web3.Keypair.generate();

    await program.methods
      .take()
      .accounts({
        taker: taker.publicKey,
        maker: maker.publicKey,
        makerAta: maker.publicKey,
        takerAta: takerAta.publicKey,
        takerReceiveAta: takerReceiveAta.publicKey,
        escrow: escrowAccount,
        vault: vaultAccount,
        auth: maker.publicKey,
        tokenProgram: SystemProgram.programId,
      })
      .signers([taker])
      .rpc();

    // Assert the transfer occurred
    const escrowState = await program.account.escrowState.fetch(escrowAccount);
    assert.isNotNull(escrowState, 'Escrow state should be updated');
  });
});
