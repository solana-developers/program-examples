import {
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
} from '@solana/web3.js';
import { createTransferInstruction, InstructionType } from './instruction';


function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};


describe("transfer-sol", () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');

    const transferAmount = 1 * LAMPORTS_PER_SOL;
    const test1Recipient = Keypair.generate();
    const test2Recipient1 = Keypair.generate();
    const test2Recipient2 = Keypair.generate();
  
    it("Transfer between accounts using the system program", async () => {

        await getBalances(payer.publicKey, test1Recipient.publicKey, "Beginning");

        let ix = createTransferInstruction(
            payer.publicKey,
            test1Recipient.publicKey,
            program.publicKey,
            InstructionType.CpiTransfer,
            transferAmount
        );

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        );

        await getBalances(payer.publicKey, test1Recipient.publicKey, "Resulting");
    });

    it("Create two accounts for the following test", async () => {

        const ix = (pubkey: PublicKey) => {
          return SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: pubkey,
            space: 0,
            lamports: 2 * LAMPORTS_PER_SOL,
            programId: program.publicKey,
          })
        };
    
        await sendAndConfirmTransaction(
          connection,
          new Transaction()
            .add(ix(test2Recipient1.publicKey))
            .add(ix(test2Recipient2.publicKey))
          ,
          [payer, test2Recipient1, test2Recipient2]
        );
      });

    it("Transfer between accounts using our program", async () => {

        await getBalances(test2Recipient1.publicKey, test2Recipient2.publicKey, "Beginning");

        let ix = createTransferInstruction(
            test2Recipient1.publicKey,
            test2Recipient2.publicKey,
            program.publicKey,
            InstructionType.ProgramTransfer,
            transferAmount
        );

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, test2Recipient1]
        );

        await getBalances(test2Recipient1.publicKey, test2Recipient2.publicKey, "Resulting");
    });

    async function getBalances(payerPubkey: PublicKey, recipientPubkey: PublicKey, timeframe: string) {
        let payerBalance = await connection.getBalance(payerPubkey);
        let recipientBalance = await connection.getBalance(recipientPubkey);
        console.log(`${timeframe} balances:`);
        console.log(`   Payer: ${payerBalance}`);
        console.log(`   Recipient: ${recipientBalance}`);
    };
  });
  