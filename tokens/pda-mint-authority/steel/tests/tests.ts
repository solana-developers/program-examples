import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { start } from 'solana-bankrun';
import { CreateTokenArgs, InitArgs, MintToArgs } from './instructions';

describe('PDA MINT AUTHORITY', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start(
    [
      { name: 'pda_mint_authority_program', programId: PROGRAM_ID },
      { name: 'token_metadata', programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  const mintKeypair: Keypair = Keypair.generate();
  const mintAuthorityPublicKey = PublicKey.findProgramAddressSync([Buffer.from('mint_authority')], PROGRAM_ID)[0];

  test('Init mint authority PDA!', async () => {
    const metadataPDA = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    const initArgs = new InitArgs();

    const initIx = new TransactionInstruction({
      keys: [
        { pubkey: mintAuthorityPublicKey, isSigner: false, isWritable: true },
        { pubkey: payer.publicKey, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: initArgs.toBuffer(),
    });

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(initIx).sign(payer);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
  });

  test('Create a SPL Token with PDA!', async () => {
    const metadataPDA = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // SPL Token default = 9 decimals
    //
    const createArgs = new CreateTokenArgs(
      'Solana Gold',
      'GOLDSOL',
      'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
    );

    const createTokenIx = new TransactionInstruction({
      keys: [
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: mintAuthorityPublicKey, isSigner: false, isWritable: true },
        { pubkey: metadataPDA, isSigner: false, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
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
    tx.add(createTokenIx).sign(payer, mintKeypair);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
  });

  test('Mint some tokens to your wallet with PDA!', async () => {
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);

    const mintArgs = new MintToArgs(100);

    const mintToIx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: mintKeypair.publicKey, isSigner: false, isWritable: true }, // mint_pda must be writable
        {
          pubkey: associatedTokenAccountAddress,
          isSigner: false,
          isWritable: true,
        }, // ATA
        { pubkey: mintAuthorityPublicKey, isSigner: false, isWritable: true },
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
    console.log(`   ATA Address: ${associatedTokenAccountAddress}`);
  });
});
