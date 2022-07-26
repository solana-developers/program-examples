import {
    Connection,
    Keypair,
    sendAndConfirmTransaction,
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


describe("custom-instruction-data", () => {

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

    class InstructionData extends Assignable {
        toBuffer() {
            return Buffer.from(borsh.serialize(InstructionDataSchema, this));
        }
    };

    const InstructionDataSchema = new Map([
        [
            InstructionData, {
                kind: 'struct',
                fields: [
                    ['name', 'string'],
                    ['height', 'u32'],
                ]
            }
        ]
    ]);
  
    it("Go to the park!", async () => {

        const jimmy = new InstructionData({
            name: "Jimmy",
            height: 3
        });

        const mary = new InstructionData({
            name: "Mary",
            height: 10
        });

        let ix1 = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true}
            ],
            programId: program.publicKey,
            data: jimmy.toBuffer(),
        });

        let ix2 = new TransactionInstruction({
            ...ix1,
            data: mary.toBuffer(),
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix1).add(ix2),
            [payer]
        );
    });
  });
  