import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CreateTokens } from '../target/types/create_tokens';

describe('create_tokens', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CreateTokens as Program<CreateTokens>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log('Your transaction signature', tx);
  });
});
