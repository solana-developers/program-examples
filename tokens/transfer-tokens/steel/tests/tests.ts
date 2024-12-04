import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { start } from 'solana-bankrun';
import { CreateTokenArgs, MintToArgs, TransferTokensArgs } from './instructions';

describe('Transferring Tokens', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start(
    [
      { name: 'transfer_tokens_program', programId: PROGRAM_ID },
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
    const metadataPDA = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), tokenMintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // SPL Token default = 9 decimals
    //
    const createArgs = new CreateTokenArgs(
      'Solana Gold',
      'GOLDSOL',
      'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
      9,
    );

    const createTokenIx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        {
          pubkey: tokenMintKeypair.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: metadataPDA, isSigner: false, isWritable: true },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: createArgs.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(createTokenIx).sign(payer, tokenMintKeypair);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${tokenMintKeypair.publicKey}`);
  });

  test('Create an NFT!', async () => {
    const metadataPDA = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), nftMintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // NFT default = 0 decimals
    //
    const createArgs = new CreateTokenArgs(
      'Homer NFT',
      'HOMR',
      'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
      0,
    );

    const createTokenIx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: nftMintKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: metadataPDA, isSigner: false, isWritable: true },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: createArgs.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(createTokenIx).sign(payer, nftMintKeypair);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${nftMintKeypair.publicKey}`);
  });

  test('Mint some tokens to your wallet!', async () => {
    const recipientATA = getAssociatedTokenAddressSync(tokenMintKeypair.publicKey, payer.publicKey);

    const mintArgs = new MintToArgs(100);

    const mintToIx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // mint_authority
        { pubkey: payer.publicKey, isSigner: false, isWritable: false }, // recipient
        {
          pubkey: tokenMintKeypair.publicKey,
          isSigner: false,
          isWritable: true,
        }, // mint_pda must be writable
        { pubkey: recipientATA, isSigner: false, isWritable: true }, // associated_token_account must be writable
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // spl_token::ID
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // spl_associated_token_account::ID
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        }, // system_program::ID
      ],
      programId: PROGRAM_ID,
      data: mintArgs.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(mintToIx).sign(payer);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   ATA Address: ${recipientATA}`);
  });

  test('Transfer tokens to another wallet!', async () => {
    const fromAssociatedTokenAddress = getAssociatedTokenAddressSync(tokenMintKeypair.publicKey, payer.publicKey);
    console.log(`Owner Token Address: ${fromAssociatedTokenAddress}`);
    const toAssociatedTokenAddress = getAssociatedTokenAddressSync(tokenMintKeypair.publicKey, recipientWallet.publicKey);
    console.log(`Recipient Token Address: ${toAssociatedTokenAddress}`);

    const transferArgs = new TransferTokensArgs(15);

    const transferTokensIx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Owner
        { pubkey: recipientWallet.publicKey, isSigner: true, isWritable: true }, // Recipient
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
        // { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Payer
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Associated token program
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
      ],
      programId: PROGRAM_ID,
      data: transferArgs.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(transferTokensIx).sign(payer, recipientWallet);

    await client.processTransaction(tx);
  });
});
