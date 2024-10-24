import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { start } from 'solana-bankrun';

export enum InstructionType {
  TransferWithProgram = 0,
  TransferWithCPI = 1,
}

class InstructionData {
  instruction: number;
  amount: number;

  constructor(props: { instruction: number; amount: number }) {
    this.instruction = props.instruction;
    this.amount = props.amount;
  }
}

const instructionDataSchema = {
  struct: {
    instruction: 'u8',
    amount: 'u64',
  },
};

describe('transfer-sol', async () => {
  let context: any;
  let client: any;
  let payer: any;
  const PROGRAM_ID = PublicKey.unique();

  before(async () => {
    // load program in solana-bankrun
    context = await start([{ name: 'steel_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('Transfer using CPI', async () => {
    const transferAmount = 1 * LAMPORTS_PER_SOL;
    const recipient = Keypair.generate();

    const preTransferRecipientBalance = await client.getBalance(recipient.publicKey);

    const ixData = new InstructionData({
      instruction: InstructionType.TransferWithCPI,
      amount: transferAmount,
    });

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: payer.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: recipient.publicKey,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from(borsh.serialize(instructionDataSchema, ixData)),
    });

    const tx = new Transaction();
    const res = await client.getLatestBlockhash();

    const [blockhash, _] = res;

    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);

    const postTransferRecipientBalance = await client.getBalance(recipient.publicKey);

    assert.equal(postTransferRecipientBalance, preTransferRecipientBalance + BigInt(transferAmount));
  });

  it('Transfer using system program', async () => {
    const transferAmount = 1 * LAMPORTS_PER_SOL;
    const recipient = Keypair.generate();

    const createAccountIx = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: recipient.publicKey,
      lamports: transferAmount,
      space: 0,
      programId: SystemProgram.programId,
    });

    let tx = new Transaction();
    tx.add(createAccountIx);
    const [blockhash, _] = await client.getLatestBlockhash();

    tx.recentBlockhash = blockhash;
    tx.sign(payer, recipient);

    await client.processTransaction(tx);

    const preTransferRecipientBalance = await client.getBalance(recipient.publicKey);

    const ixData = new InstructionData({
      instruction: InstructionType.TransferWithProgram,
      amount: transferAmount,
    });

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: payer.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: recipient.publicKey,
          isSigner: false,
          isWritable: true,
        },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from(borsh.serialize(instructionDataSchema, ixData)),
    });

    tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, recipient);

    await client.processTransaction(tx);

    const postTransferRecipientBalance = await client.getBalance(recipient.publicKey);

    assert.equal(postTransferRecipientBalance, preTransferRecipientBalance + BigInt(transferAmount));
  });
});

//   test("Transfer between accounts using our program", async () => {
//     await getBalances(
//       test2Recipient1.publicKey,
//       test2Recipient2.publicKey,
//       "Beginning"
//     );

//     const ix = createTransferInstruction(
//       test2Recipient1.publicKey,
//       test2Recipient2.publicKey,
//       PROGRAM_ID,
//       InstructionType.ProgramTransfer,
//       transferAmount
//     );

//     const tx = new Transaction();
//     const res = await client.getLatestBlockhash();

//     if (res) {
//       let [blockhash, _] = res;

//       tx.recentBlockhash = blockhash;
//       tx.add(ix).sign(payer, test2Recipient1);

//       await client.processTransaction(tx);

//       await getBalances(
//         test2Recipient1.publicKey,
//         test2Recipient2.publicKey,
//         "Resulting"
//       );
//     }
//   });
