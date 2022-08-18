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


describe("Checking accounts", async () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    
    const PROGRAM_ID: PublicKey = new PublicKey(
        ""
    );

    // We'll create this ahead of time.
    // Our program will try to modify it.
    const accountToChange = Keypair.generate();
    // Our program will create this.
    const accountToCreate = Keypair.generate();
  
    it("Create an account owned by our program", async () => {

        let ix = SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: accountToChange.publicKey,
            lamports: await connection.getMinimumBalanceForRentExemption(0),
            space: 0,
            programId: PROGRAM_ID, // Our program
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, accountToChange]
        );
    });
    
    it("Check accounts", async () => {

        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: accountToCreate.publicKey, isSigner: true, isWritable: true},
                {pubkey: accountToChange.publicKey, isSigner: true, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: PROGRAM_ID,
            data: Buffer.alloc(0),
        }); 

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, accountToCreate, accountToChange]
        );
    });
  });
  