import * as anchor from "@coral-xyz/anchor";
import { createNft, findMasterEditionPda, findMetadataPda, mplTokenMetadata, verifySizedCollectionItem } from '@metaplex-foundation/mpl-token-metadata'
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { KeypairSigner, PublicKey, createSignerFromKeypair, generateSigner, keypairIdentity, percentAmount } from '@metaplex-foundation/umi';
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync} from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftStaking as Program<NftStaking>;

  const umi = createUmi(provider.connection);

  const payer = provider.wallet as NodeWallet;

  let nftMint: KeypairSigner = generateSigner(umi);
  let collectionMint: KeypairSigner = generateSigner(umi);

  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(payer.payer.secretKey));
  const creator = createSignerFromKeypair(umi, creatorWallet);
  umi.use(keypairIdentity(creator));
  umi.use(mplTokenMetadata());

  const collection: anchor.web3.PublicKey = new anchor.web3.PublicKey(collectionMint.publicKey.toString());

  const config = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0];

  const rewardsMint = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("rewards"), config.toBuffer()], program.programId)[0];

  const userAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user"), provider.publicKey.toBuffer()], program.programId)[0];

  const stakeAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("stake"), new anchor.web3.PublicKey(nftMint.publicKey as PublicKey).toBuffer(), config.toBuffer()], program.programId)[0];
  
  it("Mint Collection NFT", async () => {
        await createNft(umi, {
            mint: collectionMint,
            name: "GM",
            symbol: "GM",
            uri: "https://arweave.net/123",
            sellerFeeBasisPoints: percentAmount(5.5),
            creators: null,
            collectionDetails: { 
              __kind: 'V1', size: 10,
            }
        }).sendAndConfirm(umi)
        console.log(`Created Collection NFT: ${collectionMint.publicKey.toString()}`)
  });

  it("Mint NFT", async () => {
        await createNft(umi, {
            mint: nftMint,
            name: "GM",
            symbol: "GM",
            uri: "https://arweave.net/123",
            sellerFeeBasisPoints: percentAmount(5.5),
            collection: {verified: false, key: collectionMint.publicKey},
            creators: null,
        }).sendAndConfirm(umi)
        console.log(`\nCreated NFT: ${nftMint.publicKey.toString()}`)
  });

  it("Verify Collection NFT", async () => {
    const collectionMetadata = findMetadataPda(umi, {mint: collectionMint.publicKey});
    const collectionMasterEdition = findMasterEditionPda(umi, {mint: collectionMint.publicKey});

    const nftMetadata = findMetadataPda(umi, {mint: nftMint.publicKey});
    await verifySizedCollectionItem(umi, {
      metadata: nftMetadata,
      collectionAuthority: creator,
      collectionMint: collectionMint.publicKey,
      collection: collectionMetadata,
      collectionMasterEditionAccount: collectionMasterEdition,
     }).sendAndConfirm(umi)
    console.log("\nCollection NFT Verified!")
  });

  it("Initialize Config Account", async () => {
    const tx = await program.methods.initializeConfig(10, 10, 0)
    .accountsPartial({
      admin: provider.wallet.publicKey,
      config,
      rewardsMint,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
    console.log("\nConfig Account Initialized!");
    console.log("Your transaction signature", tx);
  });

  it("Initialize User Account", async() => {
    const tx = await program.methods.initializeUser()
    .accountsPartial({
      user: provider.wallet.publicKey,
      userAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("\nUser Account Initialized!");
    console.log("Your transaction signature", tx);
  });

  it("Stake NFT", async() => {
    const mintAta = getAssociatedTokenAddressSync(new anchor.web3.PublicKey(nftMint.publicKey as PublicKey), provider.wallet.publicKey);

    const nftMetadata = findMetadataPda(umi, {mint: nftMint.publicKey});
    const nftEdition = findMasterEditionPda(umi, {mint: nftMint.publicKey});

    const tx = await program.methods.stake()
    .accountsPartial({
      user: provider.wallet.publicKey,
      mint: nftMint.publicKey,
      collectionMint: collectionMint.publicKey,
      mintAta,
      metadata: new anchor.web3.PublicKey(nftMetadata[0]),
      edition: new anchor.web3.PublicKey(nftEdition[0]),
      config,
      stakeAccount,
      userAccount,
    })
    .rpc();

    console.log("\nNFT Staked!");
    console.log("Your transaction signature", tx);
  })

  it("Unstake NFT", async() => {
    const mintAta = getAssociatedTokenAddressSync(new anchor.web3.PublicKey(nftMint.publicKey as PublicKey), provider.wallet.publicKey);

    const nftEdition = findMasterEditionPda(umi, {mint: nftMint.publicKey});

    const tx = await program.methods.unstake()
    .accountsPartial({
      user: provider.wallet.publicKey,
      mint: nftMint.publicKey,
      mintAta,
      edition: new anchor.web3.PublicKey(nftEdition[0]),
      config,
      stakeAccount,
      userAccount,
    })
    .rpc();

    console.log("\nNFT unstaked!");
    console.log("Your transaction signature", tx);

    let account = await program.account.userAccount.fetch(userAccount)
    console.log("user points: ", account.points);
  })

  it("Claim Rewards", async() => {
    const rewardsAta = getAssociatedTokenAddressSync(rewardsMint, provider.wallet.publicKey);

    const tx = await program.methods.claim()
    .accountsPartial({
      user: provider.wallet.publicKey,
      userAccount,
      rewardsMint,
      config,
      rewardsAta,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .rpc();

    console.log("\nRewards claimed");
    console.log("Your transaction signature", tx);

    let account = await program.account.userAccount.fetch(userAccount)
    console.log("User points: ", account.points);
  })
});
