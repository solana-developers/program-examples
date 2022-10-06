import {
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
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


describe("Change an account's owner", async () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    
    const PROGRAM_ID: PublicKey = new PublicKey(
        "Au21huMZuDQrbzu2Ec5ohpW5CKRqhcGV6qLawfydStGs"
    );

    const newKeypair = Keypair.generate();
  
    it("Create the account", async () => {

        let ix = SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: newKeypair.publicKey,
            lamports: 1 * LAMPORTS_PER_SOL,
            space: 0,
            programId: SystemProgram.programId,
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, newKeypair]
        );
    });

    it("Change ownership for the account", async () => {

        const newKeypair = Keypair.generate();

        let ix = SystemProgram.assign({
            accountPubkey: newKeypair.publicKey,
            programId: PROGRAM_ID
        })

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, newKeypair]
        );
    });

    it("Change it again using the System Program", async () => {

        const newKeypair = Keypair.generate();

        let ix = SystemProgram.assign({
            accountPubkey: newKeypair.publicKey,
            programId: SystemProgram.programId,
        })

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, newKeypair]
        );
    });
  });
  