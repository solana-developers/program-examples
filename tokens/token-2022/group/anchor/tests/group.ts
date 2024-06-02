import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import type { Group } from '../target/types/group';

describe('group', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.Group as Program<Group>;

  it('Create Mint with Group Pointer', async () => {
    const transactionSignature = await program.methods.testInitializeGroup().accounts({}).rpc({ skipPreflight: true });
    console.log('Your transaction signature', transactionSignature);
  });
});
