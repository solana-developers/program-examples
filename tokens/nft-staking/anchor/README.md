# Staking Program Documentation

This document outlines the structure and functionality of the staking program you'll find in the folders. We'll talk about States, Account and Instruction. If you have any question please reach out to @L0STE_ on twitter. 

## States:

The program is organized into three distinct states: `StakingRules`, `StakingAccount`, and `StakingInstance`. Each state plays a crucial role in ensuring the security and precision of the staking operations. For each of this we implemented a value of space to render the initialization easy and fast.

### StakingRules

This state is fundamental in establishing the groundwork for staking. It is created by an authority figure (that we later save it in case we want to create an instruction to change some of this parameter), who is responsible for setting staking rewards, and determining the collection_address that should receive this staking rewards (The collection_address is later validated within the smart contract to ensure legitimacy).

The structure of StakingRules is as follows:

```rust
pub struct StakingRules {
    pub authority: Pubkey,
    pub collection_address: Pubkey,
    pub reward_per_unix: f64,
    pub bump: u8,
}
```

In the case of the staking_for_token we put the reward_mint field to validate later that we are indeed minting a token reserved for that staking rule.

### StakingAccount

This state is derived directly from `StakingRules`. This State is used to maintains information about individual staker, like how many points have been rewarded to him and is used later to create the `StakingInstance`

It records the owner's identity and the applicable staking rules. The purpose of this state is to establish a clear link between the owner, the staking rules they are subscribing to so we are sure about the rewards they are entitled to.

```rust
pub struct StakingAccount {
    pub owner: Pubkey,
    pub staking_rules: Pubkey,
    pub point_rewarded: u64,
    pub bump: u8
}
```

### StakingInstance

This state represents a unique instance of staking for a particular NFT. For each NFT staked, a new `StakingInstance` is created, ensuring a one-to-one relationship between the staked NFT and its staking record. This approach adds an extra layer of security and clarity, as it precisely tracks when each NFT was staked, preventing any retroactive changes or misunderstandings. The structure of StakingInstance is:

```rust
pub struct StakingInstance {
    pub staking_account: Pubkey,
    pub staking_rules: Pubkey,
    pub time: i64,    
    pub bump: u8
}
```

## Instruction & Accounts:

### Create StakingAccount & StakingRules

This Instruction are just used to populate the `StakingAccount` and `StakingRule` states.

### Stake

This Instruction is used to stake the Nft. 

First we check if this is an NFT by checking that the master_edition is the authorithy of this mint; then we check if the collection of this mint is the one we have in the metadata: we check that the collection field is populated, then if it's verified and then that the publickey in there is the same of the staking rule one. 

```rust
#[account(
    mint::authority = nft_master_edition,
)]
pub nft_mint: Account<'info, Mint>,
#[account(
    seeds = [
        b"metadata",
        token_metadata_program.key().as_ref(),
        nft_mint.key().as_ref()
    ],
    seeds::program = token_metadata_program.key(),
    bump,
    constraint = nft_metadata.collection.is_some(),
    constraint = nft_metadata.collection.as_ref().unwrap().verified,
    constraint = nft_metadata.collection.as_ref().unwrap().key == staking_rules.collection_address @StakingErrors::InvalidCollection,
)]
pub nft_metadata: Account<'info, MetadataAccount>,
#[account(
    seeds = [
        b"metadata",
        token_metadata_program.key().as_ref(),
        nft_mint.key().as_ref(),
        b"edition",
        ],
    seeds::program = token_metadata_program.key(),
    bump,
)]
pub nft_master_edition: Account<'info, MasterEditionAccount>,
```

Once we checked all the parameters needee we start by Delegate the authority on the NFT using a simple approve function to the `StakingInstance` account: 

```rust
let cpi_program = self.token_program.to_account_info();
let cpi_accounts = Approve {
    to: self.signer_ata.to_account_info(),
    delegate: self.staking_rules.to_account_info(),
    authority:self.signer.to_account_info(),
};
let cpi_context = CpiContext::new(cpi_program, cpi_accounts);  

approve(cpi_context, 1)?;
```

Once that's delegated we freeze it in place using the new authority of the NFT: the `StakingInstance` (that's why we use new_with_signer & signer_seeds).

```rust
let cpi_program = self.token_metadata_program.to_account_info();
let cpi_accounts = FreezeDelegatedAccount {
    metadata: self.nft_metadata.to_account_info(),
    delegate: self.staking_rules.to_account_info(),
    token_account: self.signer_ata.to_account_info(),
    edition: self.nft_master_edition.to_account_info(), // is it the master edition?
    mint: self.nft_mint.to_account_info(),
    token_program: self.token_program.to_account_info(),
};
let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

freeze_delegated_account(cpi_context)?;
```

And we finish by populating the `StakingInstance` account.

### Claim

This instruction is a way to get the points and token withouth having the need of unstaking the NFT before. 

To do so we simply change the time of staking in the `StakingInstance` account and we reward the user by accrediting that points to the `StakingAccount` 

### Unstake

This Instruction is used to unstake the Nft and get the reward. 

Once we checked all the parameters needed we unfreeze the NFT using a thaw_delegated_account function that we sign with the `StakingInstance` since we gave them the delegation to do so.

```rust
let cpi_program = self.token_metadata_program.to_account_info();
let cpi_accounts = ThawDelegatedAccount {
    metadata: self.nft_metadata.to_account_info(),
    delegate: self.staking_rules.to_account_info(),
    token_account: self.signer_ata.to_account_info(),
    edition: self.nft_master_edition.to_account_info(),
    mint: self.nft_mint.to_account_info(),
    token_program: self.token_program.to_account_info(),
};
let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

thaw_delegated_account(cpi_context)?;
```

Once we unstake we revoke the delegation to the `StakingInstance` account and we close it

```rust
let cpi_program = self.token_program.to_account_info();
let cpi_accounts = Revoke {
    source: self.signer_ata.to_account_info(),
    authority:self.signer.to_account_info(),
};
let cpi_context = CpiContext::new(cpi_program, cpi_accounts);  

revoke(cpi_context)?;
```

```rust
#[account(
    ...,
    close = signer,
    ...
)]
pub staking_instance: Account<'info, StakingInstance>,
```