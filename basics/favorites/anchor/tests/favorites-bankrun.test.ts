import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { getCustomErrorMessage } from '@solana-developers/helpers';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { Favorites } from '../target/types/favorites';
import { systemProgramErrors } from './system-errors';

const web3 = anchor.web3;
const IDL = require('../target/idl/favorites.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Favorites Bankrun', async () => {
  // Use the cluster and the keypair from Anchor.toml
  // Load programs into anchor-bankrun
  const context = await startAnchor('', [{ name: 'favorites', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  const someRandomGuy = anchor.web3.Keypair.generate();

  const program = new anchor.Program<Favorites>(IDL, provider);

  // Here's what we want to write to the blockchain
  const favoriteNumber = new anchor.BN(23);
  const favoriteColor = 'purple';
  const favoriteHobbies = ['skiing', 'skydiving', 'biking'];

  // We don't need to airdrop if we're using the local cluster
  // because the local cluster gives us 1,000,000 SOL
  const balance = await context.banksClient.getBalance(user.publicKey);
  const balanceInSOL = balance / BigInt(web3.LAMPORTS_PER_SOL);
  const formattedBalance = new Intl.NumberFormat().format(balanceInSOL);
  console.log(`Balance: ${formattedBalance} SOL`);

  it('Writes our favorites to the blockchain', async () => {
    await program.methods
      // set_favourites in Rust becomes setFavorites in TypeScript
      .setFavorites(favoriteNumber, favoriteColor, favoriteHobbies)
      // Sign the transaction
      .signers([user])
      // Send the transaction to the cluster or RPC
      .rpc();

    // Find the PDA for the user's favorites
    const favoritesPdaAndBump = web3.PublicKey.findProgramAddressSync([Buffer.from('favorites'), user.publicKey.toBuffer()], program.programId);
    const favoritesPda = favoritesPdaAndBump[0];
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    // And make sure it matches!
    assert.equal(dataFromPda.color, favoriteColor);
    // A little extra work to make sure the BNs are equal
    assert.equal(dataFromPda.number.toString(), favoriteNumber.toString());
    // And check the hobbies too
    assert.deepEqual(dataFromPda.hobbies, favoriteHobbies);
  });

  it('Updates the favorites', async () => {
    const newFavoriteHobbies = ['skiing', 'skydiving', 'biking', 'swimming'];
    try {
      await program.methods.setFavorites(favoriteNumber, favoriteColor, newFavoriteHobbies).signers([user]).rpc();
    } catch (error) {
      console.error((error as Error).message);
      const customErrorMessage = getCustomErrorMessage(systemProgramErrors, error);
      throw new Error(customErrorMessage);
    }
  });

  it('Rejects transactions from unauthorized signers', async () => {
    try {
      await program.methods
        // set_favourites in Rust becomes setFavorites in TypeScript
        .setFavorites(favoriteNumber, favoriteColor, favoriteHobbies)
        // Sign the transaction
        .signers([someRandomGuy])
        // Send the transaction to the cluster or RPC
        .rpc();
    } catch (error) {
      const errorMessage = (error as Error).message;
      assert.isTrue(errorMessage.includes('unknown signer'));
    }
  });
});
