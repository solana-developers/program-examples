import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Metadata, PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { ASSOCIATED_TOKEN_PROGRAM_ID, AccountLayout, MintLayout, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BN } from 'bn.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';
import { CreateTokenArgs, MintNftArgs, MintSplArgs, MyInstruction, TransferTokensArgs } from './instructions';

describe('Transfer Tokens!', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start(
    [
      { name: 'transfer_tokens_steel_program', programId: PROGRAM_ID },
      { name: 'token_metadata', programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  const tokenMintKeypair: Keypair = Keypair.generate();
  const nftMintKeypair: Keypair = Keypair.generate();

  const recipientWallet = Keypair.generate();

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

    const instructionData = new CreateTokenArgs({
      ...tokenDetails,
      discriminator: 0,
      token_title: Buffer.from(tokenDetails.token_title.padEnd(32, '\0')),
      token_symbol: Buffer.from(tokenDetails.token_symbol.padEnd(10, '\0')),
      token_uri: Buffer.from(tokenDetails.token_uri.padEnd(256, '\0')),
    });

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

  test('Create an NFT!', async () => {
    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), nftMintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // NFT default = 0 decimals
    //
    const nftDetails = {
      token_title: 'Homer NFT',
      token_symbol: 'HOMR',
      token_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
      decimals: 0,
    };

    const instructionData = new CreateTokenArgs({
      ...nftDetails,
      discriminator: 0,
      token_title: Buffer.from(nftDetails.token_title.padEnd(32, '\0')),
      token_symbol: Buffer.from(nftDetails.token_symbol.padEnd(10, '\0')),
      token_uri: Buffer.from(nftDetails.token_uri.padEnd(256, '\0')),
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: nftMintKeypair.publicKey, isSigner: true, isWritable: true }, // Mint account
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

    assert(metadata.data.name.slice(0, nftDetails.token_title.length) === nftDetails.token_title, 'name does not match');
    assert(metadata.data.symbol.slice(0, nftDetails.token_symbol.length) === nftDetails.token_symbol, 'symbol does not match');
    assert(metadata.data.uri.slice(0, nftDetails.token_uri.length) === nftDetails.token_uri, 'uri does not match');
    assert(metadata.mint.toBase58() === nftMintKeypair.publicKey.toBase58(), 'mint does not match');

    const mintInfo = await client.getAccount(nftMintKeypair.publicKey);
    const mint = MintLayout.decode(mintInfo.data);
    assert(mint.mintAuthority.toBase58() === payer.publicKey.toBase58(), 'mint authority does not match');
    assert(mint.decimals === 0, 'mint decimals does not match');
  });

  test('Mint some tokens to your wallet!', async () => {
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(tokenMintKeypair.publicKey, payer.publicKey);

    const instructionData = new MintSplArgs({
      instruction: MyInstruction.MintSpl,
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

    const tokenAccountInfo = await client.getAccount(associatedTokenAccountAddress);
    assert(tokenAccountInfo !== null, 'token account not created');

    const tokenAccount = AccountLayout.decode(tokenAccountInfo.data);
    assert(tokenAccount.amount === BigInt(150), 'amount is not equal to 150');
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

    const instructionData = new MintNftArgs({
      instruction: MyInstruction.MintNft,
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: nftMintKeypair.publicKey, isSigner: false, isWritable: true }, // Mint account
        { pubkey: metadataAddress, isSigner: false, isWritable: true }, // Metadata account
        { pubkey: editionAddress, isSigner: false, isWritable: true }, // Edition account
        { pubkey: payer.publicKey, isSigner: false, isWritable: true }, // Mint authority account
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

  test('Prep a new test wallet for transfers', async () => {
    const ix = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: recipientWallet.publicKey,
      lamports: 1 * LAMPORTS_PER_SOL,
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);

    console.log(`Recipient Pubkey: ${recipientWallet.publicKey}`);
  });

  test('Transfer tokens to another wallet!', async () => {
    const fromAssociatedTokenAddress = getAssociatedTokenAddressSync(tokenMintKeypair.publicKey, payer.publicKey);
    console.log(`Owner Token Address: ${fromAssociatedTokenAddress}`);
    const toAssociatedTokenAddress = getAssociatedTokenAddressSync(tokenMintKeypair.publicKey, recipientWallet.publicKey);
    console.log(`Recipient Token Address: ${toAssociatedTokenAddress}`);

    const transferToInstructionData = new TransferTokensArgs({
      instruction: MyInstruction.TransferTokens,
      quantity: new BN(15),
    });

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: tokenMintKeypair.publicKey,
          isSigner: false,
          isWritable: true,
        }, // Mint account
        {
          pubkey: fromAssociatedTokenAddress,
          isSigner: false,
          isWritable: true,
        }, // Owner Token account
        { pubkey: toAssociatedTokenAddress, isSigner: false, isWritable: true }, // Recipient Token account
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Owner
        { pubkey: recipientWallet.publicKey, isSigner: true, isWritable: true }, // Recipient
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Payer
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Associated token program
      ],
      programId: PROGRAM_ID,
      data: transferToInstructionData.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, recipientWallet);

    await client.processTransaction(tx);

    const tokenAccountInfo = await client.getAccount(toAssociatedTokenAddress);
    assert(tokenAccountInfo !== null, 'token account not created');

    const tokenAccount = AccountLayout.decode(tokenAccountInfo.data);
    assert(tokenAccount.amount === BigInt(15), 'amount is not equal to 15');
  });

  test('Transfer NFT to another wallet!', async () => {
    const fromAssociatedTokenAddress = getAssociatedTokenAddressSync(nftMintKeypair.publicKey, payer.publicKey);
    console.log(`Owner Token Address: ${fromAssociatedTokenAddress}`);
    const toAssociatedTokenAddress = getAssociatedTokenAddressSync(nftMintKeypair.publicKey, recipientWallet.publicKey);
    console.log(`Recipient Token Address: ${toAssociatedTokenAddress}`);

    const transferToInstructionData = new TransferTokensArgs({
      instruction: MyInstruction.TransferTokens,
      quantity: new BN(1),
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: nftMintKeypair.publicKey, isSigner: false, isWritable: true }, // Mint account
        {
          pubkey: fromAssociatedTokenAddress,
          isSigner: false,
          isWritable: true,
        }, // Owner Token account
        { pubkey: toAssociatedTokenAddress, isSigner: false, isWritable: true }, // Recipient Token account
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Owner
        { pubkey: recipientWallet.publicKey, isSigner: true, isWritable: true }, // Recipient
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Payer
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Associated token program
      ],
      programId: PROGRAM_ID,
      data: transferToInstructionData.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, recipientWallet);

    await client.processTransaction(tx);

    const tokenAccountInfo = await client.getAccount(toAssociatedTokenAddress);
    assert(tokenAccountInfo !== null, 'token account not created');

    const tokenAccount = AccountLayout.decode(tokenAccountInfo.data);
    assert(tokenAccount.amount === BigInt(1), 'amount is not equal to 1');
  });
});
