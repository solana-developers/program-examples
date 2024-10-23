import { Buffer } from 'node:buffer';
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccount, createMint, mintTo } from '@solana/spl-token';
import { Connection, Keypair, PublicKey, SYSVAR_RENT_PUBKEY, Transaction, TransactionInstruction, sendAndConfirmTransaction } from '@solana/web3.js';
import { BN } from 'bn.js';
import { EscrowInstruction, ExchangeArgs, InitEscrowArgs } from './instructions';

function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(Buffer.from(JSON.parse(require('node:fs').readFileSync(path, 'utf-8'))));
}

describe('Escrow', async () => {
  const connection = new Connection('http://localhost:8899', 'confirmed');
  const payer = createKeypairFromFile(`${require('node:os').homedir()}/.config/solana/id.json`);
  const program = createKeypairFromFile('./program/target/deploy/solana_escrow_native-keypair.json');

  const alice = Keypair.generate();
  const bob = Keypair.generate();
  let mintA: PublicKey;
  let mintB: PublicKey;
  let aliceTokenAccountA: PublicKey;
  let aliceTokenAccountB: PublicKey;
  let bobTokenAccountA: PublicKey;
  let bobTokenAccountB: PublicKey;

  const escrowAccount = Keypair.generate();
  const pda = PublicKey.findProgramAddressSync([Buffer.from('escrow')], program.publicKey)[0];

  it('Initialize program state', async () => {
    // Airdrop SOL to Alice and Bob
    await connection.requestAirdrop(alice.publicKey, 1000000000);
    await connection.requestAirdrop(bob.publicKey, 1000000000);

    // Create mints
    mintA = await createMint(connection, payer, alice.publicKey, null, 0);
    mintB = await createMint(connection, payer, bob.publicKey, null, 0);

    // Create token accounts
    aliceTokenAccountA = await createAssociatedTokenAccount(connection, alice, mintA, alice.publicKey);
    aliceTokenAccountB = await createAssociatedTokenAccount(connection, alice, mintB, alice.publicKey);
    bobTokenAccountA = await createAssociatedTokenAccount(connection, bob, mintA, bob.publicKey);
    bobTokenAccountB = await createAssociatedTokenAccount(connection, bob, mintB, bob.publicKey);

    // Mint tokens to Alice and Bob
    await mintTo(connection, alice, mintA, aliceTokenAccountA, alice, 100);
    await mintTo(connection, bob, mintB, bobTokenAccountB, bob, 50);

    console.log('Program state initialized');
    console.log(`   Alice's Token A Account: ${aliceTokenAccountA}`);
    console.log(`   Bob's Token B Account: ${bobTokenAccountB}`);
  });

  it('Initialize escrow', async () => {
    const initEscrowIx = new TransactionInstruction({
      programId: program.publicKey,
      keys: [
        { pubkey: alice.publicKey, isSigner: true, isWritable: false },
        { pubkey: aliceTokenAccountA, isSigner: false, isWritable: true },
        { pubkey: aliceTokenAccountB, isSigner: false, isWritable: false },
        { pubkey: escrowAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      ],
      data: new InitEscrowArgs({
        instruction: EscrowInstruction.InitEscrow,
        amount: new BN(50),
      }).toBuffer(),
    });

    const tx = new Transaction().add(initEscrowIx);
    const sx = await sendAndConfirmTransaction(connection, tx, [alice, escrowAccount]);

    console.log('Escrow initialized');
    console.log(`   Escrow Account: ${escrowAccount.publicKey}`);
    console.log(`   Tx Signature: ${sx}`);
  });

  it('Exchange', async () => {
    const exchangeIx = new TransactionInstruction({
      programId: program.publicKey,
      keys: [
        { pubkey: bob.publicKey, isSigner: true, isWritable: false },
        { pubkey: bobTokenAccountB, isSigner: false, isWritable: true },
        { pubkey: bobTokenAccountA, isSigner: false, isWritable: true },
        { pubkey: aliceTokenAccountA, isSigner: false, isWritable: true },
        { pubkey: aliceTokenAccountB, isSigner: false, isWritable: true },
        { pubkey: alice.publicKey, isSigner: false, isWritable: true },
        { pubkey: escrowAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: pda, isSigner: false, isWritable: false },
      ],
      data: new ExchangeArgs({
        instruction: EscrowInstruction.Exchange,
        amount: new BN(50),
      }).toBuffer(),
    });

    const tx = new Transaction().add(exchangeIx);
    const sx = await sendAndConfirmTransaction(connection, tx, [bob]);

    console.log('Exchange completed');
    console.log(`   Tx Signature: ${sx}`);
  });
});
