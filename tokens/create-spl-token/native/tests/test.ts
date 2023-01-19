import { 
    PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID
} from '@metaplex-foundation/mpl-token-metadata';
import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import {
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import * as borsh from "borsh";
import { Buffer } from "buffer";


function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};


class Assignable {
    constructor(properties) {
        Object.keys(properties).map((key) => {
            return (this[key] = properties[key]);
        });
    };
};

class CreateTokenArgs extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(CreateTokenArgsSchema, this));
    }
};
const CreateTokenArgsSchema = new Map([
    [
        CreateTokenArgs, {
            kind: 'struct',
            fields: [
                ['token_title', 'string'],
                ['token_symbol', 'string'],
                ['token_uri', 'string'],
            ]
        }
    ]
]);


describe("Create an SPL Token!", async () => {

    // const connection = new Connection(`https://api.devnet.solana.com/`, 'confirmed');
    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');

    const mintKeypair: Keypair = Keypair.generate();
    console.log(`New token: ${mintKeypair.publicKey}`);

    it("Create!", async () => {

        const metadataAddress = (PublicKey.findProgramAddressSync(
            [
              Buffer.from("metadata"),
              TOKEN_METADATA_PROGRAM_ID.toBuffer(),
              mintKeypair.publicKey.toBuffer(),
            ],
            TOKEN_METADATA_PROGRAM_ID
        ))[0];
        
        const metadataInstructionData = new CreateTokenArgs({
            token_title: "Solana Gold",
            token_symbol: "GOLDSOL",
            token_uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
        });

        let ix = new TransactionInstruction({
            keys: [
                { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true },            // Mint account
                { pubkey: payer.publicKey, isSigner: false, isWritable: true },                 // Mint authority account
                { pubkey: metadataAddress, isSigner: false, isWritable: true },                 // Metadata account
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },                  // Payer
                { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },             // Rent account
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },        // System program
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },               // Token program
                { pubkey: TOKEN_METADATA_PROGRAM_ID, isSigner: false, isWritable: false },      // Token metadata program
            ],
            programId: program.publicKey,
            data: metadataInstructionData.toBuffer(),
        });

        const sx = await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, mintKeypair]
        );

        console.log("Success!");
        console.log(`   Mint Address: ${mintKeypair.publicKey}`);
        console.log(`   Tx Signature: ${sx}`);
    });
  });
  