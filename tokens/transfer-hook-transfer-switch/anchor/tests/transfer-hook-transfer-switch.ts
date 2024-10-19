import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import { TransferHookTransferSwitch } from '../target/types/transfer_hook_transfer_switch';

describe('transfer-hook-transfer-switch', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TransferHookTransferSwitch as Program<TransferHookTransferSwitch>;

  const transferSwitchKeypair = Keypair.generate();

  it('Initializes the transfer switch', async () => {
    await program.methods
      .initialize()
      .accounts({
        transferSwitch: transferSwitchKeypair.publicKey,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([transferSwitchKeypair])
      .rpc();

    const transferSwitch = await program.account.transferSwitch.fetch(transferSwitchKeypair.publicKey);
    expect(transferSwitch.isEnabled).to.be.true;
    expect(transferSwitch.authority.toString()).to.equal(provider.wallet.publicKey.toString());
  });

  it('Toggles the transfer switch', async () => {
    await program.methods
      .toggleTransfer(false)
      .accounts({
        transferSwitch: transferSwitchKeypair.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    let transferSwitch = await program.account.transferSwitch.fetch(transferSwitchKeypair.publicKey);
    expect(transferSwitch.isEnabled).to.be.false;

    await program.methods
      .toggleTransfer(true)
      .accounts({
        transferSwitch: transferSwitchKeypair.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    transferSwitch = await program.account.transferSwitch.fetch(transferSwitchKeypair.publicKey);
    expect(transferSwitch.isEnabled).to.be.true;
  });

  it('Allows transfer when enabled', async () => {
    await program.methods
      .transferHook(new anchor.BN(100))
      .accounts({
        transferSwitch: transferSwitchKeypair.publicKey,
        from: provider.wallet.publicKey,
        to: Keypair.generate().publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();
  });

  it('Prevents transfer when disabled', async () => {
    await program.methods
      .toggleTransfer(false)
      .accounts({
        transferSwitch: transferSwitchKeypair.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    try {
      await program.methods
        .transferHook(new anchor.BN(100))
        .accounts({
          transferSwitch: transferSwitchKeypair.publicKey,
          from: provider.wallet.publicKey,
          to: Keypair.generate().publicKey,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .rpc();
      expect.fail('The transaction should have failed');
    } catch (error) {
      expect(error.message).to.include('Transfers are currently disabled');
    }
  });
});
