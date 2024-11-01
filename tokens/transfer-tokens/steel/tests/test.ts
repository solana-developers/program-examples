import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BN } from 'bn.js';
import { start } from 'solana-bankrun';
import { CreateTokenArgs, MintNftArgs, MintSplArgs, MyInstruction, TransferTokensArgs } from './instructions';

describe('Transfer Tokens!', async () => {
  const PROGRAM_ID = PublicKey.unique();
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
    const instructionData = new CreateTokenArgs({
      token_title: 'Solana Gold',
      token_symbol: 'GOLDSOL',
      token_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
      token_decimals: 9,
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
  });

  test('Create an NFT!', async () => {
    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), nftMintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // NFT default = 0 decimals
    //
    const instructionData = new CreateTokenArgs({
      token_title: 'Homer NFT',
      token_symbol: 'HOMR',
      token_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
      token_decimals: 0,
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
  });
});
