use anchor_lang::prelude::*;
use solana_program::{pubkey::Pubkey};
use spl_account_compression::{
    program::SplAccountCompression, Noop,
};
use mpl_bubblegum::{state::TreeConfig};

declare_id!("CNftyK7T8udPwYRzZUMWzbh79rKrz9a5GwV2wv7iEHpk");

#[derive(Clone)]
pub struct MplBubblegum;

impl anchor_lang::Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::id()
    }
}

#[program]
pub mod cnft_vault {

    use super::*;

    pub fn withdraw_cnft<'info>(ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,) -> Result<()> {
        msg!("attempting to send nft {} from tree {}", index, ctx.accounts.merkle_tree.key());

        // CPI to bubblegum
        // //attempt 1
        // mpl_bubblegum::cpi::transfer(
        //     CpiContext::new_with_signer(
        //         ctx.accounts.bubblegum_program.to_account_info(), 
        //         mpl_bubblegum::cpi::accounts::Transfer{
        //             tree_authority: ctx.accounts.tree_authority.to_account_info(),
        //             leaf_owner: ctx.accounts.leaf_owner.to_account_info(),
        //             leaf_delegate: ctx.accounts.leaf_delegate.to_account_info(),
        //             new_leaf_owner: ctx.accounts.new_leaf_owner.to_account_info(),
        //             merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        //             log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
        //             compression_program: ctx.accounts.compression_program.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //         }, &[&[b"cNFT-vault", &[*ctx.bumps.get("vault").unwrap()]]]),
        //         root, data_hash, creator_hash, nonce, index)
        
