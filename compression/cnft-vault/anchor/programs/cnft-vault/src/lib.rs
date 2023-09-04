#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use mpl_bubblegum::state::TreeConfig;
use solana_program::pubkey::Pubkey;
use spl_account_compression::{program::SplAccountCompression, Noop};

declare_id!("CNftyK7T8udPwYRzZUMWzbh79rKrz9a5GwV2wv7iEHpk");

#[derive(Clone)]
pub struct MplBubblegum;

impl anchor_lang::Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::id()
    }
}

// first 8 bytes of SHA256("global:transfer")
const TRANSFER_DISCRIMINATOR: &[u8; 8] = &[163, 52, 200, 231, 140, 3, 69, 186];

#[program]
pub mod cnft_vault {

    use super::*;

    pub fn withdraw_cnft<'info>(
        ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        msg!(
            "attempting to send nft {} from tree {}",
            index,
            ctx.accounts.merkle_tree.key()
        );

        let mut accounts: Vec<solana_program::instruction::AccountMeta> = vec![
            AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), false),
            AccountMeta::new_readonly(ctx.accounts.new_leaf_owner.key(), false),
            AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
            AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
            AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ];

        let mut data: Vec<u8> = vec![];
        data.extend(TRANSFER_DISCRIMINATOR);
        data.extend(root);
        data.extend(data_hash);
        data.extend(creator_hash);
        data.extend(nonce.to_le_bytes());
        data.extend(index.to_le_bytes());

        let mut account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.tree_authority.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
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
            &solana_program::instruction::Instruction {
                program_id: ctx.accounts.bubblegum_program.key(),
                accounts,
                data,
            },
            &account_infos[..],
            &[&[b"cNFT-vault", &[*ctx.bumps.get("leaf_owner").unwrap()]]],
        )
        .map_err(Into::into)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn withdraw_two_cnfts<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawTwo<'info>>,
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
        _proof_2_length: u8, // we don't actually need this (proof_2_length = remaining_accounts_len - proof_1_length)
    ) -> Result<()> {
        let merkle_tree1 = ctx.accounts.merkle_tree1.key();
        let merkle_tree2 = ctx.accounts.merkle_tree2.key();
        msg!(
            "attempting to send nfts from trees {} and {}",
            merkle_tree1,
            merkle_tree2
        );

        // Note: in this example anyone can withdraw any NFT from the vault
        // in productions you should check if nft transfers are valid (correct NFT, correct authority)

        let mut accounts1: Vec<solana_program::instruction::AccountMeta> = vec![
            AccountMeta::new_readonly(ctx.accounts.tree_authority1.key(), false),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), false),
            AccountMeta::new_readonly(ctx.accounts.new_leaf_owner1.key(), false),
            AccountMeta::new(ctx.accounts.merkle_tree1.key(), false),
            AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
            AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ];

        let mut accounts2: Vec<solana_program::instruction::AccountMeta> = vec![
            AccountMeta::new_readonly(ctx.accounts.tree_authority2.key(), false),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), false),
            AccountMeta::new_readonly(ctx.accounts.new_leaf_owner2.key(), false),
            AccountMeta::new(ctx.accounts.merkle_tree2.key(), false),
            AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
            AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ];

        let mut data1: Vec<u8> = vec![];
        data1.extend(TRANSFER_DISCRIMINATOR);
        data1.extend(root1);
        data1.extend(data_hash1);
        data1.extend(creator_hash1);
        data1.extend(nonce1.to_le_bytes());
        data1.extend(index1.to_le_bytes());
        let mut data2: Vec<u8> = vec![];
        data2.extend(TRANSFER_DISCRIMINATOR);
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

        for (i, acc) in ctx.remaining_accounts.iter().enumerate() {
            if i < proof_1_length as usize {
                accounts1.push(AccountMeta::new_readonly(acc.key(), false));
                account_infos1.push(acc.to_account_info());
            } else {
                accounts2.push(AccountMeta::new_readonly(acc.key(), false));
                account_infos2.push(acc.to_account_info());
            }
        }

        msg!("withdrawing cNFT#1");
        solana_program::program::invoke_signed(
            &solana_program::instruction::Instruction {
                program_id: ctx.accounts.bubblegum_program.key(),
                accounts: accounts1,
                data: data1,
            },
            &account_infos1[..],
            &[&[b"cNFT-vault", &[*ctx.bumps.get("leaf_owner").unwrap()]]],
        )?;

        msg!("withdrawing cNFT#2");
        solana_program::program::invoke_signed(
            &solana_program::instruction::Instruction {
                program_id: ctx.accounts.bubblegum_program.key(),
                accounts: accounts2,
                data: data2,
            },
            &account_infos2[..],
            &[&[b"cNFT-vault", &[*ctx.bumps.get("leaf_owner").unwrap()]]],
        )?;

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
