import {
    Connection,
    Keypair, LAMPORTS_PER_SOL,
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


describe("Create a system account", async () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');
  
    it("Create the account via a cross program invocation", async () => {

        const newKeypair = Keypair.generate();

        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: newKeypair.publicKey, isSigner: true, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: program.publicKey,
            data: Buffer.alloc(0),
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, newKeypair]
        );
    });

    it("Create the account via direct call to system program", async () => {

        const newKeypair = Keypair.generate();

        const ix = SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: newKeypair.publicKey,
            lamports: LAMPORTS_PER_SOL,
            space: 0,
            programId: SystemProgram.programId
        })


        await sendAndConfirmTransaction(connection,
            new Transaction().add(ix),
            [payer, newKeypair]);
    });
  });
  