import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Metadata, PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { ASSOCIATED_TOKEN_PROGRAM_ID, AccountLayout, MintLayout, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BN } from 'bn.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';
import { CreateTokenArgs, MintToArgs, MyInstruction } from './instructions';

describe('SPL Token Minter!', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start(
    [
      { name: 'spl_token_minter_steel_program', programId: PROGRAM_ID },
      { name: 'token_metadata', programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  const tokenMintKeypair: Keypair = Keypair.generate();

  test('Create an SPL Token!', async () => {
    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), tokenMintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // SPL Token default = 9 decimals
    //
    const tokenDetails = {
      token_title: 'Solana Gold',
      token_symbol: 'GOLDSOL',
      token_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
      decimals: 9,
    };

    const instructionData = new CreateTokenArgs(tokenDetails);

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: tokenMintKeypair.publicKey,
          isSigner: true,
          isWritable: true,
        }, // Mint account
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
    tx.add(ix).sign(payer, tokenMintKeypair);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${tokenMintKeypair.publicKey}`);

    const metadataInfo = await client.getAccount(metadataAddress);
    assert(metadataInfo !== null, 'metadata account not created');

    const [metadata] = Metadata.fromAccountInfo({
      ...metadataInfo,
      data: Buffer.from(metadataInfo.data),
    });

    assert(metadata.data.name.slice(0, tokenDetails.token_title.length) === tokenDetails.token_title, 'name does not match');
    assert(metadata.data.symbol.slice(0, tokenDetails.token_symbol.length) === tokenDetails.token_symbol, 'symbol does not match');
    assert(metadata.data.uri.slice(0, tokenDetails.token_uri.length) === tokenDetails.token_uri, 'uri does not match');
    assert(metadata.mint.toBase58() === tokenMintKeypair.publicKey.toBase58(), 'mint does not match');

    const mintInfo = await client.getAccount(tokenMintKeypair.publicKey);
    const mint = MintLayout.decode(mintInfo.data);
    assert(mint.mintAuthority.toBase58() === payer.publicKey.toBase58(), 'mint authority does not match');
    assert(mint.decimals === tokenDetails.decimals, 'mint decimals does not match');
  });

  test('Mint some tokens to your wallet!', async () => {
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(tokenMintKeypair.publicKey, payer.publicKey);

    const instructionData = new MintToArgs({
      instruction: MyInstruction.MintTo,
      quantity: new BN(150),
    });

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: tokenMintKeypair.publicKey,
          isSigner: false,
          isWritable: true,
        }, // Mint account
        { pubkey: payer.publicKey, isSigner: false, isWritable: true }, // Mint authority account
        {
          pubkey: associatedTokenAccountAddress,
          isSigner: false,
          isWritable: true,
        }, // ATA
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Payer
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: true }, // System program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
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

    console.log('Success!');
    console.log(`   ATA Address: ${associatedTokenAccountAddress}`);

    const tokenAccountInfo = await client.getAccount(associatedTokenAccountAddress);
    assert(tokenAccountInfo !== null, 'token account not created');

    const tokenAccount = AccountLayout.decode(tokenAccountInfo.data);
    assert(tokenAccount.amount === BigInt(150), 'amount is not equal to 150');
    assert(tokenAccount.mint.toBase58() === tokenMintKeypair.publicKey.toBase58(), 'mint key does not match');
  });
});
