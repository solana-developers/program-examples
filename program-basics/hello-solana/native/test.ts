import {
    Connection,
    Keypair,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';

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
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');
  
    it("Say hello!", async () => {

        // We set up our instruction first.
        //
        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            ],
            programId: program.publicKey,
            data: Buffer.alloc(0), // No data
        });

        // Now we send the transaction over RPC
        //
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix), // Add our instruction (you can add more than one)
            [payer]
        );
    });
  });
  