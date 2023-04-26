# Solana Program cNFT Transfer example

This repo contains example code of how you can work with Metaplex compressed NFTs inside of Solana Anchor programs.

## Components

The Anchor program can be found in the *programs* folder and *app* some typescript node scripts to interact with the program from client side.

## Deployment

The program is deployed on devnet at `CNftyK7T8udPwYRzZUMWzbh79rKrz9a5GwV2wv7iEHpk`. 
You can deploy it yourself by changing the respective values in lib.rs and Anchor.toml.

## Limitations

This is just an example implementation. It is missing all logic wheter a transfer should be performed or not (everyone can withdraw any cNFT in the vault). 
Furthermore it is not optimized for using lowest possible compute. It is intended as a proof of concept and reference implemention only. 

## Further resources

A video about the creation of this code which also contains further explanations has been publised on Solandy's YouTube channel:
https://youtu.be/qzr-q_E7H0M