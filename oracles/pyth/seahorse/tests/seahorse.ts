import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import type { Seahorse } from '../target/types/seahorse';

describe('seahorse', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Seahorse as Program<Seahorse>;

  const PYTH_PRICE_ACCOUNT = new anchor.web3.PublicKey('H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG');

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods
      .getPythPrice()
      .accounts({
        pythPriceAccount: PYTH_PRICE_ACCOUNT,
      })
      .rpc();
    console.log('Your transaction signature', tx);
  });
});
