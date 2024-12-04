import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BN } from 'bn.js';
import { start } from 'solana-bankrun';
import { CreateTokenArgs, MintToArgs, SplMinterInstruction } from './instructions';

describe('SPL Token Minter!', async () => {
  const PROGRAM_ID = new PublicKey('8V26fyhrQobKbvkRCV3KvT6jZQLzviovdARfGrw8kUdG');
  const context = await start(
    [
      { name: 'spl_token_minter_program', programId: PROGRAM_ID },
      { name: 'token_metadata', programId: TOKEN_METADATA_PROGRAM_ID },
    ],
    [],
  );
  const client = context.banksClient;
  const payer = context.payer;

  const mintKeypair: Keypair = Keypair.generate();

  test('Create an SPL Token!', async () => {
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
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true },
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
    tx.add(createTokenIx).sign(payer, mintKeypair);

    await client.processTransaction(tx);

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
  });

  test('Mint some tokens to your wallet!', async () => {
    const recipientATA = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey);

    const mintArgs = new MintToArgs(100);

    const mintToIx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // mint_authority
        { pubkey: payer.publicKey, isSigner: false, isWritable: false }, // recipient
        { pubkey: mintKeypair.publicKey, isSigner: false, isWritable: true }, // mint_pda must be writable
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
});
