import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { ProcessingInstructions } from '../target/types/processing_instructions';

describe('processing-instructions', () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Generate new user keypairs for testing
  const user = Keypair.generate(); // First user
  const user2 = Keypair.generate(); // Second user

  const program = anchor.workspace.ProcessingInstructions as Program<ProcessingInstructions>;

  // Variables for storing Program Derived Address (PDA) and bump
  let userAccountPDA: PublicKey; // PDA for user account
  let userAccountBump: number;

  // Before all tests, airdrop SOL to users for transaction fees
  before(async () => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    // Airdrop 1 SOL to the first user
    const airdropUser = await provider.connection.requestAirdrop(user.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropUser,
    });

    // Airdrop 1 SOL to the second user
    const airdropUser2 = await provider.connection.requestAirdrop(user2.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropUser2,
    });
  });

  it('Process instruction!', async () => {
    // Derive PDA for the first user account
    [userAccountPDA, userAccountBump] = PublicKey.findProgramAddressSync([user.publicKey.toBuffer()], program.programId);

    // User information for the instruction
    const userInfo = {
      name: 'Jimmy',
      height: 3,
    };

    // Invoke GoToPark Instruction from the program
    await program.methods
      .goToPark(userInfo.name, userInfo.height)
      .accountsPartial({
        payer: user.publicKey, // Payer of the transaction
        user: userAccountPDA, // User account PDA
      })
      .signers([user]) // Signer of the transaction
      .rpc();

    // Fetch the user account information after processing
    const userAccountInfo = await program.account.userAccount.fetch(userAccountPDA);

    // Log a welcome message
    console.log('\nWelcome to the park,', userAccountInfo.name);

    // Check if the user is tall enough for a ride
    if (userAccountInfo.height > 5) {
      console.log('You are tall enough to ride this ride. Congratulations!');
    } else {
      console.log('You are NOT tall enough to ride this ride. Sorry mate.');
    }
  });

  it('Process instruction again!', async () => {
    // Derive PDA for the second user account
    [userAccountPDA, userAccountBump] = PublicKey.findProgramAddressSync([user2.publicKey.toBuffer()], program.programId);

    // User information for the instruction
    const userInfo = {
      name: 'Mary',
      height: 10,
    };

    // Invoke GoToPark Instruction from the program
    await program.methods
      .goToPark(userInfo.name, userInfo.height)
      .accountsPartial({
        payer: user2.publicKey, // Payer of the transaction
        user: userAccountPDA, // User account PDA
      })
      .signers([user2]) // Signer of the transaction
      .rpc();

    // Fetch the user account information after processing
    const userAccountInfo = await program.account.userAccount.fetch(userAccountPDA);

    // Log a welcome message
    console.log('\nWelcome to the park,', userAccountInfo.name);

    // Check if the user is tall enough for a ride
    if (userAccountInfo.height > 5) {
      console.log('You are tall enough to ride this ride. Congratulations!');
    } else {
      console.log('You are NOT tall enough to ride this ride. Sorry mate.');
    }
  });
});
