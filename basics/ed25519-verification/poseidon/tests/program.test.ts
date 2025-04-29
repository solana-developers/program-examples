import { describe, it } from 'node:test';
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { Ed25519CustodialProgram } from '../src/program';

describe('Ed25519 Custodial Program', () => {
  const connection = new Connection('http://localhost:8899', 'confirmed');
  const programId = new PublicKey('your_program_id_here');
  const program = new Ed25519CustodialProgram(connection, programId);

  it('should create transfer instruction', async () => {
    const custodialAccount = Keypair.generate();
    const recipient = Keypair.generate();
    const signer = Keypair.generate();
    const amount = LAMPORTS_PER_SOL;

    const instruction = await program.createTransferInstruction(custodialAccount.publicKey, recipient.publicKey, signer, amount);

    // Verify instruction structure
    expect(instruction.programId).toEqual(programId);
    expect(instruction.keys).toHaveLength(4);
    expect(instruction.data).toBeDefined();
  });

  it('should execute transfer', async () => {
    const custodialAccount = Keypair.generate();
    const recipient = Keypair.generate();
    const signer = Keypair.generate();
    const payer = Keypair.generate();
    const amount = LAMPORTS_PER_SOL;

    // Fund accounts for testing
    await connection.requestAirdrop(custodialAccount.publicKey, 2 * LAMPORTS_PER_SOL);
    await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);

    const signature = await program.transfer(custodialAccount.publicKey, recipient.publicKey, signer, amount, payer);

    expect(signature).toBeDefined();

    // Verify balances
    const recipientBalance = await connection.getBalance(recipient.publicKey);
    expect(recipientBalance).toEqual(amount);
  });
});
