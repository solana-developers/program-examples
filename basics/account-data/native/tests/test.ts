import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';
import * as borsh from "borsh";
import { Buffer } from "buffer";


function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};


describe("Account Data!", () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const PROGRAM_ID: PublicKey = new PublicKey(
        "BCw7MQWBugruuYgno5crGUGFNufqGJbPpzZevhRRRQAu"
    );

    class Assignable {
        constructor(properties) {
            Object.keys(properties).map((key) => {
                return (this[key] = properties[key]);
            });
        };
    };

    class AddressInfo extends Assignable {
        toBuffer() { return Buffer.from(borsh.serialize(AddressInfoSchema, this)) }
        
        static fromBuffer(buffer: Buffer) {
            return borsh.deserialize(AddressInfoSchema, AddressInfo, buffer);
        };
    };
    const AddressInfoSchema = new Map([
        [ AddressInfo, { 
            kind: 'struct', 
            fields: [ 
                ['name', 'string'], 
                ['house_number', 'u8'], 
                ['street', 'string'], 
                ['city', 'string'], 
            ],
        }]
    ]);

    const addressInfoAccount = Keypair.generate();

    it("Create the address info account", async () => {
        console.log(`Payer Address      : ${payer.publicKey}`);
        console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);
        let ix = new TransactionInstruction({
            keys: [
                {pubkey: addressInfoAccount.publicKey, isSigner: true, isWritable: true},
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: PROGRAM_ID,
            data: (
                new AddressInfo({
                    name: "Joe C",
                    house_number: 136,
                    street: "Mile High Dr.",
                    city: "Solana Beach",
                })
            ).toBuffer(),
        });
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, addressInfoAccount]
        );
    });

    it("Read the new account's data", async () => {
        const accountInfo = await connection.getAccountInfo(addressInfoAccount.publicKey);
        const readAddressInfo = AddressInfo.fromBuffer(accountInfo.data);
        console.log(`Name     : ${readAddressInfo.name}`);
        console.log(`House Num: ${readAddressInfo.house_number}`);
        console.log(`Street   : ${readAddressInfo.street}`);
        console.log(`City     : ${readAddressInfo.city}`);
    });
});
