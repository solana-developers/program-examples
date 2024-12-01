import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { Anchor } from "../target/types/anchor";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { assert } from 'chai';
import { Bankrun } from 'solana-bankrun';
import idl from '../target/idl/anchor.json';

describe('anchor', () => {
  const provider = AnchorProvider.env();
  const PROGRAM_ID = new PublicKey("7tUBaLEw5BVFmqWF8ixfxVSb7WM7CqRTXMmY8kK6uf1N");
  anchor.setProvider(provider);

  const program = new Program(idl, PROGRAM_ID, provider);
  const bankrun = new Bankrun(provider.connection, PROGRAM_ID);

  let user: Keypair;
  let walletState: string;
  let mint: Token;
  let userTokenAccount: PublicKey;

  before(async () => {
    user = Keypair.generate();

    await provider.connection.requestAirdrop(user.publicKey, LAMPORTS_PER_SOL * 1); 

    mint = await Token.createMint(
      provider.connection,
      user, 
      user.publicKey, 
      null, 
      9,
      TOKEN_PROGRAM_ID
    );

    userTokenAccount = await mint.createAccount(user.publicKey);

    walletState = await program.methods
      .initialize()
      .accounts({
        walletState: walletState,
        user: user.publicKey,
      })
      .rpc();
  });

  it('should mint tokens to user account', async () => {
    const mintAmount = 1000;

    await mint.mintTo(userTokenAccount, user.publicKey, [], mintAmount);

    const userAccountInfo = await mint.getAccountInfo(userTokenAccount);
    assert.equal(userAccountInfo.amount.toString(), mintAmount.toString()); 
  });

  it('should toggle on and allow token transfer', async () => {
    const mintAmount = 500; 
    const transferAmount = 200; 
    const recipient = Keypair.generate(); 

    await mint.mintTo(userTokenAccount, user.publicKey, [], mintAmount);
    
    await program.methods
      .toggle(true) 
      .accounts({
        walletState: walletState,
        user: user.publicKey,
      })
      .rpc();

    const recipientTokenAccount = await mint.createAccount(recipient.publicKey);

    await mint.transfer(userTokenAccount, recipientTokenAccount, user.publicKey, [], transferAmount);

    const userAccountInfo = await mint.getAccountInfo(userTokenAccount);
    const recipientAccountInfo = await mint.getAccountInfo(recipientTokenAccount);

    assert.equal(userAccountInfo.amount.toString(), (mintAmount - transferAmount).toString());
    assert.equal(recipientAccountInfo.amount.toString(), transferAmount.toString());
  });

  it('should toggle off and prevent token transfer', async () => {
    const mintAmount = 500;
    const transferAmount = 100;
    const recipient = Keypair.generate(); 

    await program.methods
      .toggle(false) 
      .accounts({
        walletState: walletState,
        user: user.publicKey,
      })
      .rpc();

    const recipientTokenAccount = await mint.createAccount(recipient.publicKey);

    try {
      await mint.transfer(userTokenAccount, recipientTokenAccount, user.publicKey, [], transferAmount);
      assert.fail('Expected transfer to fail, but it succeeded');
    } catch (error) {
      assert.include(error.message, 'transfer not allowed when toggle is off');
    }

    const userAccountInfo = await mint.getAccountInfo(userTokenAccount);
    assert.equal(userAccountInfo.amount.toString(), mintAmount.toString());
  });

});
