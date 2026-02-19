import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { assert } from 'chai';

describe('cpi_example_basic', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  // Generate keypairs for accounts
  const cpiExampleKeypair = new Keypair();
  const fromSolAccountKeypair = new Keypair();
  const toSolAccountKeypair = new Keypair();

  it('Initialize the CPI example', async () => {
    // Get the program ID from the built program
    const programId = new PublicKey("A6reKAfewwif4GxzqpYTr1CLMKp2mwytKaQrjWdPCsBi");
    
    // Create a simple program instance with basic IDL structure
    const program = new anchor.Program({
      "address": "A6reKAfewwif4GxzqpYTr1CLMKp2mwytKaQrjWdPCsBi",
      "metadata": {
        "name": "cpi_example",
        "version": "0.1.0",
        "spec": "0.1.0",
        "description": "CPI Example Program"
      },
      "instructions": [
        {
          "name": "initialize",
          "accounts": [
            { "name": "cpiExample", "isMut": true, "isSigner": true },
            { "name": "authority", "isMut": true, "isSigner": true },
            { "name": "systemProgram", "isMut": false, "isSigner": false }
          ],
          "args": []
        },
        {
          "name": "transferSolViaCpi",
          "accounts": [
            { "name": "cpiExample", "isMut": true, "isSigner": false },
            { "name": "fromAccount", "isMut": true, "isSigner": true },
            { "name": "toAccount", "isMut": true, "isSigner": false },
            { "name": "authority", "isMut": false, "isSigner": true },
            { "name": "systemProgram", "isMut": false, "isSigner": false }
          ],
          "args": [
            { "name": "amount", "type": "u64" }
          ]
        }
      ],
      "accounts": [
        {
          "name": "CpiExample",
          "type": {
            "kind": "struct",
            "fields": [
              { "name": "authority", "type": "publicKey" },
              { "name": "totalCpiCalls", "type": "u64" }
            ]
          }
        }
      ]
    }, programId, provider);

    // Initialize the CPI example program
    await program.methods
      .initialize()
      .accounts({
        cpiExample: cpiExampleKeypair.publicKey,
        authority: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([cpiExampleKeypair])
      .rpc();

    console.log("✅ CPI Example initialized successfully!");
  });

  it('Transfer SOL via CPI', async () => {
    const programId = new PublicKey("A6reKAfewwif4GxzqpYTr1CLMKp2mwytKaQrjWdPCsBi");
    
    const program = new anchor.Program({
      "address": "A6reKAfewwif4GxzqpYTr1CLMKp2mwytKaQrjWdPCsBi",
      "metadata": {
        "name": "cpi_example",
        "version": "0.1.0",
        "spec": "0.1.0",
        "description": "CPI Example Program"
      },
      "instructions": [
        {
          "name": "transferSolViaCpi",
          "accounts": [
            { "name": "cpiExample", "isMut": true, "isSigner": false },
            { "name": "fromAccount", "isMut": true, "isSigner": true },
            { "name": "toAccount", "isMut": true, "isSigner": false },
            { "name": "authority", "isMut": false, "isSigner": true },
            { "name": "systemProgram", "isMut": false, "isSigner": false }
          ],
          "args": [
            { "name": "amount", "type": "u64" }
          ]
        }
      ],
      "accounts": [
        {
          "name": "CpiExample",
          "type": {
            "kind": "struct",
            "fields": [
              { "name": "authority", "type": "publicKey" },
              { "name": "totalCpiCalls", "type": "u64" }
            ]
          }
        }
      ]
    }, programId, provider);
    
    const transferAmount = new anchor.BN(0.1 * LAMPORTS_PER_SOL); // 0.1 SOL

    // Fund the from account
    const fundTx = await provider.connection.requestAirdrop(fromSolAccountKeypair.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(fundTx);

    // Get initial balances
    const fromAccountBefore = await provider.connection.getBalance(fromSolAccountKeypair.publicKey);
    const toAccountBefore = await provider.connection.getBalance(toSolAccountKeypair.publicKey);

    // Call the transfer_sol_via_cpi function
    await program.methods
      .transferSolViaCpi(transferAmount)
      .accounts({
        cpiExample: cpiExampleKeypair.publicKey,
        fromAccount: fromSolAccountKeypair.publicKey,
        toAccount: toSolAccountKeypair.publicKey,
        authority: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([fromSolAccountKeypair])
      .rpc();

    // Verify the SOL transfer
    const fromAccountAfter = await provider.connection.getBalance(fromSolAccountKeypair.publicKey);
    const toAccountAfter = await provider.connection.getBalance(toSolAccountKeypair.publicKey);

    // Account for transaction fees
    const expectedFromBalance = fromAccountBefore - transferAmount.toNumber();
    const expectedToBalance = toAccountBefore + transferAmount.toNumber();

    assert(
      Math.abs(fromAccountAfter - expectedFromBalance) < 10000, // Allow for small fee differences
      'From account should have approximately the expected balance'
    );
    assert(
      toAccountAfter === expectedToBalance,
      'To account should have the exact expected balance'
    );

    console.log("✅ SOL transfer via CPI completed successfully!");
    console.log(`From account balance: ${fromAccountAfter} lamports`);
    console.log(`To account balance: ${toAccountAfter} lamports`);
  });

  it('Verify program deployment', async () => {
    const programId = new PublicKey("A6reKAfewwif4GxzqpYTr1CLMKp2mwytKaQrjWdPCsBi");
    
    // Check if the program is deployed
    const programInfo = await provider.connection.getAccountInfo(programId);
    
    assert(programInfo !== null, 'Program should be deployed');
    assert(programInfo.executable === true, 'Program should be executable');
    
    console.log("✅ Program is properly deployed and executable");
  });
});
