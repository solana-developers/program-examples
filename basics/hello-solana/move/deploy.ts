/** WIP */
import * as fs from 'fs';
import * as solanaWeb3 from '@solana/web3.js';

async function startValidator() {
  // TODO: Start a test-validator programmatically to have a self contained test.
  // solana-test-validator -l test-ledger
}

async function deployProgramShell(programPath: string) {
  const util = require('util');
  const exec = util.promisify(require('child_process').exec);
  try {
    const { stdout, stderr } = await exec(`solana program deploy ${programPath}`);
    console.log('stdout:', stdout);
    console.log('stderr:', stderr);
    return stdout;
  } catch (e) {
    console.error(e); // should contain code (exit code) and signal (that caused the termination).
    return e;
  }
}

// WIP: Function to deploy a Solana program programmatically.
async function deployProgram(
  connection: solanaWeb3.Connection,
  payer: solanaWeb3.Keypair,
  programKeypair: solanaWeb3.Keypair,
  programPath: string
): Promise<solanaWeb3.PublicKey> {
  // solana program deploy programPath
  // Load the program data
  const programData = fs.readFileSync(programPath);

  // Allocate space for the program data
  const transaction = new solanaWeb3.Transaction();
  const { feeCalculator } = await connection.getRecentBlockhash();
  const programSpace = solanaWeb3.BpfLoader.getMinimumBalanceForRentExemption(
    programData.length,
    feeCalculator
  );

  // Create the program account
  transaction.add(
    solanaWeb3.SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: programKeypair.publicKey,
      lamports: programSpace,
      space: programData.length,
      programId: solanaWeb3.BpfLoader.programId,
    })
  );

  // Load the program
  transaction.add(
    solanaWeb3.BpfLoader.load(
      connection,
      payer,
      programKeypair,
      programData,
      solanaWeb3.BpfLoader.programId
    )
  );

  // Send the transaction
  await solanaWeb3.sendAndConfirmTransaction(
    connection,
    transaction,
    [payer, programKeypair]
  );

  return programKeypair.publicKey;
}

async function main() {
  const connection = new solanaWeb3.Connection(solanaWeb3.clusterApiUrl('devnet'), 'confirmed');
  const payer = solanaWeb3.Keypair.generate();
  const programKeypair = solanaWeb3.Keypair.generate();
  const programPath = 'bin/hello_solana_move_program.so';
  const programId = await deployProgramShell(programPath);
  console.log('Program deployed with', programId);
}

main().catch(console.error);