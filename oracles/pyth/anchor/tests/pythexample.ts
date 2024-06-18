import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import type { Pythexample } from '../target/types/pythexample';

describe('pythexample', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Pythexample as Program<Pythexample>;

  const PYTH_FEED_ID = new anchor.web3.PublicKey('H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG');

  it('Check SOL_USD Price', async () => {
    const tx = await program.methods
      .readPrice()
      .accounts({
        priceFeed: PYTH_FEED_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();

    console.log('Your transaction signature, find the price in the program logs', tx);
  });
});
