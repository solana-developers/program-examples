const anchor = require('@coral-xyz/anchor');
const { Keypair, SystemProgram, LAMPORTS_PER_SOL, PublicKey } = require('@solana/web3.js');

async function testCpiExample() {
  try {
    console.log('🚀 Starting CPI Example Test...');
    
    // Configure the client to use the local cluster
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    console.log('✅ Anchor provider configured');
    
    // Get the program from workspace
    const program = anchor.workspace.CpiExample;
    console.log('✅ Program loaded from workspace');
    console.log('Program ID:', program.programId.toBase58());
    
    // Test 1: Verify program deployment
    const accountInfo = await provider.connection.getAccountInfo(program.programId);
    if (accountInfo && accountInfo.executable) {
      console.log('✅ Program is deployed and executable');
    } else {
      console.log('❌ Program is not deployed or not executable');
      return;
    }
    
    // Test 2: Initialize the CPI example
    const cpiExampleKeypair = new Keypair();
    const payer = provider.wallet;
    
    console.log('🔄 Testing initialize function...');
    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          cpiExample: cpiExampleKeypair.publicKey,
          authority: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([cpiExampleKeypair])
        .rpc();
      
      console.log('✅ Initialize function executed successfully!');
      console.log('Transaction signature:', tx);
    } catch (error) {
      console.log('❌ Initialize function failed:', error.message);
      return;
    }
    
    // Test 3: Test SOL transfer via CPI
    console.log('🔄 Testing SOL transfer via CPI...');
    const fromAccountKeypair = new Keypair();
    const toAccountKeypair = new Keypair();
    
    // Fund the from account
    await provider.connection.requestAirdrop(fromAccountKeypair.publicKey, LAMPORTS_PER_SOL);
    
    // Create the to account by requesting a small airdrop
    await provider.connection.requestAirdrop(toAccountKeypair.publicKey, 1000); // Small amount to create account
    
    const transferAmount = new anchor.BN(0.1 * LAMPORTS_PER_SOL);
    
    try {
      const tx = await program.methods
        .transferSolViaCpi(transferAmount)
        .accounts({
          cpiExample: cpiExampleKeypair.publicKey,
          fromAccount: fromAccountKeypair.publicKey,
          toAccount: toAccountKeypair.publicKey,
          authority: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([fromAccountKeypair])
        .rpc();
      
      console.log('✅ SOL transfer via CPI successful!');
      console.log('Transaction signature:', tx);
    } catch (error) {
      console.log('❌ SOL transfer failed:', error.message);
      return;
    }
    
    console.log('🎉 All tests passed! CPI Example is working correctly.');
    
  } catch (error) {
    console.error('❌ Test failed:', error.message);
  }
}

testCpiExample();
