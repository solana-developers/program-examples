import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
} from '@solana/web3.js';
import {
    describe,
    it,
} from 'mocha';
import {
    createCreateUserInstruction,
    createCloseUserInstruction,
    createKeypairFromFile,
} from '../ts';



describe("Close Account!", async () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/deploy/program-keypair.json');

    const testAccountPublicKey = PublicKey.findProgramAddressSync(
        [Buffer.from("USER"), payer.publicKey.toBuffer()],
        program.publicKey,
    )[0];

    it("Create the account", async () => {
        console.log(`${testAccountPublicKey}`);
        const ix = createCreateUserInstruction(
            testAccountPublicKey,
            payer.publicKey, 
            program.publicKey,
            "Jacob",
        );
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        );
    });
    
    it("Close the account", async () => {
        const ix = createCloseUserInstruction(
            testAccountPublicKey,
            payer.publicKey, 
            program.publicKey,
        );
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer],
            { skipPreflight: true }
        );
    });
  });
  