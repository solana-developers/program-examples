import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { findTreeConfigPda } from '@metaplex-foundation/mpl-bubblegum';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { ValidDepthSizePair, createAllocTreeIx } from '@solana/spl-account-compression';
import { createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import { Keypair, LAMPORTS_PER_SOL, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { CnftCandyMachine } from '../target/types/cnft_candy_machine';

describe('cnft-candy-machine', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CnftCandyMachine as Program<CnftCandyMachine>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');

  const wallet = provider.wallet as anchor.Wallet;

  const config = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('config'), wallet.publicKey.toBuffer()], program.programId);

  let allowMint: anchor.web3.PublicKey;
  let paymentMint: anchor.web3.PublicKey;

  const allowedOne = Keypair.generate();
  const allowedTwo = Keypair.generate();
  const allowedThree = Keypair.generate();
  const publicOne = Keypair.generate();

  const maxDepthSizePair: ValidDepthSizePair = {
    maxDepth: 14,
    maxBufferSize: 64,
  };
  const canopyDepth = maxDepthSizePair.maxDepth - 5;

  const emptyMerkleTree = anchor.web3.Keypair.generate();
  console.log(`Merke tree: ${emptyMerkleTree.publicKey.toBase58()}`);
  const umi = createUmi(provider.connection.rpcEndpoint);
  const treeConfig = findTreeConfigPda(umi, {
    merkleTree: emptyMerkleTree.publicKey.toBase58(),
  })[0];

  const treeConfigPublicKey = new anchor.web3.PublicKey(treeConfig);
  console.log('treeConfigPublicKey', treeConfigPublicKey.toBase58());

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const getMetadata = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];
  };

  const getMasterEdition = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer(), Buffer.from('edition')],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];
  };

  it('Airdrop SOl to wallet', async () => {
    const tx = await provider.connection.requestAirdrop(allowedOne.publicKey, 10 * LAMPORTS_PER_SOL).then(confirm);
    const tx2 = await provider.connection.requestAirdrop(publicOne.publicKey, 10 * LAMPORTS_PER_SOL).then(confirm);
    console.log('\nAirdrop to Allowed User done: ', tx);
    console.log('Airdrop to Public User done: ', tx2);
  });

  it('Create payment mint', async () => {
    paymentMint = await createMint(provider.connection, wallet.payer, provider.publicKey, provider.publicKey, 6);
    console.log('\nPayment mint created: ', paymentMint.toBase58());
  });

  it('Mint payment mint', async () => {
    const destination = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, paymentMint, publicOne.publicKey)).address;
    const tx = await mintTo(provider.connection, wallet.payer, paymentMint, destination, wallet.payer, 100_000_000);
    console.log('\nPayment mint minted to user: ', wallet.publicKey.toBase58());
    console.log('Current payment mint balance: ', (await provider.connection.getTokenAccountBalance(destination)).value.uiAmount);
    console.log('Your transaction signature', tx);
  });

  it('Create allow mint', async () => {
    allowMint = await createMint(provider.connection, wallet.payer, provider.publicKey, provider.publicKey, 6);
    console.log('\nAllow mint created: ', allowMint.toBase58());
  });

  it('Mint allow mint', async () => {
    const destination = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, allowMint, wallet.publicKey)).address;
    const tx = await mintTo(provider.connection, wallet.payer, allowMint, destination, wallet.payer, 20_000_000);
    console.log('\nAllow mint minted to user: ', wallet.publicKey.toBase58());
    console.log('Current allow mint balance: ', (await provider.connection.getTokenAccountBalance(destination)).value.uiAmount);
    console.log('Your transaction signature', tx);
  });

  it('Create Config Account, Initialize Merkle Tree, Create Collection Mint', async () => {
    // Add your test here.
    const allocTreeIx = await createAllocTreeIx(provider.connection, emptyMerkleTree.publicKey, provider.publicKey, maxDepthSizePair, canopyDepth);

    const signature = await sendAndConfirmTransaction(provider.connection, new Transaction().add(allocTreeIx), [wallet.payer, emptyMerkleTree]);

    console.log('\nAllocated tree', signature);

    const tx = await program.methods
      .initialize(100, new anchor.BN(0.2 * LAMPORTS_PER_SOL), new anchor.BN(5_000_000), paymentMint, 14, 64)
      .accounts({
        authority: provider.wallet.publicKey,
        allowMint,
        merkleTree: emptyMerkleTree.publicKey,
        treeConfig: treeConfigPublicKey,
      })
      .rpc();
    console.log('Config account created');
    console.log('Merkle tree initialized');
    console.log('Your transaction signature', tx);
  });

  it('Mint Collection NFT', async () => {
    const tx = await program.methods
      .createCollection('Test', 'TST', 'https://arweave.net/123')
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log('\nCollection NFT minted');
    console.log('Your transaction signature', tx);
  });

  it('Add user to allow list', async () => {
    const tx = await program.methods
      .addAllowList(allowedOne.publicKey, 88)
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log('\nUser added to allow list: ', allowedOne.publicKey.toBase58());

    const allowList = await program.account.config.fetch(config[0]);
    console.log('\nAllow list:');
    allowList.allowList.forEach((user) => console.log('User: ', user.user.toBase58(), '\tAmount: ', user.amount));
  });

  it('Add user to allow list', async () => {
    const tx = await program.methods
      .addAllowList(allowedTwo.publicKey, 10)
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log('\nUser added to allow list: ', allowedTwo.publicKey.toBase58());

    const allowList = await program.account.config.fetch(config[0]);
    console.log('\nAllow list:');
    allowList.allowList.forEach((user) => console.log('User: ', user.user.toBase58(), '\tAmount: ', user.amount));
  });

  it('Add user to allow list', async () => {
    const tx = await program.methods
      .addAllowList(allowedThree.publicKey, 50)
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log('\nUser added to allow list: ', allowedThree.publicKey.toBase58());

    const allowList = await program.account.config.fetch(config[0]);
    console.log('\nAllow list:');
    allowList.allowList.forEach((user) => console.log('User: ', user.user.toBase58(), '\tAmount: ', user.amount));
  });

  it('Mint cNFT with Allow List - Pay with SOL', async () => {
    console.log('\nMinting cNFT for user: ', allowedOne.publicKey.toBase58());
    console.log(
      'User allowed amount: ',
      await program.account.config.fetch(config[0]).then((config) => config.allowList.find((user) => user.user.equals(allowedOne.publicKey))?.amount),
    );

    const tx = await program.methods
      .mint('Test', 'TST', 'https://arweave.net/123', true)
      .accounts({
        user: allowedOne.publicKey,
        authority: provider.wallet.publicKey,
        allowMint: null,
        allowMintAta: null,
        treeConfig: treeConfigPublicKey,
        merkleTree: emptyMerkleTree.publicKey,
      })
      .signers([allowedOne])
      .rpc();

    console.log(`\ncNFT minted for user: ${allowedOne.publicKey.toBase58()} with tx: ${tx}`);
    console.log(
      'User allowed amount: ',
      await program.account.config.fetch(config[0]).then((config) => config.allowList.find((user) => user.user.equals(allowedOne.publicKey))?.amount),
    );
  });

  it('Mint cNFT with Allow Token - Pay with SOL', async () => {
    console.log('\nMinting cNFT for user: ', wallet.publicKey.toBase58());

    const allowMintAta = getAssociatedTokenAddressSync(allowMint, wallet.publicKey);
    console.log('Allow mint balance before mint: ', (await provider.connection.getTokenAccountBalance(allowMintAta)).value.uiAmount);

    const tx = await program.methods
      .mint('Test', 'TST', 'https://arweave.net/123', true)
      .accounts({
        user: wallet.publicKey,
        authority: provider.wallet.publicKey,
        allowMint,
        allowMintAta,
        treeConfig: treeConfigPublicKey,
        merkleTree: emptyMerkleTree.publicKey,
      })
      .rpc();

    console.log('Allow mint balance after mint: ', (await provider.connection.getTokenAccountBalance(allowMintAta)).value.uiAmount);
  });

  it('Mint cNFT to Public User (Tree is Private, so test shall fail) - Pay with SOL', async () => {
    try {
      console.log('\nMinting cNFT for user: ', publicOne.publicKey.toBase58());

      const tx = await program.methods
        .mint('Test', 'TST', 'https://arweave.net/123', true)
        .accounts({
          user: publicOne.publicKey,
          authority: provider.wallet.publicKey,
          allowMint: null,
          allowMintAta: null,
          treeConfig: treeConfigPublicKey,
          merkleTree: emptyMerkleTree.publicKey,
        })
        .signers([publicOne])
        .rpc();

      console.log('\ncNFT minted for Public User');
      console.log('Transaction signature:', tx);
    } catch (error) {
      console.log('\nError: ', error.error.errorMessage);
    }
  });

  it('Change Tree Status to Public', async () => {
    console.log('\nCurrent tree status: ', await program.account.config.fetch(config[0]).then((config) => config.status));

    const tx = await program.methods
      .setTreeStatus({ public: {} })
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log('\nTree status changed to Public');
    console.log('Current tree status: ', await program.account.config.fetch(config[0]).then((config) => config.status));
    console.log('\nTransaction signature:', tx);
  });

  it('Mint cNFT to Public User (Tree is now public) - Pay with SOL', async () => {
    console.log('\nMinting cNFT for user: ', publicOne.publicKey.toBase58());

    const tx = await program.methods
      .mint('Test', 'TST', 'https://arweave.net/123', true)
      .accounts({
        user: publicOne.publicKey,
        authority: provider.wallet.publicKey,
        allowMint: null,
        allowMintAta: null,
        treeConfig: treeConfigPublicKey,
        merkleTree: emptyMerkleTree.publicKey,
      })
      .signers([publicOne])
      .rpc();

    console.log('\ncNFT minted for Public User');
    console.log('Transaction signature:', tx);
  });

  it('Mint cNFT to Public User (Tree is now public) - Pay with SPL', async () => {
    console.log('\nMinting cNFT for user: ', publicOne.publicKey.toBase58());

    const source = await getOrCreateAssociatedTokenAccount(provider.connection, publicOne, paymentMint, publicOne.publicKey);
    const destination = await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, paymentMint, provider.wallet.publicKey);

    console.log('User Payment Mint balance before mint: ', (await provider.connection.getTokenAccountBalance(source.address)).value.uiAmount);

    const tx = await program.methods
      .mint('Test', 'TST', 'https://arweave.net/123', false)
      .accounts({
        user: publicOne.publicKey,
        authority: provider.wallet.publicKey,
        allowMint: null,
        allowMintAta: null,
        treeConfig: treeConfigPublicKey,
        merkleTree: emptyMerkleTree.publicKey,
      })
      .remainingAccounts([
        { pubkey: source.address, isWritable: true, isSigner: false },
        { pubkey: destination.address, isWritable: true, isSigner: false },
      ])
      .signers([publicOne])
      .rpc({ skipPreflight: true });

    console.log('\ncNFT minted for Public User');
    console.log('User Payment Mint balance after mint: ', (await provider.connection.getTokenAccountBalance(source.address)).value.uiAmount);
    console.log('\nTransaction signature:', tx);
  });
});
