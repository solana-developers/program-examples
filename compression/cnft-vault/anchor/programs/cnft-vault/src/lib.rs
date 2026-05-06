use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};
use borsh::BorshSerialize;

declare_id!("Fd4iwpPWaCU8BNwGQGtvvrcvG4Tfizq3RgLm8YLBJX6D");

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

/// Transfer instruction discriminator from mpl-bubblegum
const TRANSFER_DISCRIMINATOR: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

/// Instruction arguments for mpl-bubblegum Transfer, serialized with borsh
#[derive(BorshSerialize)]
struct TransferArgs {
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

/// Build a mpl-bubblegum Transfer instruction from pubkeys and args.
/// This avoids using mpl-bubblegum's CPI wrapper which requires solana-program 2.x AccountInfo.
fn build_transfer_instruction(
    tree_config: Pubkey,
    leaf_owner: Pubkey,
    leaf_delegate: Pubkey,
    new_leaf_owner: Pubkey,
    merkle_tree: Pubkey,
    log_wrapper: Pubkey,
    compression_program: Pubkey,
    system_program: Pubkey,
    remaining_accounts: &[AccountMeta],
    args: TransferArgs,
) -> Result<Instruction> {
    let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
    accounts.push(AccountMeta::new_readonly(tree_config, false));
    // leaf_owner is a signer (PDA signs via invoke_signed)
    accounts.push(AccountMeta::new_readonly(leaf_owner, true));
    // leaf_delegate = leaf_owner, not an additional signer
    accounts.push(AccountMeta::new_readonly(leaf_delegate, false));
    accounts.push(AccountMeta::new_readonly(new_leaf_owner, false));
    accounts.push(AccountMeta::new(merkle_tree, false));
    accounts.push(AccountMeta::new_readonly(log_wrapper, false));
    accounts.push(AccountMeta::new_readonly(compression_program, false));
    accounts.push(AccountMeta::new_readonly(system_program, false));
    accounts.extend_from_slice(remaining_accounts);

    let mut data = TRANSFER_DISCRIMINATOR.to_vec();
    args.serialize(&mut data)?;

    Ok(Instruction {
        program_id: MPL_BUBBLEGUM_ID,
        accounts,
        data,
    })
}

#[program]
pub mod cnft_vault {
    use super::*;

    pub fn withdraw_cnft<'info>(
        context: Context<'info, Withdraw<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        msg!(
            "attempting to send nft {} from tree {}",
            index,
            context.accounts.merkle_tree.key()
        );

        let proof_metas: Vec<AccountMeta> = context
            .remaining_accounts
            .iter()
            .map(|acc| AccountMeta::new_readonly(acc.key(), false))
            .collect();

        let instruction = build_transfer_instruction(
            context.accounts.tree_authority.key(),
            context.accounts.leaf_owner.key(),
            context.accounts.leaf_owner.key(),
            context.accounts.new_leaf_owner.key(),
            context.accounts.merkle_tree.key(),
            context.accounts.log_wrapper.key(),
            context.accounts.compression_program.key(),
            context.accounts.system_program.key(),
            &proof_metas,
            TransferArgs {
                root,
                data_hash,
                creator_hash,
                nonce,
                index,
            },
        )?;

        // Gather all account infos for the CPI
        let mut account_infos = vec![
            context.accounts.bubblegum_program.to_account_info(),
            context.accounts.tree_authority.to_account_info(),
            context.accounts.leaf_owner.to_account_info(),
            context.accounts.new_leaf_owner.to_account_info(),
            context.accounts.merkle_tree.to_account_info(),
            context.accounts.log_wrapper.to_account_info(),
            context.accounts.compression_program.to_account_info(),
            context.accounts.system_program.to_account_info(),
        ];
        for acc in context.remaining_accounts.iter() {
            account_infos.push(acc.to_account_info());
        }

