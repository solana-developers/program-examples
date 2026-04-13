use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};
use borsh::BorshSerialize;

declare_id!("C6qxH8n6mZxrrbtMtYWYSp8JR8vkQ55X1o4EBg7twnMv");

/// mpl-bubblegum program ID (BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY)
const MPL_BUBBLEGUM_ID: Pubkey = Pubkey::new_from_array([
    0x98, 0x8b, 0x80, 0xeb, 0x79, 0x35, 0x28, 0x69, 0xb2, 0x24, 0x74, 0x5f, 0x59, 0xdd, 0xbf,
    0x8a, 0x26, 0x58, 0xca, 0x13, 0xdc, 0x68, 0x81, 0x21, 0x26, 0x35, 0x1c, 0xae, 0x07, 0xc1,
    0xa5, 0xa5,
]);

/// SPL Account Compression program ID (cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK)
const SPL_ACCOUNT_COMPRESSION_ID: Pubkey = Pubkey::new_from_array([
    0x09, 0x2a, 0x13, 0xee, 0x95, 0xc4, 0x1c, 0xba, 0x08, 0xa6, 0x7f, 0x5a, 0xc6, 0x7e, 0x8d,
    0xf7, 0xe1, 0xda, 0x11, 0x62, 0x5e, 0x1d, 0x64, 0x13, 0x7f, 0x8f, 0x4f, 0x23, 0x83, 0x03,
    0x7f, 0x14,
]);

/// Burn instruction discriminator from mpl-bubblegum
const BURN_DISCRIMINATOR: [u8; 8] = [116, 110, 29, 56, 107, 219, 42, 93];

/// Instruction arguments for mpl-bubblegum Burn, serialized with borsh
#[derive(BorshSerialize)]
struct BurnArgs {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
}

#[derive(Clone)]
pub struct SPLCompression;

impl anchor_lang::Id for SPLCompression {
    fn id() -> Pubkey {
        SPL_ACCOUNT_COMPRESSION_ID
    }
}

#[program]
pub mod cnft_burn {
    use super::*;

    pub fn burn_cnft<'info>(
        context: Context<'info, BurnCnft<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        // Build instruction data: discriminator + borsh-serialized args
        let args = BurnArgs {
            root,
            data_hash,
            creator_hash,
            nonce,
            index,
        };
        let mut data = BURN_DISCRIMINATOR.to_vec();
        args.serialize(&mut data)?;

        // Build account metas matching mpl-bubblegum Burn instruction layout
        let mut accounts = Vec::with_capacity(7 + context.remaining_accounts.len());
        accounts.push(AccountMeta::new_readonly(
            context.accounts.tree_authority.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.leaf_owner.key(),
            true,
        ));
        // leaf_delegate = leaf_owner, not a signer in this call
        accounts.push(AccountMeta::new_readonly(
            context.accounts.leaf_owner.key(),
            false,
        ));
        accounts.push(AccountMeta::new(context.accounts.merkle_tree.key(), false));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.log_wrapper.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.compression_program.key(),
            false,
        ));
        accounts.push(AccountMeta::new_readonly(
            context.accounts.system_program.key(),
            false,
        ));
        // Append remaining accounts (proof nodes)
        for acc in context.remaining_accounts.iter() {
            accounts.push(AccountMeta::new_readonly(acc.key(), false));
        }

        let instruction = Instruction {
            program_id: MPL_BUBBLEGUM_ID,
            accounts,
            data,
        };

        // Gather all account infos for the CPI
        let mut account_infos = vec![
            context.accounts.bubblegum_program.to_account_info(),
            context.accounts.tree_authority.to_account_info(),
            context.accounts.leaf_owner.to_account_info(),
            context.accounts.merkle_tree.to_account_info(),
            context.accounts.log_wrapper.to_account_info(),
            context.accounts.compression_program.to_account_info(),
            context.accounts.system_program.to_account_info(),
        ];
        for acc in context.remaining_accounts.iter() {
            account_infos.push(acc.to_account_info());
        }

        invoke(&instruction, &account_infos)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnCnft<'info> {
    #[account(mut)]
    pub leaf_owner: Signer<'info>,
    #[account(mut)]
    #[account(
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is modified in the downstream program
    pub tree_authority: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is neither written to nor read from.
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub log_wrapper: UncheckedAccount<'info>,
    pub compression_program: Program<'info, SPLCompression>,
    /// CHECK: This account is neither written to nor read from.
    pub bubblegum_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
