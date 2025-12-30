import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { sign } from '@noble/ed25519';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { Ed25519Custodial } from '../target/types/ed25519_custodial';

describe('Ed25519 Custodial', async () => {
  const context = await startAnchor(
    '',
    [
      {
        name: 'ed25519_custodial',
        programId: new PublicKey('Ed25519CustodiaLXXXXXXXXXXXXXXXXXXXXXXXXXXX'),
      },
    ],
    [],
  );
  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);

  const program = anchor.workspace.Ed25519Custodial as Program<Ed25519Custodial>;

  it('Verifies signature and transfers funds', async () => {
    const custodialAccount = Keypair.generate();
    const recipient = Keypair.generate();
    const signerKeypair = Keypair.generate();
    const amount = 1000000; // lamports

    // Message to sign
    const message = Buffer.from(`Transfer ${amount} lamports to ${recipient.publicKey.toBase58()}`);

    // Sign the message with Ed25519
    const signature = await sign(message, signerKeypair.secretKey.slice(0, 32));

    try {
      await program.methods
        .transfer(Array.from(signature), Array.from(signerKeypair.publicKey.toBytes()), Array.from(message), new anchor.BN(amount))
        .accounts({
          custodialAccount: custodialAccount.publicKey,
          recipient: recipient.publicKey,
          signer: signerKeypair.publicKey,
          ed25519Program: new PublicKey('Ed25519SigVerify111111111111111111111111111'),
        })
        .signers([signerKeypair])
        .rpc();

      console.log('Transaction processed successfully');
    } catch (error) {
      console.error('Error:', error);
      throw error;
    }
  });
});
