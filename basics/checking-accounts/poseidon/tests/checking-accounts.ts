import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { BN } from 'bn.js';
import { CheckingAccounts } from '../target/types/checking_accounts';

describe('checking-accounts', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CheckingAccounts as Program<CheckingAccounts>;

  // Generate new user keypairs for testing
  const user = Keypair.generate();

  let userAccount: PublicKey;
  let userAccountBump: number;

  before(async () => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    // Airdrop 1 SOL to the user
    const airdropUser = await provider.connection.requestAirdrop(user.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropUser,
    });

    // Derive PDA for the user account
    [userAccount, userAccountBump] = await PublicKey.findProgramAddressSync([Buffer.from('program')], program.programId);
  });

  it('Initialize User Account', async () => {
    // Initialize user account instruction invoked from the program
    await program.methods
      .initialize(new BN(1))
      .accountsPartial({
        payer: user.publicKey, // User's publickey
        userAccount, // PDA of the user account
      })
      .signers([user])
      .rpc();
  });

  it('Updates the User Account data', async () => {
    // Update user account instruction invoked from the program
    await program.methods
      .update(new BN(2))
      .accountsPartial({
        authority: user.publicKey, // Authority of the user account
        userAccount, // PDA of the user account
      })
      .signers([user])
      .rpc();
  });
});
