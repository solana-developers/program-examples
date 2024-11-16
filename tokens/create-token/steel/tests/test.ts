import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Metadata, PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { MintLayout, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { start } from 'solana-bankrun';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

class CreateTokenInstructionData extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(CreateTokenInstructionDataSchema, this));
  }
}
const CreateTokenInstructionDataSchema = new Map([
  [
    CreateTokenInstructionData,
    {
      kind: 'struct',
      fields: [
        ['discriminator', 'u8'], // add the instructiion discriminator
        ['token_title', 'string'],
        ['token_symbol', 'string'],
        ['token_uri', 'string'],
        ['token_decimals', 'u8'],
      ],
    },
  ],
]);

describe('Create Tokens!', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start(
    [
      { name: 'create_token_steel_program', programId: PROGRAM_ID },
      { name: 'token_metadata', programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  test('Create an SPL Token!', async () => {
    const mintKeypair: Keypair = Keypair.generate();

    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // SPL Token default = 9 decimals
    //
    const tokenDetails = {
      token_title: 'Solana Gold',
      token_symbol: 'GOLDSOL',
      token_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
      token_decimals: 9,
    };

    const instructionData = new CreateTokenInstructionData({
      ...tokenDetails,
      discriminator: 0,
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true }, // Mint account
        { pubkey: payer.publicKey, isSigner: false, isWritable: true }, // Mint authority account
        { pubkey: metadataAddress, isSigner: false, isWritable: true }, // Metadata account
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Payer
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // Rent account
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Token metadata program
      ],
      programId: PROGRAM_ID,
      data: Buffer.concat([instructionData.toBuffer()]),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, mintKeypair);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);

    const metadataInfo = await client.getAccount(metadataAddress);
    assert(metadataInfo !== null, 'metadata account not created');

    const [metadata] = Metadata.fromAccountInfo({
      ...metadataInfo,
      data: Buffer.from(metadataInfo.data),
    });

    assert(metadata.data.name.slice(0, tokenDetails.token_title.length) === tokenDetails.token_title, 'name does not match');
    assert(metadata.data.symbol.slice(0, tokenDetails.token_symbol.length) === tokenDetails.token_symbol, 'symbol does not match');
    assert(metadata.data.uri.slice(0, tokenDetails.token_uri.length) === tokenDetails.token_uri, 'uri does not match');
    assert(metadata.mint.toBase58() === mintKeypair.publicKey.toBase58(), 'mint does not match');

    const mintInfo = await client.getAccount(mintKeypair.publicKey);
    const mint = MintLayout.decode(mintInfo.data);
    assert(mint.mintAuthority.toBase58() === payer.publicKey.toBase58(), 'mint authority does not match');
    assert(mint.decimals === 9, 'mint decimals does not match');
  });

  test('Create an NFT!', async () => {
    const mintKeypair: Keypair = Keypair.generate();

    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // NFT default = 0 decimals
    //
    const nftDetails = {
      token_title: 'Homer NFT',
      token_symbol: 'HOMR',
      token_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
      token_decimals: 0,
    };

    const instructionData = new CreateTokenInstructionData({
      ...nftDetails,
      discriminator: 0,
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true }, // Mint account
        { pubkey: payer.publicKey, isSigner: false, isWritable: true }, // Mint authority account
        { pubkey: metadataAddress, isSigner: false, isWritable: true }, // Metadata account
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Payer
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // Rent account
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Token metadata program
      ],
      programId: PROGRAM_ID,
      data: instructionData.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, mintKeypair);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);

    const metadataInfo = await client.getAccount(metadataAddress);
    assert(metadataInfo !== null, 'metadata account not created');

    const [metadata] = Metadata.fromAccountInfo({
      ...metadataInfo,
      data: Buffer.from(metadataInfo.data),
    });

    assert(metadata.data.name.slice(0, nftDetails.token_title.length) === nftDetails.token_title, 'name does not match');
    assert(metadata.data.symbol.slice(0, nftDetails.token_symbol.length) === nftDetails.token_symbol, 'symbol does not match');
    assert(metadata.data.uri.slice(0, nftDetails.token_uri.length) === nftDetails.token_uri, 'uri does not match');
    assert(metadata.mint.toBase58() === mintKeypair.publicKey.toBase58(), 'mint does not match');

    const mintInfo = await client.getAccount(mintKeypair.publicKey);
    const mint = MintLayout.decode(mintInfo.data);
    assert(mint.mintAuthority.toBase58() === payer.publicKey.toBase58(), 'mint authority does not match');
    assert(mint.decimals === 0, 'mint decimals does not match');
  });
});
