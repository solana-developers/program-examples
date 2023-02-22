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


describe("PDA Rent-Payer", () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const PROGRAM_ID: PublicKey = createKeypairFromFile('./program/target/deploy/program-keypair.json').publicKey;

    class Assignable {
        constructor(properties) {
            Object.keys(properties).map((key) => {
                return (this[key] = properties[key]);
            });
        };
    };

    enum MyInstruction {
        InitRentVault,
        CreateNewAccount,
    }

    class InitRentVault extends Assignable {
        toBuffer() { return Buffer.from(borsh.serialize(InitRentVaultSchema, this)) }
    };
    const InitRentVaultSchema = new Map([
        [ InitRentVault, { 
            kind: 'struct', 
            fields: [ ['instruction', 'u8'], ['fund_lamports', 'u64'] ],
        }]
    ]);

    class CreateNewAccount extends Assignable {
        toBuffer() { return Buffer.from(borsh.serialize(CreateNewAccountSchema, this)) }
    };
    const CreateNewAccountSchema = new Map([
        [ CreateNewAccount, { 
            kind: 'struct', 
            fields: [ ['instruction', 'u8'] ],
        }]
    ]);

    function deriveRentVaultPda() {
        const pda = PublicKey.findProgramAddressSync(
            [Buffer.from("rent_vault")],
            PROGRAM_ID,
        )
        console.log(`PDA: ${pda[0].toBase58()}`)
        return pda
    }

    it("Initialize the Rent Vault", async () => {
        const [rentVaultPda, _] = deriveRentVaultPda();
        let ix = new TransactionInstruction({
            keys: [
                {pubkey: rentVaultPda, isSigner: false, isWritable: true},
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: PROGRAM_ID,
            data: (new InitRentVault({ instruction: MyInstruction.InitRentVault, fund_lamports: 1000000000 })).toBuffer(),
        });
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        );
    });

    it("Create a new account using the Rent Vault", async () => {
        const newAccount = Keypair.generate();
        const [rentVaultPda, _] = deriveRentVaultPda();
        let ix = new TransactionInstruction({
            keys: [
                {pubkey: newAccount.publicKey, isSigner: true, isWritable: true},
                {pubkey: rentVaultPda, isSigner: false, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId: PROGRAM_ID,
            data: new CreateNewAccount({ instruction: MyInstruction.CreateNewAccount }).toBuffer(),
        });
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, newAccount]
        );
    });
});