        //attempt 2
        let mut accounts:  Vec<solana_program::instruction::AccountMeta> = vec![
            AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.leaf_delegate.key(), false),
            AccountMeta::new_readonly(ctx.accounts.new_leaf_owner.key(), false),
            AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
            AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
            AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ];

        // first 8 bytes of SHA256("global:transfer")   
        let transfer_discriminator: [u8;8] = [163, 52, 200, 231, 140, 3, 69, 186];//hex::decode("a334c8e78c0345ba").expect("hex decode fail"); 
        //msg!("{:?}", transfer_discriminator);
        
        let mut data: Vec<u8> = vec![];
        data.extend(transfer_discriminator);
        data.extend(root);
        data.extend(data_hash);
        data.extend(creator_hash);
        data.extend(nonce.to_le_bytes());
        data.extend(index.to_le_bytes());

        let mut account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.tree_authority.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.leaf_delegate.to_account_info(),
            ctx.accounts.new_leaf_owner.to_account_info(),
            ctx.accounts.merkle_tree.to_account_info(),
            ctx.accounts.log_wrapper.to_account_info(),
            ctx.accounts.compression_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        
        // add "accounts" (hashes) that make up the merkle proof
        for acc in ctx.remaining_accounts.iter() {
            accounts.push(AccountMeta::new_readonly(acc.key(), false));
            account_infos.push(acc.to_account_info());
        }

        msg!("manual cpi call");
        solana_program::program::invoke_signed(
        & solana_program::instruction::Instruction {
            program_id: ctx.accounts.bubblegum_program.key(),
            accounts: accounts,
            data: data,
        },
        &account_infos[..],
        &[&[b"cNFT-vault", &[*ctx.bumps.get("leaf_owner").unwrap()]]])
        .map_err(Into::into)

        
    }

    
    pub fn withdraw_two_cnfts<'info>(ctx: Context<'_, '_, '_, 'info, WithdrawTwo<'info>>,
        root1: [u8; 32],
        data_hash1: [u8; 32],
        creator_hash1: [u8; 32],
        nonce1: u64,
        index1: u32,
        proof_1_length: u8,
        root2: [u8; 32],
        data_hash2: [u8; 32],
        creator_hash2: [u8; 32],
        nonce2: u64,
        index2: u32,
        _proof_2_length: u8 // we don't actually need this (proof_2_length = remaining_accounts_len - proof_1_length)
    ) -> Result<()> {
        let merkle_tree1 = ctx.accounts.merkle_tree1.key();
        let merkle_tree2 = ctx.accounts.merkle_tree2.key();
        msg!("attempting to send nfts from trees {} and {}", merkle_tree1, merkle_tree2);
        
        // TODO check if nft transfers are even valid (correct NFT, correct authority)
        // in this example anyone can withdraw any NFT from the vault

        let mut accounts1:  Vec<solana_program::instruction::AccountMeta> = vec![
            AccountMeta::new_readonly(ctx.accounts.tree_authority1.key(), false),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), false),
            AccountMeta::new_readonly(ctx.accounts.new_leaf_owner1.key(), false),
            AccountMeta::new(ctx.accounts.merkle_tree1.key(), false),
            AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
            AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ];
        
        let mut accounts2:  Vec<solana_program::instruction::AccountMeta> = vec![
            AccountMeta::new_readonly(ctx.accounts.tree_authority2.key(), false),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), false),
            AccountMeta::new_readonly(ctx.accounts.new_leaf_owner2.key(), false),
            AccountMeta::new(ctx.accounts.merkle_tree2.key(), false),
            AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
            AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ];

        let transfer_discriminator: [u8;8] = [163, 52, 200, 231, 140, 3, 69, 186];

        let mut data1: Vec<u8> = vec![];
        data1.extend(&transfer_discriminator);
        data1.extend(root1);
        data1.extend(data_hash1);
        data1.extend(creator_hash1);
        data1.extend(nonce1.to_le_bytes());
        data1.extend(index1.to_le_bytes());
        let mut data2: Vec<u8> = vec![];
        data2.extend(&transfer_discriminator);
        data2.extend(root2);
        data2.extend(data_hash2);
        data2.extend(creator_hash2);
        data2.extend(nonce2.to_le_bytes());
        data2.extend(index2.to_le_bytes());

        let mut account_infos1: Vec<AccountInfo> = vec![
            ctx.accounts.tree_authority1.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.new_leaf_owner1.to_account_info(),
            ctx.accounts.merkle_tree1.to_account_info(),
            ctx.accounts.log_wrapper.to_account_info(),
            ctx.accounts.compression_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        let mut account_infos2: Vec<AccountInfo> = vec![
            ctx.accounts.tree_authority2.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.new_leaf_owner2.to_account_info(),
            ctx.accounts.merkle_tree2.to_account_info(),
            ctx.accounts.log_wrapper.to_account_info(),
            ctx.accounts.compression_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        
        // add "accounts" (hashes) that make up the merkle proof
        let mut i = 0u8;
        for acc in ctx.remaining_accounts.iter() {
            if i < proof_1_length {
                accounts1.push(AccountMeta::new_readonly(acc.key(), false));
                account_infos1.push(acc.to_account_info());
            } else {
                accounts2.push(AccountMeta::new_readonly(acc.key(), false));
                account_infos2.push(acc.to_account_info());
            }
            i+=1;
        }

        msg!("withdrawing cNFT#1");
        solana_program::program::invoke_signed(
        & solana_program::instruction::Instruction {
            program_id: ctx.accounts.bubblegum_program.key(),
            accounts: accounts1,
            data: data1,
        },
        &account_infos1[..],
        &[&[b"cNFT-vault", &[*ctx.bumps.get("leaf_owner").unwrap()]]])?;
        
        msg!("withdrawing cNFT#2");
        solana_program::program::invoke_signed(
        & solana_program::instruction::Instruction {
            program_id: ctx.accounts.bubblegum_program.key(),
            accounts: accounts2,
            data: data2,
        },
        &account_infos2[..],
        &[&[b"cNFT-vault", &[*ctx.bumps.get("leaf_owner").unwrap()]]])?;

        msg!("successfully sent cNFTs");
        Ok(())

        
    }

}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [merkle_tree.key().as_ref()],
        bump, 
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is neither written to nor read from.
    pub tree_authority: Account<'info, TreeConfig>,
    
    #[account(
        seeds = [b"cNFT-vault"],
        bump,
    )]
    /// CHECK: This account doesnt even exist (it is just the pda to sign)
    pub leaf_owner: UncheckedAccount<'info>, // sender (the vault in our case)
    /// CHECK: This account is chekced in the instruction
    pub leaf_delegate: UncheckedAccount<'info>, // we could actually remove this and just use leaf_owner instead
    /// CHECK: This account is neither written to nor read from.
    pub new_leaf_owner: UncheckedAccount<'info>, // receiver
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawTwo<'info> {
    #[account(
        seeds = [merkle_tree1.key().as_ref()],
        bump, 
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is neither written to nor read from.
    pub tree_authority1: Account<'info, TreeConfig>,
    #[account(
        seeds = [b"cNFT-vault"],
        bump,
    )]
    /// CHECK: This account doesnt even exist (it is just the pda to sign)
    pub leaf_owner: UncheckedAccount<'info>, // you might need two accounts if the nfts are owned by two different PDAs
    /// CHECK: This account is neither written to nor read from.
    pub new_leaf_owner1: UncheckedAccount<'info>, // receiver
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree1: UncheckedAccount<'info>,
    
    #[account(
        seeds = [merkle_tree2.key().as_ref()],
        bump, 
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is neither written to nor read from.
    pub tree_authority2: Account<'info, TreeConfig>,
    /// CHECK: This account is neither written to nor read from.
    pub new_leaf_owner2: UncheckedAccount<'info>, // receiver
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree2: UncheckedAccount<'info>,

    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}

