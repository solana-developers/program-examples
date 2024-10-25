import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';
import { getCreateAmmInstructionData } from './utils';

const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

describe('Account Data Program', () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  let admin: Keypair;
  let fee: number;

  before(async () => {
    context = await start([{ name: 'token_swap_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
    admin = Keypair.generate();
    fee = 1000; // 10%
  });

  it('Should create a new amm successfully', async () => {
    const id = Keypair.generate();
    const [ammPda] = PublicKey.findProgramAddressSync([id.publicKey.toBuffer()], PROGRAM_ID);

    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: admin.publicKey, isSigner: false, isWritable: false },
          { pubkey: ammPda, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getCreateAmmInstructionData(id.publicKey, fee),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    const account = await client.getAccount(ammPda);
    assert.isNotNull(account);
    assert.equal(account?.owner.toBase58(), PROGRAM_ID.toBase58());
  });
});
