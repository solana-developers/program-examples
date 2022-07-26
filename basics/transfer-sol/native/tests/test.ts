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
import * as buffer_layout from "buffer-layout";


function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};


describe("transfer-sol", () => {

    async function getBalances(payerPubkey: PublicKey, recipientPubkey: PublicKey, timeframe: string) {
        let payerBalance = await connection.getBalance(payerPubkey);
        let recipientBalance = await connection.getBalance(recipientPubkey);
        console.log(`${timeframe} balances:`);
        console.log(`   Payer: ${payerBalance}`);
        console.log(`   Recipient: ${recipientBalance}`);
    };

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');
  
    it("Transfer some SOL", async () => {

        let recipientKeypair = Keypair.generate();
        let transferAmount = 1 * LAMPORTS_PER_SOL;

        await getBalances(payer.publicKey, recipientKeypair.publicKey, "Beginning");

        let data = Buffer.alloc(8) // 8 bytes
        buffer_layout.ns64("value").encode(transferAmount, data);

        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: recipientKeypair.publicKey, isSigner: false, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: program.publicKey,
            data: data,
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        );

        await getBalances(payer.publicKey, recipientKeypair.publicKey, "Resulting");
    });
  });
  