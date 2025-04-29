import { sign } from '@noble/ed25519';
import { Connection, Keypair, PublicKey, SystemProgram, TransactionInstruction, TransactionMessage, VersionedTransaction } from '@solana/web3.js';

export class Ed25519CustodialProgram {
  constructor(
    private connection: Connection,
    private programId: PublicKey,
  ) {}

  async createTransferInstruction(
    custodialAccount: PublicKey,
    recipient: PublicKey,
    signer: Keypair,
    amount: number,
  ): Promise<TransactionInstruction> {
    // Create message to sign
    const message = Buffer.from(`Transfer ${amount} lamports to ${recipient.toBase58()}`);

    // Sign the message with Ed25519
    const signature = await sign(message, signer.secretKey.slice(0, 32));

    // Create instruction data
    const data = Buffer.concat([
      Buffer.from(signature),
      Buffer.from(signer.publicKey.toBytes()),
      Buffer.from(new Uint8Array(new BigUint64Array([BigInt(amount)]).buffer)),
      message,
    ]);

    return new TransactionInstruction({
      keys: [
        { pubkey: custodialAccount, isSigner: false, isWritable: true },
        { pubkey: recipient, isSigner: false, isWritable: true },
        { pubkey: signer.publicKey, isSigner: true, isWritable: false },
        {
          pubkey: new PublicKey('Ed25519SigVerify111111111111111111111111111'),
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: this.programId,
      data,
    });
  }

  async transfer(custodialAccount: PublicKey, recipient: PublicKey, signer: Keypair, amount: number, payer: Keypair): Promise<string> {
    const instruction = await this.createTransferInstruction(custodialAccount, recipient, signer, amount);

    const latestBlockhash = await this.connection.getLatestBlockhash();

    const messageV0 = new TransactionMessage({
      payerKey: payer.publicKey,
      recentBlockhash: latestBlockhash.blockhash,
      instructions: [instruction],
    }).compileToV0Message();

    const transaction = new VersionedTransaction(messageV0);
    transaction.sign([payer, signer]);

    const signature = await this.connection.sendTransaction(transaction);
    await this.connection.confirmTransaction({
      signature,
      ...latestBlockhash,
    });

    return signature;
  }
}
