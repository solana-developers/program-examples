import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { start } from 'solana-bankrun';
import { CreateTokenArgs, InitArgs, MintToArgs, NftMinterInstruction } from './instructions';

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

  const mintAuthorityPublicKey = PublicKey.findProgramAddressSync([Buffer.from('mint_authority')], PROGRAM_ID)[0];

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
  });

  test('Create an NFT!', async () => {
    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), nftMintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // NFT default = 0 decimals
    //
    const instructionData = new CreateTokenArgs({
      instruction: NftMinterInstruction.CreateToken,
      nft_title: 'Homer NFT',
      nft_symbol: 'HOMR',
      nft_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
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
  });
});
