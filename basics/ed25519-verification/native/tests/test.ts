import { Buffer } from 'node:buffer';
import { describe, it } from 'node:test';
import { sign } from '@noble/ed25519';
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { start } from 'solana-bankrun';

describe('Ed25519 Custodial Program', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'ed25519_custodial', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  it('should verify signature and transfer funds', async () => {
    const custodialAccount = Keypair.generate();
    const recipient = Keypair.generate();
    const signerKeypair = Keypair.generate();
    const amount = 1000000; // lamports

    // Message to sign
    const message = Buffer.from(`Transfer ${amount} lamports to ${recipient.publicKey.toBase58()}`);

    // Sign the message with Ed25519
    const signature = await sign(message, signerKeypair.secretKey.slice(0, 32));

    // Create instruction data
    const instructionData = Buffer.concat([
      Buffer.from(signature),
      Buffer.from(signerKeypair.publicKey.toBytes()),
      Buffer.from(new Uint8Array(new BigUint64Array([BigInt(amount)]).buffer)),
      message,
    ]);

    const instruction = new TransactionInstruction({
      keys: [
        {
          pubkey: custodialAccount.publicKey,
          isSigner: false,
          isWritable: true,
        },
        { pubkey: recipient.publicKey, isSigner: false, isWritable: true },
        { pubkey: signerKeypair.publicKey, isSigner: true, isWritable: false },
        {
          pubkey: new PublicKey('Ed25519SigVerify111111111111111111111111111'),
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: PROGRAM_ID,
      data: instructionData,
    });

    const transaction = new Transaction().add(instruction);

    try {
      await client.processTransaction(transaction);
      console.log('Transaction processed successfully');
    } catch (error) {
      console.error('Error:', error);
      throw error;
    }
  });
});
