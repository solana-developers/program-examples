import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IDL, NftStaking } from "../target/types/nft_staking";
import { PublicKey, 
  Commitment,
  SystemProgram, 
  LAMPORTS_PER_SOL } from "@solana/web3.js";

import {
  createProgrammableNft, 
  createNft,
  mplTokenMetadata,
  verifyCollection, 
  verifyCollectionV1,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import {Connection} from "@solana/web3.js";

import { MPL_TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { base58 } from "@metaplex-foundation/umi/serializers";


import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, generateSigner, percentAmount, publicKey, signerIdentity } from "@metaplex-foundation/umi";

describe("nft-staking", () => {
  const commitment: Commitment = "confirmed"; // processed, confirmed, finalized
  const connection = new Connection("http://localhost:8899", {
      commitment,
      wsEndpoint: "ws://localhost:8900/",
  });
  const keypair = anchor.web3.Keypair.generate();
  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(keypair), { commitment });
  const programId = new PublicKey("GbJ8poY67YD19EWwP6sbo5TVMzCuoMBKgm6jqhGfNzLB");
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

  // Variable
  let collectionMint: PublicKey;
  let collectionMetadata: PublicKey;
  let collectionMasterEdition: PublicKey;
  let collectionATA: PublicKey;

  let nftMint: PublicKey;
  let nftMetadata: PublicKey;
  let nftMasterEdition: PublicKey;
  let nftTokenRecord: PublicKey;
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

    const rewardPerUnix = 0.1;
    stakingRuleAddress = PublicKey.findProgramAddressSync([Buffer.from("rules"), collectionMint.toBuffer()], programId)[0];

      const signature = await program.methods.createStakingRule(rewardPerUnix)
      .accounts({
        stakingRule: stakingRuleAddress,
        collectionMint: collectionMint,
        collectionMetadata: collectionMetadata,
        collectionMasterEdition: collectionMasterEdition,
        initializer: keypair.publicKey,
        tokenMetadataProgram: new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
        systemProgram: SystemProgram.programId,        
      })
      .signers([keypair]).rpc().then(confirm).then(log);

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

  it("Create NFT", async () => {

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
    let minttx = createProgrammableNft(
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

    const result = await minttx.sendAndConfirm(umi, {
      send: {
        skipPreflight: true
      },
      confirm: {
        commitment
      }
    });

    const signature = base58.deserialize(result.signature);
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature[0]}?cluster=custom&customUrl=${connection.rpcEndpoint}`)

    // Create NFT Accounts
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

    const token_record_seeds = [
      Buffer.from("metadata"),
      new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID).toBuffer(),
      new PublicKey(mint.publicKey).toBuffer(),
      Buffer.from("token_record"),
      new PublicKey(nftATA).toBuffer(),
    ];

    nftMetadata = PublicKey.findProgramAddressSync(metadata_seeds, new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID))[0];
    nftMasterEdition = PublicKey.findProgramAddressSync(master_edition_seeds, new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID))[0]; 
    nftTokenRecord = PublicKey.findProgramAddressSync(token_record_seeds, new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID))[0];
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
    let verifytx = verifyCollectionV1(
      umi, 
      {
        metadata: umiNftMetadata,
        collectionMint: umiCollectionAddress,
      }
    );

    const result = await verifytx.sendAndConfirm(umi, {
      send: {
        skipPreflight: true
      },
      confirm: {
        commitment
      }
    });

    const signature = base58.deserialize(result.signature);
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature[0]}?cluster=custom&customUrl=${connection.rpcEndpoint}`)
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
      nftTokenRecord: nftTokenRecord,
      signer: keypair.publicKey,
      signerAta: nftATA,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenMetadataProgram: new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      systemProgram: SystemProgram.programId,        
    })
    .signers([keypair]).rpc().then(confirm).then(log);

  });

  it("Claim", async () => {
    //await wait(10000);

    const signature = await program.methods.claim()
    .accounts({
      stakingRules: stakingRuleAddress,
      stakingAccount: stakingAccountAddress,
      stakingInstance: stakingInstanceAddress,
      nftMint: nftMint,
      nftMetadata: nftMetadata,
      nftMasterEdition: nftMasterEdition,
      signer: keypair.publicKey,
      tokenMetadataProgram: new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
      systemProgram: SystemProgram.programId,        
    })
    .signers([keypair]).rpc().then(confirm).then(log);
  });

  it("Unstake", async () => {
    
    //await wait(10000);

    const signature = await program.methods.unstake()
    .accounts({
      stakingRules: stakingRuleAddress,
      stakingAccount: stakingAccountAddress,
      stakingInstance: stakingInstanceAddress,
      nftMint: nftMint,
      nftMetadata: nftMetadata,
      nftMasterEdition: nftMasterEdition,
      nftTokenRecord: nftTokenRecord,
      signer: keypair.publicKey,
      signerAta: nftATA,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenMetadataProgram: new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      systemProgram: SystemProgram.programId,        
    })
    .signers([keypair]).rpc().then(confirm).then(log);
  });

});
