import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import { FavoritesProgram } from '../target/types/favorites_program';

describe('favorites_program', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.FavoritesProgram as Program<FavoritesProgram>;

  // Account to hold the favorite state.
  const [favoriteStatePda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("favorites")], 
    program.programId
  );

  it('Adds a favorite', async () => {
    // Call add method
    await program.methods.add()
      .accounts({
        state: favoriteStatePda,  // Use generated PDA for 'state'
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    // Fetch the account state again
    const state = await program.account.favoriteState.fetch(favoriteStatePda);

    // Check that the favorite count is incremented by 1
    assert.strictEqual(state.favorite.toNumber(), 1, "favorite count should be 1 after add");
  });

  it('Removes a favorite', async () => {

    // Call remove method
    await program.methods.remove()
      .accounts({
        state: favoriteStatePda,  // Use generated PDA for 'state'
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    // Fetch the account state again
    const state = await program.account.favoriteState.fetch(favoriteStatePda);

    // Check that the favorite count is decremented by 1
    assert.strictEqual(state.favorite.toNumber(), 0, "favorite count should be 0 after remove");
  });
});