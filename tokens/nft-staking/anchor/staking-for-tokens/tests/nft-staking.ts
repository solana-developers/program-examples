import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IDL, NftStaking } from "../target/types/nft_staking";

import { 
  PublicKey, 
  Keypair,
  Commitment,
  SystemProgram, 
  LAMPORTS_PER_SOL 
} from "@solana/web3.js";

import {
  createNft, 
  mplTokenMetadata, 
  verifyCollection 
} from "@metaplex-foundation/mpl-token-metadata";

import {
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import {Connection} from "@solana/web3.js";

import { 
  MPL_TOKEN_METADATA_PROGRAM_ID
} from '@metaplex-foundation/mpl-token-metadata';

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, generateSigner, percentAmount, publicKey, signerIdentity } from "@metaplex-foundation/umi";

describe("nft-staking", () => {
  const commitment: Commitment = "confirmed"; // processed, confirmed, finalized
  const connection = new Connection("http://localhost:8899", {
      commitment,
      wsEndpoint: "ws://localhost:8900/",
  });
  // Configure the client to use the local cluster.
  const keypair = anchor.web3.Keypair.generate();

  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(keypair), { commitment });

  const programId = new PublicKey("GGWeSTDbz2WCERf59nTxwSRTFLMg7butvdk9tv4D53sN");

  const program = new anchor.Program<NftStaking>(IDL, programId, provider);

  // Helpers
  function wait(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
  }

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block
    })
    return signature
  }

  const log = async(signature: string): Promise<string> => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`);
    return signature;
  }

  // Variables
  let rewardMint: Keypair;
  let rewardATA: PublicKey;

  let collectionMint: PublicKey;
  let collectionMetadata: PublicKey;
  let collectionMasterEdition: PublicKey;
  let collectionATA: PublicKey;

  let nftMint: PublicKey;
  let nftMetadata: PublicKey;
  let nftMasterEdition: PublicKey;
  let nftATA: PublicKey;

  let stakingRuleAddress: PublicKey;
  let stakingAccountAddress: PublicKey;
  let stakingInstanceAddress: PublicKey;

  // Instructions
  it("Airdrop", async () => {
    await connection.requestAirdrop(keypair.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm)
    .then(log)
  })

  it("Create Collection", async () => {

    // Metaplex Setup
    const umi = createUmi(connection.rpcEndpoint);
    let umiKeypair = umi.eddsa.createKeypairFromSecretKey(keypair.secretKey);
    const signerKeypair = createSignerFromKeypair(umi, umiKeypair);
    umi.use(signerIdentity(signerKeypair));
    umi.use(mplTokenMetadata())
    const mint = generateSigner(umi);
    collectionMint = new PublicKey(mint.publicKey)

    // Create Collection NFT
    let minttx = createNft(
      umi, 
      {
        mint: mint,
        authority: signerKeypair,
        updateAuthority: umiKeypair.publicKey,
        name: "Collection Example",
        symbol: "EXM",
        uri: "",
        sellerFeeBasisPoints: percentAmount(0),
        creators: [
            {address: umiKeypair.publicKey, verified: true, share: 100 }
        ],
        collection: null,
        uses: null,
        isMutable: true,
        collectionDetails: null,
      }
    );

    await minttx.sendAndConfirm(umi, {
      send: {
        skipPreflight: true
      },
      confirm: {
        commitment
      }
    });

    // Create Collection Accounts
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      collectionMint,
      keypair.publicKey
    );

    collectionATA = ata.address;

    const metadata_seeds = [
      Buffer.from('metadata'),
      new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID).toBuffer(),
      new PublicKey(mint.publicKey).toBuffer(),
    ];

    const master_edition_seeds = [
      ...metadata_seeds,
      Buffer.from("edition")
    ];

    collectionMetadata = PublicKey.findProgramAddressSync(metadata_seeds, new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID))[0];
    collectionMasterEdition = PublicKey.findProgramAddressSync(master_edition_seeds, new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID))[0]; 
  });

  it("Create Staking Rule", async () => {
    rewardMint = new Keypair();

    const rewardPerUnix = 0.1;
    const decimals = 9;
    stakingRuleAddress = PublicKey.findProgramAddressSync([Buffer.from("rules"), collectionMint.toBuffer()], programId)[0];

    const signature = await program.methods.createStakingRule(decimals, rewardPerUnix)
    .accounts({
      stakingRules: stakingRuleAddress,
      rewardMint: rewardMint.publicKey,
      collectionMint: collectionMint,
      collectionMetadata: collectionMetadata,
      collectionMasterEdition: collectionMasterEdition,
      initializer: keypair.publicKey,
      tokenMetadataProgram: new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
      systemProgram: SystemProgram.programId,        
    })
    .signers([keypair, rewardMint]).rpc({skipPreflight: true}).then(confirm).then(log);
  });

  it("Create Staking Account", async () => {

    stakingAccountAddress = PublicKey.findProgramAddressSync([Buffer.from("account"), stakingRuleAddress.toBuffer(), keypair.publicKey.toBuffer()], programId)[0];

      const signature = await program.methods.createStakingAccount()
      .accounts({
        stakingRules: stakingRuleAddress,
        stakingAccount: stakingAccountAddress,
        signer: keypair.publicKey,
        systemProgram: SystemProgram.programId,        
      })
      .signers([keypair]).rpc().then(confirm).then(log);

  });

  it("Create NFT with the right Collection", async () => {

    // Metaplex Setup
    const umi = createUmi(connection.rpcEndpoint);
    let umiKeypair = umi.eddsa.createKeypairFromSecretKey(keypair.secretKey);
    const signerKeypair = createSignerFromKeypair(umi, umiKeypair);
    umi.use(signerIdentity(signerKeypair));
    umi.use(mplTokenMetadata())
    const mint = generateSigner(umi);
    nftMint = new PublicKey(mint.publicKey)

    let umiCollectionAddress = publicKey(collectionMint)

    // Create NFT
    let minttx = createNft(
      umi, 
      {
        mint: mint,
        authority: signerKeypair,
        updateAuthority: umiKeypair.publicKey,
        name: "NFT Example",
        symbol: "EXM",
        uri: "",
        sellerFeeBasisPoints: percentAmount(0),
        creators: [
            {address: umiKeypair.publicKey, verified: true, share: 100 }
        ],
        collection: {verified: false, key: umiCollectionAddress},
        uses: null,
        isMutable: true,
        collectionDetails: null,
      }
    );

    await minttx.sendAndConfirm(umi, {
      send: {
        skipPreflight: true
      },
      confirm: {
        commitment
      }
    });

    // Create Collection Accounts
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      nftMint,
      keypair.publicKey
    );

    nftATA = ata.address;

    const metadata_seeds = [
      Buffer.from('metadata'),
      new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID).toBuffer(),
      new PublicKey(mint.publicKey).toBuffer(),
    ];

    const master_edition_seeds = [
      ...metadata_seeds,
      Buffer.from("edition")
    ];

    nftMetadata = PublicKey.findProgramAddressSync(metadata_seeds, new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID))[0];
    nftMasterEdition = PublicKey.findProgramAddressSync(master_edition_seeds, new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID))[0]; 
  });

  it ("Verify Collection", async () => {

    // Metaplex Setup
    const umi = createUmi(connection.rpcEndpoint);
    let umiKeypair = umi.eddsa.createKeypairFromSecretKey(keypair.secretKey);
    const signerKeypair = createSignerFromKeypair(umi, umiKeypair);
    umi.use(signerIdentity(signerKeypair));
    umi.use(mplTokenMetadata())

    let umiNftMetadata = publicKey(nftMetadata)
    let umiCollectionAddress = publicKey(collectionMint)
    let umiCollectionMetatdata = publicKey(collectionMetadata)
    let umiCollectionMasterEdition = publicKey(collectionMasterEdition)

    // Verify the NFT in the collection
    let verifytx = verifyCollection(
      umi, 
      {
        metadata: umiNftMetadata,
        collectionAuthority: signerKeypair,
        collectionMint: umiCollectionAddress,
        collection: umiCollectionMetatdata,
        collectionMasterEditionAccount: umiCollectionMasterEdition,
      }
    );

    await verifytx.sendAndConfirm(umi, {
      send: {
        skipPreflight: true
      },
      confirm: {
        commitment
      }
    });
  });

  it("Stake", async () => {

    stakingInstanceAddress = PublicKey.findProgramAddressSync([Buffer.from("instance"), keypair.publicKey.toBuffer(), nftMint.toBuffer()], programId)[0];

    const signature = await program.methods.stake()
    .accounts({
      stakingRules: stakingRuleAddress,
      stakingAccount: stakingAccountAddress,
      stakingInstance: stakingInstanceAddress,
      nftMint: nftMint,
      nftMetadata: nftMetadata,
      nftMasterEdition: nftMasterEdition,
      signer: keypair.publicKey,
      nftAta: nftATA,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenMetadataProgram: new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,        
    })
    .signers([keypair]).rpc().then(confirm).then(log);

  });

  it("Claim", async () => {

    let ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      rewardMint.publicKey,
      keypair.publicKey
    );

    rewardATA = ata.address;

    await wait(10000);

    const signature = await program.methods.claim()
    .accounts({
      stakingRules: stakingRuleAddress,
      stakingAccount: stakingAccountAddress,
      stakingInstance: stakingInstanceAddress,
      rewardMint: rewardMint.publicKey,
      nftMint: nftMint,
      nftMetadata: nftMetadata,
      nftMasterEdition: nftMasterEdition,
      signer: keypair.publicKey,
      tokenAta: rewardATA,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenMetadataProgram: new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
      systemProgram: SystemProgram.programId,        
    })
    .signers([keypair]).rpc().then(confirm).then(log);
  });

  it("Unstake", async () => {
    
    await wait(10000);

    const signature = await program.methods.unstake()
    .accounts({
      stakingRules: stakingRuleAddress,
      stakingAccount: stakingAccountAddress,
      stakingInstance: stakingInstanceAddress,
      rewardMint: rewardMint.publicKey,
      nftMint: nftMint,
      nftMetadata: nftMetadata,
      nftMasterEdition: nftMasterEdition,
      signer: keypair.publicKey,
      nftAta: nftATA,
      tokenAta: rewardATA,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenMetadataProgram: new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,     
    })
    .signers([keypair]).rpc({skipPreflight: true}).then(confirm).then(log);
  });

});
