import * as anchor from '@coral-xyz/anchor';
import type { HelloSolana } from '../target/types/hello_solana';

describe('hello_solana', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.HelloSolana as anchor.Program<HelloSolana>;
  const payer = provider.wallet as anchor.Wallet;

  it('Say hello!', async () => {
    // Just run Anchor's IDL method to build a transaction
    // and sign it via a signer.
    await program.methods
      .hello()
      .accounts({
        signer: provider.wallet.publicKey,
      })
      .signers([payer.payer])
      .rpc();
  });
});
