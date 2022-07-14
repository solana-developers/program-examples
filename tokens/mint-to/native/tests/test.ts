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
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    MINT_SIZE,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import * as borsh from "borsh";
import { Buffer } from "buffer";


const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);


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

class TokenMetadata extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(TokenMetadataSchema, this));
    }
};
const TokenMetadataSchema = new Map([
    [
        TokenMetadata, {
            kind: 'struct',
            fields: [
                ['title', 'string'],
                ['symbol', 'string'],
                ['uri', 'string'],
            ]
        }
    ]
]);

class MintTokenTo extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(MintTokenToSchema, this));
    }
};
const MintTokenToSchema = new Map([
    [
        MintTokenTo, {
            kind: 'struct',
            fields: [
                ['amount', 'u64'],
            ]
        }
    ]
]);


describe("mint-token", () => {

    const connection = new Connection(`http://api.devnet.solana.com/`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');

    const mintKeypair: Keypair = Keypair.generate();
    console.log(`New token: ${mintKeypair.publicKey}`);

    it("Mint!", async () => {

        const metadataAddress = (await PublicKey.findProgramAddress(
            [
              Buffer.from("metadata"),
              TOKEN_METADATA_PROGRAM_ID.toBuffer(),
              mintKeypair.publicKey.toBuffer(),
            ],
            TOKEN_METADATA_PROGRAM_ID
        ))[0];
        const metadataInstructionData = new TokenMetadata({
            title: "Solana Gold",
            symbol: "GOLDSOL",
            uri: "https://raw.githubusercontent.com/solana-developers/program-examples/main/tokens/token_metadata.json",
        });

        // Transact with the "create_mint" function in our on-chain program
        //
        let ix = new TransactionInstruction({
            keys: [
                // Mint account
                {
                    pubkey: mintKeypair.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                // Metadata account
                {
                    pubkey: metadataAddress,
                    isSigner: false,
                    isWritable: true,
                },
                // Mint Authority
                {
                    pubkey: payer.publicKey,
                    isSigner: true,
                    isWritable: false,
                },
                // Rent account
                {
                    pubkey: SYSVAR_RENT_PUBKEY,
                    isSigner: false,
                    isWritable: false,
                },
                // System program
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false,
                },
                // Token program
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                // Token metadata program
                {
                    pubkey: TOKEN_METADATA_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: program.publicKey,
            data: metadataInstructionData.toBuffer(),
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, mintKeypair]
        );
    });


    it("Mint to a wallet!", async () => {

        const tokenAddress = await getAssociatedTokenAddress(
            mintKeypair.publicKey,
            payer.publicKey
        );
        const mintToInstructionData = new MintTokenTo({
            amount: 1,
        });

        // Transact with the "mint_to" function in our on-chain program
        //
        let ix = new TransactionInstruction({
            keys: [
                // Mint account
                {
                    pubkey: mintKeypair.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                // Token account
                {
                    pubkey: tokenAddress,
                    isSigner: false,
                    isWritable: true,
                },
                // Mint Authority
                {
                    pubkey: payer.publicKey,
                    isSigner: true,
                    isWritable: false,
                },
                // Rent account
                {
                    pubkey: SYSVAR_RENT_PUBKEY,
                    isSigner: false,
                    isWritable: false,
                },
                // System program
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false,
                },
                // Token program
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                // Associated token program
                {
                    pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: program.publicKey,
            data: mintToInstructionData.toBuffer(),
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, mintKeypair]
        );
    });
  });
  