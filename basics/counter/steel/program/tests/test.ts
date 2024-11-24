import { describe, it } from 'mocha';
import { PublicKey, Transaction, TransactionInstruction, SystemProgram, Connection, clusterApiUrl, Keypair } from '@solana/web3.js';
import { assert } from 'chai';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

describe('steelcounter', function () {
  // load program in solana-bankrun
  let context: ProgramTestContext; 
  let client: BanksClient; 
  let payer: Keypair; 
  const PROGRAM_ID = PublicKey.unique();
  const COUNTER_SEED = Buffer.from("counter");
  before(async () => { // load program in solana-bankrun 
    context = await start( 
    [{ 
      name: "steelcounter_program", 
      programId: PROGRAM_ID 
    }], [] ); 
    client = context.banksClient; 
    payer = context.payer; 
  });


  it('start', async () => {
    const blockhash = context.lastBlockhash;
    // We set up our instruction first.
    const [counterD]  = PublicKey.findProgramAddressSync([COUNTER_SEED], PROGRAM_ID)

    const startTx = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: counterD, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from([0]), // No data
    });


    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(startTx).sign(payer);

    // Now we process the transaction
    const transaction = await client.processTransaction(tx);
    assert(transaction.logMessages[3].startsWith(`Program log: counter created`));

  });

  it("add one", async () => {
    const blockhash = context.lastBlockhash;

    const [counterD] = PublicKey.findProgramAddressSync(
        [COUNTER_SEED],
        PROGRAM_ID,
    );

    //setup instructions
    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: counterD, isSigner: false, isWritable: true },
      ],
      data: Buffer.from([1]),
    });


    const tx = new Transaction(); 
    tx.recentBlockhash = blockhash; 
    tx.add(ix).sign(payer);
    // Now we process the transaction 
    const transaction = await client.processTransaction(tx);
    assert(transaction.logMessages[1] === `Program log: Counter value incremented!`);
    console.log(transaction.logMessages[2]);

  })
});