import {
    Connection,
    Keypair,
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


describe("Create a system account", async () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');

    class Assignable {
        constructor(properties) {
            Object.keys(properties).map((key) => {
                return (this[key] = properties[key]);
            });
        };
    };
    
    class AddressData extends Assignable {
        toBuffer() {
            return Buffer.from(borsh.serialize(AddressDataSchema, this));
        }
    };
    
    const AddressDataSchema = new Map([
        [
            AddressData, {
                kind: 'struct',
                fields: [
                    ['name', 'string'],
                    ['address', 'string'],
                ]
            }
        ]
    ]);
  
    it("Create the account", async () => {

        const newKeypair = Keypair.generate();

        const addressData = new AddressData({
            name: "Marcus",
            address: "123 Main St. San Francisco, CA"
        });

        const addressDataBuffer = addressData.toBuffer();
        console.log(`Address data buffer length: ${addressDataBuffer.length}`)

        let ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: newKeypair.publicKey, isSigner: true, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: program.publicKey,
            data: addressDataBuffer,
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, newKeypair]
        );
    });
  });
  