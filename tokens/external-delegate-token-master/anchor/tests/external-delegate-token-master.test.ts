import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { assert } from 'chai';
import { LiteSVM } from 'litesvm';
import { LiteSVMProvider } from 'anchor-litesvm';
import { describe, it, before } from 'node:test';
import type { ExternalDelegateTokenMaster } from '../target/types/external_delegate_token_master';

import IDL from '../target/idl/external_delegate_token_master.json' with { type: 'json' };

const PROGRAM_ID = new PublicKey(IDL.address);
const ACCOUNT_SIZE = 8 + 32 + 20; // discriminator + authority pubkey + ethereum address

describe('External Delegate Token Master Tests', () => {
  let svm: LiteSVM;
  let provider: LiteSVMProvider;
  let program: anchor.Program<ExternalDelegateTokenMaster>;
  let authority: Keypair;
  let payer: Keypair;
  let userAccount: Keypair;

  before(() => {
    authority = Keypair.generate();
    userAccount = Keypair.generate();
    payer = Keypair.generate();

    svm = new LiteSVM();
    svm.addProgramFromFile(PROGRAM_ID, 'target/deploy/external_delegate_token_master.so');

    svm.airdrop(payer.publicKey, BigInt(100 * LAMPORTS_PER_SOL));
    svm.airdrop(authority.publicKey, BigInt(100 * LAMPORTS_PER_SOL));

    provider = new LiteSVMProvider(svm, new anchor.Wallet(payer));
    program = new anchor.Program<ExternalDelegateTokenMaster>(IDL, provider);
  });

  it('should initialize user account', async () => {
    const rentExempt = Number(svm.minimumBalanceForRentExemption(BigInt(ACCOUNT_SIZE)));

    await program.methods
      .initialize()
      .accounts({
        userAccount: userAccount.publicKey,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .preInstructions([
        SystemProgram.createAccount({
          fromPubkey: authority.publicKey,
          newAccountPubkey: userAccount.publicKey,
          lamports: rentExempt,
          space: ACCOUNT_SIZE,
          programId: PROGRAM_ID,
        }),
      ])
      .signers([authority, userAccount])
      .rpc();

    const account = await program.account.userAccount.fetch(userAccount.publicKey);
    assert.equal(account.authority.toString(), authority.publicKey.toString());
    assert.deepEqual(account.ethereumAddress, new Array(20).fill(0));
  });

  it('should set ethereum address', async () => {
    const ethereumAddress = Array.from(Buffer.from('1C8cd0c38F8DE35d6056c7C7aBFa7e65D260E816', 'hex'));

    await program.methods
      .setEthereumAddress(ethereumAddress)
      .accounts({
        userAccount: userAccount.publicKey,
        authority: authority.publicKey,
      })
      .signers([authority])
      .rpc();

    const account = await program.account.userAccount.fetch(userAccount.publicKey);
    assert.deepEqual(account.ethereumAddress, ethereumAddress);
  });

  it('should perform authority transfer', async () => {
    const newAuthority = Keypair.generate();

    await program.methods
      .transferAuthority(newAuthority.publicKey)
      .accounts({
        userAccount: userAccount.publicKey,
        authority: authority.publicKey,
      })
      .signers([authority])
      .rpc();

    const account = await program.account.userAccount.fetch(userAccount.publicKey);
    assert.equal(account.authority.toString(), newAuthority.publicKey.toString());
  });
});
