import {
    Connection,
    Keypair,
    sendAndConfirmTransaction,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';

import { assert } from "chai";

function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};


describe("hello-solana", () => {

    // Loading these from local files for development
    //
    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/hello_solana_program-keypair.json');
    it("Say hello!", async () => {

        // We set up our instruction first.
        //
        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true}
            ],
            programId: program.publicKey,
            data: Buffer.alloc(0), // No data
        });

        // Now we send the transaction over RPC
        //
        let signature = await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix), // Add our instruction (you can add more than one)
            [payer]
        );

        let transaction = await connection.getTransaction(signature, {commitment: "confirmed"});
        assert(transaction?.meta?.logMessages[0].startsWith("Program " + program.publicKey));
        assert(transaction?.meta?.logMessages[1] === "Program log: Hello, Solana!");
        assert(transaction?.meta?.logMessages[2] === "Program log: Our program's Program ID: " + program.publicKey);
    });
  });