        invoke_signed(
            &instruction,
            &account_infos,
            &[&[b"cNFT-vault", &[context.bumps.leaf_owner]]],
        )?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn withdraw_two_cnfts<'info>(
        context: Context<'info, WithdrawTwo<'info>>,
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
        _proof_2_length: u8,
    ) -> Result<()> {
        let merkle_tree1 = context.accounts.merkle_tree1.key();
        let merkle_tree2 = context.accounts.merkle_tree2.key();
        msg!(
            "attempting to send nfts from trees {} and {}",
            merkle_tree1,
            merkle_tree2
        );

        let signer_seeds: &[&[u8]] = &[b"cNFT-vault", &[context.bumps.leaf_owner]];

        // Split remaining accounts into proof1 and proof2
        let (proof1_accounts, proof2_accounts) =
            context.remaining_accounts.split_at(proof_1_length as usize);

        let proof1_metas: Vec<AccountMeta> = proof1_accounts
            .iter()
            .map(|acc| AccountMeta::new_readonly(acc.key(), false))
            .collect();

        let proof2_metas: Vec<AccountMeta> = proof2_accounts
            .iter()
            .map(|acc| AccountMeta::new_readonly(acc.key(), false))
            .collect();

        // Withdraw cNFT#1
        msg!("withdrawing cNFT#1");
        let instruction1 = build_transfer_instruction(
            context.accounts.tree_authority1.key(),
            context.accounts.leaf_owner.key(),
            context.accounts.leaf_owner.key(),
            context.accounts.new_leaf_owner1.key(),
            context.accounts.merkle_tree1.key(),
            context.accounts.log_wrapper.key(),
            context.accounts.compression_program.key(),
            context.accounts.system_program.key(),
            &proof1_metas,
            TransferArgs {
                root: root1,
                data_hash: data_hash1,
                creator_hash: creator_hash1,
                nonce: nonce1,
                index: index1,
            },
        )?;

        let mut account_infos1 = vec![
            context.accounts.bubblegum_program.to_account_info(),
            context.accounts.tree_authority1.to_account_info(),
            context.accounts.leaf_owner.to_account_info(),
            context.accounts.new_leaf_owner1.to_account_info(),
            context.accounts.merkle_tree1.to_account_info(),
            context.accounts.log_wrapper.to_account_info(),
            context.accounts.compression_program.to_account_info(),
            context.accounts.system_program.to_account_info(),
        ];
        for acc in proof1_accounts.iter() {
            account_infos1.push(acc.to_account_info());
        }

        invoke_signed(&instruction1, &account_infos1, &[signer_seeds])?;

        // Withdraw cNFT#2
        msg!("withdrawing cNFT#2");
        let instruction2 = build_transfer_instruction(
            context.accounts.tree_authority2.key(),
            context.accounts.leaf_owner.key(),
            context.accounts.leaf_owner.key(),
            context.accounts.new_leaf_owner2.key(),
            context.accounts.merkle_tree2.key(),
            context.accounts.log_wrapper.key(),
            context.accounts.compression_program.key(),
            context.accounts.system_program.key(),
            &proof2_metas,
            TransferArgs {
                root: root2,
                data_hash: data_hash2,
                creator_hash: creator_hash2,
                nonce: nonce2,
                index: index2,
            },
        )?;

        let mut account_infos2 = vec![
            context.accounts.bubblegum_program.to_account_info(),
            context.accounts.tree_authority2.to_account_info(),
            context.accounts.leaf_owner.to_account_info(),
            context.accounts.new_leaf_owner2.to_account_info(),
            context.accounts.merkle_tree2.to_account_info(),
            context.accounts.log_wrapper.to_account_info(),
            context.accounts.compression_program.to_account_info(),
            context.accounts.system_program.to_account_info(),
        ];
        for acc in proof2_accounts.iter() {
            account_infos2.push(acc.to_account_info());
        }

        invoke_signed(&instruction2, &account_infos2, &[signer_seeds])?;

        msg!("successfully sent cNFTs");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    #[account(
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is modified in the downstream program
    pub tree_authority: UncheckedAccount<'info>,
    #[account(
        seeds = [b"cNFT-vault"],
        bump,
    )]
    /// CHECK: This account doesnt even exist (it is just the pda to sign)
    pub leaf_owner: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub new_leaf_owner: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub log_wrapper: UncheckedAccount<'info>,
    pub compression_program: Program<'info, SPLCompression>,
    /// CHECK: This account is neither written to nor read from.
    pub bubblegum_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawTwo<'info> {
    #[account(mut)]
    #[account(
        seeds = [merkle_tree1.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is modified in the downstream program
    pub tree_authority1: UncheckedAccount<'info>,
    #[account(
        seeds = [b"cNFT-vault"],
        bump,
    )]
    /// CHECK: This account doesnt even exist (it is just the pda to sign)
    pub leaf_owner: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub new_leaf_owner1: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree1: UncheckedAccount<'info>,

    #[account(mut)]
    #[account(
        seeds = [merkle_tree2.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is modified in the downstream program
    pub tree_authority2: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub new_leaf_owner2: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree2: UncheckedAccount<'info>,

    /// CHECK: This account is neither written to nor read from.
    pub log_wrapper: UncheckedAccount<'info>,
    pub compression_program: Program<'info, SPLCompression>,
    /// CHECK: This account is neither written to nor read from.
    pub bubblegum_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
