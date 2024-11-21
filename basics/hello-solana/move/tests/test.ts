import { describe, test } from 'node:test';
import { assert } from 'chai';
import {deployProgramShell, parseProgramID} from './deploy';

import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';

function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};

function getInstructionData(path: string): Buffer {
  // instruction_data (See: sui/external-crates/move/solana/move-mv-llvm-compiler/docs/Entrypoint.md)
  let j = JSON.parse(require('fs').readFileSync(path, "utf-8"));
  return j['instruction_data'];
}

async function deployProgram(programPath: string) : Promise<string> {
  const programIdLog = await deployProgramShell(programPath);
  const programId = await parseProgramID(programIdLog);
  if (programId) {
    console.log('Program deployed with', programId);
    return programId;
  }
  console.log('Program could not be deployed');
  return null;
}

describe("hello-solana", async () => {
    // Loading these from local files for development
    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    // PublicKey of the deployed program.
    const programPath = 'bin/hello_solana_move_program.so';
    const programIdStr = await deployProgram(programPath);
    const programId = new PublicKey(programIdStr);
    const instructionData = getInstructionData('bin/input.json');

    it("Say hello!", async () => {
        // Set up transaction instructions first.
        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true}
            ],
            programId,
            data: instructionData,
        });

        // Send the transaction over RPC
        let signature = await sendAndConfirmTransaction(
            connection,
            new Transaction().add(ix), // Add our instruction (you can add more than one)
            [payer]
        );

        let transaction = await connection.getTransaction(signature, {commitment: "confirmed"});
        console.log(transaction);
        assert(transaction?.meta?.logMessages[0].startsWith(`Program ${programId}`));
        // 'Hello Solana' as bytes
        assert(transaction?.meta?.logMessages[1] === 'Program log: 0000000000000000000000000000000000000000000000000000000000000001::string::String { bytes: [72, 101, 108, 108, 111, 32, 83, 111, 108, 97, 110, 97], }');
        assert(transaction?.meta?.logMessages[2] === `Program ${programId} consumed 5331 of 200000 compute units`);
        assert(transaction?.meta?.logMessages[3] === `Program ${programId} success`);
    });
  });