import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
    LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
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
                ['mint_authority_pda_bump', 'u8'],
            ]
        }
    ]
]);

class MintTokensTo extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(MintTokensToSchema, this));
    }
};
const MintTokensToSchema = new Map([
    [
        MintTokensTo, {
            kind: 'struct',
            fields: [
                ['amount', 'u64'],
                ['mint_authority_pda_bump', 'u8'],
            ]
        }
    ]
]);

class TransferTokensTo extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(TransferTokensToSchema, this));
    }
};
const TransferTokensToSchema = new Map([
    [
        TransferTokensTo, {
            kind: 'struct',
            fields: [
                ['amount', 'u64'],
            ]
        }
    ]
]);


describe("mint-token", async () => {

    const connection = new Connection(`http://api.devnet.solana.com/`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');

    const mintKeypair: Keypair = Keypair.generate();
    console.log(`New token: ${mintKeypair.publicKey}`);

    it("Mint!", async () => {

        const [mintAuthorityPda, mintAuthorityPdaBump] = await PublicKey.findProgramAddress(
            [
              Buffer.from("mint_authority_"),
              mintKeypair.publicKey.toBuffer(),
            ],
            program.publicKey,
        );

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
            uri: "https://raw.githubusercontent.com/solana-developers/program-examples/main/tokens/mint-2/native/tests/token_metadata.json",
            mint_authority_pda_bump: mintAuthorityPdaBump
        });

        let ix = new TransactionInstruction({
            keys: [
                { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true },            // Mint account
                { pubkey: mintAuthorityPda, isSigner: false, isWritable: true },                // Mint authority account
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

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, mintKeypair]
        );
    });


    it("Mint to a wallet!", async () => {

        const [mintAuthorityPda, mintAuthorityPdaBump] = await PublicKey.findProgramAddress(
            [
              Buffer.from("mint_authority_"),
              mintKeypair.publicKey.toBuffer(),
            ],
            program.publicKey,
        );

        const tokenAddress = await getAssociatedTokenAddress(
            mintKeypair.publicKey,
            payer.publicKey
        );
        console.log(`Token Address: ${tokenAddress}`);

        const mintToInstructionData = new MintTokensTo({
            amount: 1,
            mint_authority_pda_bump: mintAuthorityPdaBump,
        });

        let ix = new TransactionInstruction({
            keys: [
                { pubkey: mintKeypair.publicKey, isSigner: false, isWritable: true },           // Mint account
                { pubkey: mintAuthorityPda, isSigner: false, isWritable: false },               // Mint authority account
                { pubkey: tokenAddress, isSigner: false, isWritable: true },                    // Token account
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },                  // Payer
                { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },             // Rent account
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },        // System program
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },               // Token program
                { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },    // Associated token program
            ],
            programId: program.publicKey,
            data: mintToInstructionData.toBuffer(),
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        );
    });

    it("Transfer to a wallet!", async () => {

        const recipientWallet = Keypair.generate();
        await connection.confirmTransaction(
            await connection.requestAirdrop(recipientWallet.publicKey, 2 * LAMPORTS_PER_SOL)
        );
        console.log(`Recipient Pubkey: ${recipientWallet.publicKey}`);

        const ownerTokenAddress = await getAssociatedTokenAddress(
            mintKeypair.publicKey,
            payer.publicKey
        );
        console.log(`Owner Token Address: ${ownerTokenAddress}`);
        const recipientTokenAddress = await getAssociatedTokenAddress(
            mintKeypair.publicKey,
            recipientWallet.publicKey
        );
        console.log(`Recipient Token Address: ${recipientTokenAddress}`);

        const transferToInstructionData = new TransferTokensTo({
            amount: 1,
        });

        let ix = new TransactionInstruction({
            keys: [
                { pubkey: mintKeypair.publicKey, isSigner: false, isWritable: true },           // Mint account
                { pubkey: ownerTokenAddress, isSigner: false, isWritable: true },               // Owner Token account
                { pubkey: recipientTokenAddress, isSigner: false, isWritable: true },           // Recipient Token account
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },                  // Owner
                { pubkey: recipientWallet.publicKey, isSigner: true, isWritable: true },        // Recipient
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },        // System program
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },               // Token program
                { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },    // Associated token program
            ],
            programId: program.publicKey,
            data: transferToInstructionData.toBuffer(),
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, recipientWallet]
        );
    });
  });
  