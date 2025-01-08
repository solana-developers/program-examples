import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import { HelloSolanaProgram } from '../target/types/hello_solana_program'; // Assuming this path

const IDL = require('../target/idl/hello_solana_program.json');
const PROGRAM_ID = new PublicKey(IDL.metadata.address);

describe('hello_solana_program (Bankrun)', async () => {
  const context = await startAnchor('', [{ name: 'hello_solana_program', programId: PROGRAM_ID }], []);

  const provider = new BankrunProvider(context);

  const program = new anchor.Program<HelloSolanaProgram>(IDL, PROGRAM_ID, provider);

  it("Executes 'hello' successfully", async () => {
    const tx = await program.methods.hello().rpc();

    // Chai assert to ensure no error occurred and transaction completed successfully
    assert.isOk(tx, 'Transaction should complete without errors');
  });
});
