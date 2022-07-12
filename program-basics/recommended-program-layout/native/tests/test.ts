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


describe("Carnival", () => {

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
    
    class CarnivalInstruction extends Assignable {
        toBuffer() {
            return Buffer.from(borsh.serialize(CarnivalInstructionSchema, this));
        }
    };
    
    const CarnivalInstructionSchema = new Map([
        [
            CarnivalInstruction, {
                kind: 'struct',
                fields: [
                    ['name', 'string'],
                    ['height', 'u32'],
                    ['ticket_count', 'u32'],
                    ['attraction', 'string'],
                    ['attraction_name', 'string'],
                ]
            }
        ]
    ]);

    async function sendCarnivalInstructions(instructionsList: CarnivalInstruction[]) {
        let tx = new Transaction();
        for (var ix of instructionsList) {
            tx.add(new TransactionInstruction({
                keys: [
                    {pubkey: payer.publicKey, isSigner: true, isWritable: true}
                ],
                programId: program.publicKey,
                data: ix.toBuffer(),
            }));
        };
        await sendAndConfirmTransaction(
            connection, 
            tx,
            [payer]
        );
    }
  
    it("Go on some rides!", async () => {

        await sendCarnivalInstructions([
            new CarnivalInstruction({
                name: "Jimmy",
                height: 36,
                ticket_count: 15,
                attraction: "ride",
                attraction_name: "Scrambler",
            }),
            new CarnivalInstruction({
                name: "Mary",
                height: 52,
                ticket_count: 1,
                attraction: "ride",
                attraction_name: "Ferris Wheel",
            }),
            new CarnivalInstruction({
                name: "Alice",
                height: 56,
                ticket_count: 15,
                attraction: "ride",
                attraction_name: "Scrambler",
            }),
            new CarnivalInstruction({
                name: "Bob",
                height: 49,
                ticket_count: 6,
                attraction: "ride",
                attraction_name: "Tilt-a-Whirl",
            }),
        ]);
    });


    it("Play some games!", async () => {

        await sendCarnivalInstructions([
            new CarnivalInstruction({
                name: "Jimmy",
                height: 36,
                ticket_count: 15,
                attraction: "game",
                attraction_name: "I Got It!",
            }),
            new CarnivalInstruction({
                name: "Mary",
                height: 52,
                ticket_count: 1,
                attraction: "game",
                attraction_name: "Ring Toss",
            }),
            new CarnivalInstruction({
                name: "Alice",
                height: 56,
                ticket_count: 15,
                attraction: "game",
                attraction_name: "Ladder Climb",
            }),
            new CarnivalInstruction({
                name: "Bob",
                height: 49,
                ticket_count: 6,
                attraction: "game",
                attraction_name: "Ring Toss",
            }),
        ]);
    });


    it("Eat some food!", async () => {

        await sendCarnivalInstructions([
            new CarnivalInstruction({
                name: "Jimmy",
                height: 36,
                ticket_count: 15,
                attraction: "food",
                attraction_name: "Taco Shack",
            }),
            new CarnivalInstruction({
                name: "Mary",
                height: 52,
                ticket_count: 1,
                attraction: "food",
                attraction_name: "Larry's Pizza",
            }),
            new CarnivalInstruction({
                name: "Alice",
                height: 56,
                ticket_count: 15,
                attraction: "food",
                attraction_name: "Dough Boy's",
            }),
            new CarnivalInstruction({
                name: "Bob",
                height: 49,
                ticket_count: 6,
                attraction: "food",
                attraction_name: "Dough Boy's",
            }),
        ]);
    });
  });
  