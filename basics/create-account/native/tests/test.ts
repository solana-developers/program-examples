import {
    Connection,
    Keypair,
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


describe("Create a system account", async () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    
    const PROGRAM_ID: PublicKey = new PublicKey(
        "Au21huMZuDQrbzu2Ec5ohpW5CKRqhcGV6qLawfydStGs"
    );
  
    it("Create the account", async () => {

        const newKeypair = Keypair.generate();

        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: newKeypair.publicKey, isSigner: true, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: PROGRAM_ID,
            data: Buffer.alloc(0),
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, newKeypair]
        );
    });
  });
  