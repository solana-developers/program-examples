import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { assert } from 'chai';
import { ReallocProgram } from '../target/types/realloc_program';

describe('realloc_program', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ReallocProgram as Program<ReallocProgram>;

  // Define account keypair and PDA
  const messageAccount = anchor.web3.Keypair.generate();
  let messagePDA: anchor.web3.PublicKey;
  let bump: number;

  before(async () => {
    // Derive the PDA using the seed [b"message"]
    [messagePDA, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from('message')], program.programId);
  });

  it('initialize the message account', async () => {
    // Define a message to store
    const initialMessage = 'Hello, Solana!';

    // Call the initialize instruction
    await program.methods
      .initialize(initialMessage)
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .signers([])
      .rpc();

    // Fetch the account to confirm the data
    const account = await program.account.messageAccountState.fetch(messagePDA);
    assert.equal(account.message, initialMessage, 'Message should be initialized correctly');
    assert.equal(account.bump, bump, 'Bump value should match');
  });

  it('update the message account', async () => {
    // Define a new message to update
    const updatedMessage = 'changed';

    // Call the update instruction
    await program.methods
      .update(updatedMessage)
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .signers([])
      .rpc();

    // Fetch the account to confirm the updated data
    const account = await program.account.messageAccountState.fetch(messagePDA);
    assert.equal(account.message, updatedMessage, 'Message should be updated correctly');
  });
});