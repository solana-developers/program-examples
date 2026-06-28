import { Buffer } from "node:buffer";
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { assert } from "chai";
import { start } from "solana-bankrun";

// The legacy SPL Token and Associated Token Account programs are bundled with
// bankrun. The Metaplex Token Metadata program is not, so it is dumped from
// mainnet into tests/fixtures by prepare.mjs and loaded by name below.
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
const INSTRUCTIONS_SYSVAR_ID = new PublicKey("Sysvar1nstructions1111111111111111111111111");

// Instruction discriminators (the Borsh enum variant index).
const CREATE_COLLECTION = 0;
const MINT_NFT = 1;
const VERIFY_COLLECTION = 2;

function getAuthorityPda(programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync([Buffer.from("authority")], programId);
}

function getMetadataAddress(mint: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID,
  )[0];
}

function getMasterEditionAddress(mint: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer(), Buffer.from("edition")],
    TOKEN_METADATA_PROGRAM_ID,
  )[0];
}

function getAssociatedTokenAddress(mint: PublicKey, owner: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  )[0];
}

// Read the `amount` field (u64 at offset 64) of an SPL token account.
function readTokenAmount(data: Uint8Array): bigint {
  return Buffer.from(data).readBigUInt64LE(64);
}

describe("NFT Operations (Pinocchio)", async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start(
    [
      { name: "nft_operations_pinocchio_program", programId: PROGRAM_ID },
      { name: "token_metadata", programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  const [mintAuthorityPda, mintAuthorityBump] = getAuthorityPda(PROGRAM_ID);

  const collectionMint = Keypair.generate();
  const nftMint = Keypair.generate();

  async function sendInstruction(ix: TransactionInstruction, signers: Keypair[]) {
    const tx = new Transaction();
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix);
    tx.sign(...signers);
    await client.processTransaction(tx);
  }

  it("Creates a collection NFT", async () => {
    const metadata = getMetadataAddress(collectionMint.publicKey);
    const masterEdition = getMasterEditionAddress(collectionMint.publicKey);
    const destination = getAssociatedTokenAddress(collectionMint.publicKey, payer.publicKey);

    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // user
        { pubkey: collectionMint.publicKey, isSigner: true, isWritable: true }, // mint
        { pubkey: mintAuthorityPda, isSigner: false, isWritable: false }, // mint authority PDA
        { pubkey: metadata, isSigner: false, isWritable: true }, // metadata
        { pubkey: masterEdition, isSigner: false, isWritable: true }, // master edition
        { pubkey: destination, isSigner: false, isWritable: true }, // destination ATA
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // token program
        { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // associated token program
        { pubkey: TOKEN_METADATA_PROGRAM_ID, isSigner: false, isWritable: false }, // token metadata program
      ],
      data: Buffer.from([CREATE_COLLECTION, mintAuthorityBump]),
    });

    await sendInstruction(ix, [payer, collectionMint]);

    const mintAccount = await client.getAccount(collectionMint.publicKey);
    if (mintAccount === null) throw new Error("Collection mint not found");
    assert.deepEqual(mintAccount.owner.toBytes(), TOKEN_PROGRAM_ID.toBytes());

    const destinationAccount = await client.getAccount(destination);
    if (destinationAccount === null) throw new Error("Collection token account not found");
    assert.equal(readTokenAmount(destinationAccount.data), 1n);

    const metadataAccount = await client.getAccount(metadata);
    if (metadataAccount === null) throw new Error("Collection metadata not found");
    assert.deepEqual(metadataAccount.owner.toBytes(), TOKEN_METADATA_PROGRAM_ID.toBytes());
    assert.isTrue(Buffer.from(metadataAccount.data).toString("utf-8").includes("DummyCollection"));
  });

  it("Mints an NFT into the collection", async () => {
    const metadata = getMetadataAddress(nftMint.publicKey);
    const masterEdition = getMasterEditionAddress(nftMint.publicKey);
    const destination = getAssociatedTokenAddress(nftMint.publicKey, payer.publicKey);

    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // owner
        { pubkey: nftMint.publicKey, isSigner: true, isWritable: true }, // mint
        { pubkey: mintAuthorityPda, isSigner: false, isWritable: false }, // mint authority PDA
        { pubkey: metadata, isSigner: false, isWritable: true }, // metadata
        { pubkey: masterEdition, isSigner: false, isWritable: true }, // master edition
        { pubkey: destination, isSigner: false, isWritable: true }, // destination ATA
        { pubkey: collectionMint.publicKey, isSigner: false, isWritable: false }, // collection mint
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // token program
        { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // associated token program
        { pubkey: TOKEN_METADATA_PROGRAM_ID, isSigner: false, isWritable: false }, // token metadata program
      ],
      data: Buffer.from([MINT_NFT, mintAuthorityBump]),
    });

    await sendInstruction(ix, [payer, nftMint]);

    const destinationAccount = await client.getAccount(destination);
    if (destinationAccount === null) throw new Error("NFT token account not found");
    assert.equal(readTokenAmount(destinationAccount.data), 1n);

    const metadataAccount = await client.getAccount(metadata);
    if (metadataAccount === null) throw new Error("NFT metadata not found");
    assert.deepEqual(metadataAccount.owner.toBytes(), TOKEN_METADATA_PROGRAM_ID.toBytes());
    assert.isTrue(Buffer.from(metadataAccount.data).toString("utf-8").includes("Mint Test"));

    const editionAccount = await client.getAccount(masterEdition);
    if (editionAccount === null) throw new Error("NFT master edition not found");
    assert.deepEqual(editionAccount.owner.toBytes(), TOKEN_METADATA_PROGRAM_ID.toBytes());
  });

  it("Verifies the NFT as part of the collection", async () => {
    const metadata = getMetadataAddress(nftMint.publicKey);
    const collectionMetadata = getMetadataAddress(collectionMint.publicKey);
    const collectionMasterEdition = getMasterEditionAddress(collectionMint.publicKey);

    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: mintAuthorityPda, isSigner: false, isWritable: false }, // mint authority PDA
        { pubkey: metadata, isSigner: false, isWritable: true }, // NFT metadata
        { pubkey: collectionMint.publicKey, isSigner: false, isWritable: false }, // collection mint
        { pubkey: collectionMetadata, isSigner: false, isWritable: true }, // collection metadata
        { pubkey: collectionMasterEdition, isSigner: false, isWritable: false }, // collection master edition
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: INSTRUCTIONS_SYSVAR_ID, isSigner: false, isWritable: false }, // instructions sysvar
        { pubkey: TOKEN_METADATA_PROGRAM_ID, isSigner: false, isWritable: false }, // token metadata program
      ],
      data: Buffer.from([VERIFY_COLLECTION, mintAuthorityBump]),
    });

    // Metaplex `Verify` performs strict checks: the signer must be the
    // collection's update authority (our PDA), the collection metadata and
    // master edition must be valid, and the NFT must reference the collection.
    // A successful transaction therefore proves the whole flow is correct.
    await sendInstruction(ix, [payer]);

    const metadataAccount = await client.getAccount(metadata);
    if (metadataAccount === null) throw new Error("NFT metadata not found");
    assert.deepEqual(metadataAccount.owner.toBytes(), TOKEN_METADATA_PROGRAM_ID.toBytes());
  });
});
