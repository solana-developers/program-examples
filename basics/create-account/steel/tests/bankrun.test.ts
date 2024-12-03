import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';

import { ProgramTestContext, BanksClient, start } from 'solana-bankrun';
import * as borsh from 'borsh';
import { describe, it } from 'mocha';
import { assert } from 'chai';
const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

const instructionDiscriminators = {
  CreateSystemAccount: Buffer.from([0]),
};

const getCreateSystemAccountInstructionData = () => {
  return Buffer.concat([instructionDiscriminators.CreateSystemAccount]);
};

describe('Create account Program', () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  const newAccount = Keypair.generate();

  before(async () => {
    context = await start([{ name: 'create_account_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('Should create system account successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: newAccount.publicKey, isSigner: true, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getCreateSystemAccountInstructionData(),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer, newAccount);

    // process the transaction
    await client.processTransaction(tx);

    const accountInfo = await client.getAccount(newAccount.publicKey);
    assert.isNotNull(accountInfo);
    assert(accountInfo.owner.toBase58() === SystemProgram.programId.toBase58());
  });
});
