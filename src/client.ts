import { Buffer } from 'node:buffer';
import { sign } from '@noble/ed25519';
import { Connection, Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction, sendAndConfirmTransaction } from '@solana/web3.js';

export async function createCustodialTransfer(
  connection: Connection,
  payer: Keypair,
  programId: PublicKey,
  custodialAccount: PublicKey,
  recipient: PublicKey,
  amount: number,
  signerKeypair: Keypair,
) {
  // Message to sign
  const message = Buffer.from(`Transfer ${amount} lamports to ${recipient.toBase58()}`);

  // Sign the message with Ed25519
  const signatureBytes = await sign(message, signerKeypair.secretKey.slice(0, 32));

  // Create instruction data
  const instructionData = Buffer.concat([Buffer.from(signatureBytes), Buffer.from(signerKeypair.publicKey.toBytes()), message]);

  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: custodialAccount, isSigner: false, isWritable: true },
      { pubkey: recipient, isSigner: false, isWritable: true },
      { pubkey: signerKeypair.publicKey, isSigner: true, isWritable: false },
      {
        pubkey: new PublicKey('Ed25519SigVerify111111111111111111111111111'),
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: instructionData,
  });

  const transaction = new Transaction().add(instruction);

  const txSignature = await sendAndConfirmTransaction(connection, transaction, [payer, signerKeypair]);

  return txSignature;
}

// Example usage
async function main() {
  const connection = new Connection('http://localhost:8899', 'confirmed');
  const payer = Keypair.generate();
  const programId = new PublicKey('Your_Program_ID');
  const custodialAccount = new PublicKey('Custodial_Account_Address');
  const recipient = new PublicKey('Recipient_Address');
  const amount = 1000000; // lamports
  const signerKeypair = Keypair.generate();

  try {
    const signature = await createCustodialTransfer(connection, payer, programId, custodialAccount, recipient, amount, signerKeypair);
    console.log('Transaction signature:', signature);
  } catch (error) {
    console.error('Error:', error);
  }
}
