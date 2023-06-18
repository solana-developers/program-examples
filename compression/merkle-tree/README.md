
[Solana](https://solana.com) has a huge problem. Compressed NFTs are way too cheap!

# Compressed NFTs

Compressed NFTs are a new class of non-fungible tokens that utilize compression techniques to reduce the size of the digital assets they represent. By compressing the underlying data, these NFTs allow developers to store more information within the blockchain at a lower cost, while maintaining the integrity and uniqueness of the asset.

## Working of Compressed NFTs

Unlike traditional NFTs, which store all their metadata directly on the blockchain, compressed NFTs store their metadata in a special structure called a Merkle tree. This Merkle tree has a root, which is stored on the blockchain, and leaves, which are stored off-chain in the Solana ledger.

When a compressed NFT is created and confirmed on the blockchain, Solana RPC providers are used to handle and store the data associated with the NFT off-chain. This approach helps to reduce the storage costs on the blockchain.

To manage the data and enable efficient data queries between RPC providers and on-chain smart contracts, compressed NFTs use indexers. These indexers help organize and retrieve transaction data related to the compressed NFTs. It's important to note that existing smart contracts need to be modified to interact with compressed NFTs. However, if necessary, compressed NFTs can be decompressed to work with unmodified Solana programs.

## How do NFTs get compressed?

1. The Bubblegum program, developed by Metaplex, plays a role in verifying the metadata (information) linked to an NFT (non-fungible token).

2. When Bubblegum is activated, it uses a process called "account-compression" to add a new data point (called a "leaf") to the Merkle tree structure.

3. This "account-compression" process updates the Merkle tree on the Solana blockchain, reflecting the latest state of the NFT ecosystem.

4. Whenever there are changes to the Merkle tree, such as the addition of new NFTs, these changes are recorded and written onto the Solana blockchain for everyone to see and verify.

Compressed NFT metadata is stored in a special structure called a Merkle tree, but this tree is not directly on the Solana blockchain. Instead, it is stored off-chain using Solana RPC service providers. These service providers help manage and store the metadata data for the compressed NFTs.

To make it more cost-effective, the actual data is stored off-chain, while only a small proof of that data, called the Merkle tree root, is stored on the Solana blockchain. This means that the blockchain contains a compact representation of the entire dataset.

By storing the data off-chain and keeping only the Merkle tree root on-chain, the cost of storing and processing the NFT metadata is significantly reduced. This allows for more efficient management of compressed NFTs while still maintaining the integrity and security of the data

## Cost of Compressed NFTs

In fact, the larger the collection of NFTs, the greater the reduction in minting costs with compression, as demonstrated in the image below:

![](https://assets-global.website-files.com/5f973c97cf5aea614f93a26c/63c92ed90d7c51169250d500_SpfybawohX9MOEVXckdnnSXCJYgYjt9LeoF5Em02yZDypwdK8F06LdfEGb1iyWUiJmCPQ017IfxhDZo5Mt6c-OMwX1V4jvRzy-C_K3JyLKS9yE4kbCUji3mXb0hoHEszj5603Jrgl3bPQLxaoI9dXiZBzuDhByyO7sk_dw-P7xU9Yd2c4bdTS4p2SQ-x7w.png)

Because the most significant outlay when minting conventional NFTs is paying for storage space on Solana, which compressed NFTs remove, the majority of the remaining costs are simple transaction fees.

## Minting Compressed NFTs

To mint the Compressed NFTs, first we have to store the metadata by creating a Merkle Tree. 
1. For the **Merkle Tree** Account, generate a Keypair that starts with TRE (just for convenience, you can go on with any account).

```bash
solana-keygen grind --starts-with TRE:1
```

This will create an account into the directory that starts with *TRE*

2. Similarly, create accounts using solana-cli for compressed NFTs and Collection as well.
3. Replace the `TREE` with the keypair for Merkle Tree, `COLL` with your collection keypair and `CNFT` with your keypair that will hold these compressed NFTs.
4. Now, you can create the Merkle tree by running - 
   
```bash
ts-node createTree.ts
```