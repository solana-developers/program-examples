import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Metadata, PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { ASSOCIATED_TOKEN_PROGRAM_ID, AccountLayout, MintLayout, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { start } from 'solana-bankrun';
import { CreateTokenArgs, InitArgs, MintToArgs, NftMinterInstruction } from './instructions';

class MintAuthorityPda {
  discriminator: number;
  bump: number;

  constructor(data: { discriminator: number; bump: number }) {
    this.discriminator = data.discriminator;
    this.bump = data.bump;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(MintAuthorityPdaSchema, this));
  }

  static fromAccountInfo(buffer: Uint8Array) {
    return borsh.deserialize(MintAuthorityPdaSchema, MintAuthorityPda, Buffer.from(buffer));
  }
}
const MintAuthorityPdaSchema = new Map([
  [
    MintAuthorityPda,
    {
      kind: 'struct',
      fields: [
        ['discriminator', 'u64'],
        ['bump', 'u8'],
      ],
    },
  ],
]);

describe('PDA Mint Authority!', async () => {
  // your program ID
  //
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

  const context = await start(
    [
      { name: 'pda_mint_authority_steel_program', programId: PROGRAM_ID },
      { name: 'token_metadata', programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  const nftMintKeypair: Keypair = Keypair.generate();

  const [mintAuthorityPublicKey, bump] = PublicKey.findProgramAddressSync([Buffer.from('mint_authority')], PROGRAM_ID);

  test('Init Mint Authority PDA', async () => {
    const instructionData = new InitArgs({
      instruction: NftMinterInstruction.Init,
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: mintAuthorityPublicKey, isSigner: false, isWritable: true },
        { pubkey: payer.publicKey, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: instructionData.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Authority Address: ${mintAuthorityPublicKey}`);

    const pdaMintAuthorityInfo = await client.getAccount(mintAuthorityPublicKey);
    assert(pdaMintAuthorityInfo !== null, 'mint authority pda not created');
    const pdaMintAuthority = MintAuthorityPda.fromAccountInfo(pdaMintAuthorityInfo.data);
    assert(pdaMintAuthority.bump === bump, 'mint authority pda bump does not match');
  });

  test('Create an NFT!', async () => {
    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), nftMintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    const nftData = {
      nft_title: 'Homer NFT',
      nft_symbol: 'HOMR',
      nft_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
    };

    // NFT default = 0 decimals
    //
    const instructionData = new CreateTokenArgs({
      instruction: NftMinterInstruction.CreateToken,
      nft_title: Buffer.from(nftData.nft_title.padEnd(32, '\0')),
      nft_symbol: Buffer.from(nftData.nft_symbol.padEnd(10, '\0')),
      nft_uri: Buffer.from(nftData.nft_uri.padEnd(256, '\0')),
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: nftMintKeypair.publicKey, isSigner: true, isWritable: true }, // Mint account
        { pubkey: mintAuthorityPublicKey, isSigner: false, isWritable: true }, // Mint authority account
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
    tx.add(ix).sign(payer, nftMintKeypair);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${nftMintKeypair.publicKey}`);

    const metadataInfo = await client.getAccount(metadataAddress);
    assert(metadataInfo !== null, 'metadata account not created');

    const [metadata] = Metadata.fromAccountInfo({
      ...metadataInfo,
      data: Buffer.from(metadataInfo.data),
    });

    assert(metadata.data.name.slice(0, nftData.nft_title.length) === nftData.nft_title, 'name does not match');
    assert(metadata.data.symbol.slice(0, nftData.nft_symbol.length) === nftData.nft_symbol, 'symbol does not match');
    assert(metadata.data.uri.slice(0, nftData.nft_uri.length) === nftData.nft_uri, 'uri does not match');
    assert(metadata.mint.toBase58() === nftMintKeypair.publicKey.toBase58(), 'mint does not match');

    const mintInfo = await client.getAccount(nftMintKeypair.publicKey);
    const mint = MintLayout.decode(mintInfo.data);
    assert(mintAuthorityPublicKey.toBase58() === mint.mintAuthority.toBase58(), 'mint authority does not match');
  });

  test('Mint the NFT to your wallet!', async () => {
    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), nftMintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    const editionAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), nftMintKeypair.publicKey.toBuffer(), Buffer.from('edition')],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(nftMintKeypair.publicKey, payer.publicKey);

    const instructionData = new MintToArgs({
      instruction: NftMinterInstruction.MintTo,
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: nftMintKeypair.publicKey, isSigner: false, isWritable: true }, // Mint account
        { pubkey: metadataAddress, isSigner: false, isWritable: true }, // Metadata account
        { pubkey: editionAddress, isSigner: false, isWritable: true }, // Edition account
        { pubkey: mintAuthorityPublicKey, isSigner: false, isWritable: true }, // Mint authority account
        {
          pubkey: associatedTokenAccountAddress,
          isSigner: false,
          isWritable: true,
        }, // ATA
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Payer
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // Rent account
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Associated token program
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
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   ATA Address: ${associatedTokenAccountAddress}`);

    const tokenAccountInfo = await client.getAccount(associatedTokenAccountAddress);
    assert(tokenAccountInfo !== null, 'token account not created');

    const tokenAccount = AccountLayout.decode(tokenAccountInfo.data);
    assert(tokenAccount.amount === BigInt(1), 'amount is not equal to 1');
    assert(tokenAccount.mint.toBase58() === nftMintKeypair.publicKey.toBase58(), 'mint key does not match');
  });
});
