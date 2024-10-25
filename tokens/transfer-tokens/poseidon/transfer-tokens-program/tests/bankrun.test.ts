// import { describe, it, before } from 'node:test';
// import * as anchor from '@coral-xyz/anchor';
// import { PublicKey, Keypair } from '@solana/web3.js';
// import { BankrunProvider } from 'anchor-bankrun';
// import { assert } from 'chai';
// import { startAnchor } from 'solana-bankrun';
// import {
//   getAssociatedTokenAddressSync,
//   createMint,
//   mintTo,
//   getOrCreateAssociatedTokenAccount,
// } from '@solana/spl-token';
// import { TransferTokensProgram } from '../target/types/transfer_tokens_program';

// // Load the program ID and IDL from the target folder
// const IDL = require('../target/idl/transfer_tokens_program.json');
// const PROGRAM_ID = new PublicKey(IDL.address);

// describe('token_transfer_program', async () => {
//   // Initialize Bankrun context
//   const context = await startAnchor(
//     "",
//     [
//       { name: "transfer_tokens_program", programId: PROGRAM_ID },
//     ],
//     []
//   );
//   const provider = new BankrunProvider(context);
//   const program = new anchor.Program<TransferTokensProgram>(IDL, provider);
//   const connection = provider.connection;
//   const payer = provider.wallet as anchor.Wallet;

//   // Keypair for mint and recipient
//   const mintKeypair = Keypair.generate();
//   const recipientKeypair = Keypair.generate();

//   let mintAccount: PublicKey;
//   let senderTokenAccount: PublicKey;
//   let recipientTokenAccount: PublicKey;

//   const confirm = async (signature: string): Promise<string> => {
//     const block = await provider.connection.getLatestBlockhash();
//     await provider.connection.confirmTransaction({
//       signature,
//       ...block,
//     });
//     return signature;
//   };

//   it('Test Preparation', async () => {
//     console.log('Starting test preparation...');

//     // Create an SPL token mint
//     console.log('Creating mint account...');
//     mintAccount = await createMint(
//       connection,
//       payer.payer,
//       provider.publicKey,
//       null,
//       9 // Decimals
//     );
//     console.log('Mint account created:', mintAccount.toBase58());

//     // Get or create associated token accounts for sender and recipient
//     senderTokenAccount = getAssociatedTokenAddressSync(
//       mintAccount,
//       payer.publicKey
//     );
//     recipientTokenAccount = getAssociatedTokenAddressSync(
//       mintAccount,
//       recipientKeypair.publicKey
//     );

//     console.log('Sender Token Account PublicKey:', senderTokenAccount.toBase58());
//     console.log('Recipient Token Account PublicKey:', recipientTokenAccount.toBase58());

//     // Create associated token accounts for the sender and recipient
//     console.log('Creating associated token accounts...');
//     await getOrCreateAssociatedTokenAccount(
//       connection,
//       payer.payer,
//       mintAccount,
//       payer.publicKey
//     );
//     console.log('Sender associated token account created.');

//     await getOrCreateAssociatedTokenAccount(
//       connection,
//       payer.payer,
//       mintAccount,
//       recipientKeypair.publicKey
//     );
//     console.log('Recipient associated token account created.');

//     // Mint some tokens to the sender's associated token account
//     console.log('Minting tokens to sender\'s associated token account...');
//     await mintTo(
//       connection,
//       payer.payer,
//       mintAccount,
//       senderTokenAccount,
//       payer.payer,
//       100 * 10 ** 9 // 100 tokens
//     );
//     console.log('Minting complete. 100 tokens minted to:', senderTokenAccount.toBase58());
//   });

//   it('Transfers tokens between accounts using Bankrun', async () => {
//     console.log('Starting token transfer test...');

//     const transferAmount = new anchor.BN(50 * 10 ** 9);
//     console.log('Initiating token transfer of:', transferAmount.toString());

//     // Call the transfer_tokens function of the program
//     const txSignature = await program.methods
//       .transferTokens(transferAmount)
//       .accounts({
//         owner: payer.publicKey,
//         mint: mintAccount,
//         destination: recipientTokenAccount,
//       })
//       .rpc();

//     console.log('Transfer transaction signature:', txSignature);

//     // Fetch the balances of the associated token accounts
//     const senderAccountInfo = await connection.getTokenAccountBalance(
//       senderTokenAccount
//     );
//     const recipientAccountInfo = await connection.getTokenAccountBalance(
//       recipientTokenAccount
//     );

//     console.log('Sender account balance after transfer:', senderAccountInfo.value.uiAmount);
//     console.log('Recipient account balance after transfer:', recipientAccountInfo.value.uiAmount);

//     // Verify that the transfer occurred correctly
//     assert.equal(
//       senderAccountInfo.value.uiAmount,
//       50,
//       'Sender should have 50 tokens left'
//     );
//     assert.equal(
//       recipientAccountInfo.value.uiAmount,
//       50,
//       'Recipient should have received 50 tokens'
//     );

//     console.log('Token transfer test completed successfully.');
//   });
// });
