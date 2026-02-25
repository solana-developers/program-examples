use anchor_lang::prelude::*;

declare_id!("Fd4iwpPWaCU8BNwGQGtvvrcvG4Tfizq3RgLm8YLBJX6D");

#[derive(Clone)]
pub struct SPLCompression;

impl anchor_lang::Id for SPLCompression {
    fn id() -> Pubkey {
        spl_account_compression::id()
    }
}

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

        let tree_config = ctx.accounts.tree_authority.to_account_info();
        let leaf_owner = ctx.accounts.leaf_owner.to_account_info();
        let new_leaf_owner = ctx.accounts.new_leaf_owner.to_account_info();
        let merkle_tree = ctx.accounts.merkle_tree.to_account_info();
        let log_wrapper = ctx.accounts.log_wrapper.to_account_info();
        let compression_program = ctx.accounts.compression_program.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        let transfer_cpi = mpl_bubblegum::instructions::TransferCpi::new(
            &ctx.accounts.bubblegum_program,
            mpl_bubblegum::instructions::TransferCpiAccounts {
                tree_config: &tree_config,
                leaf_owner: (&leaf_owner, true),
                leaf_delegate: (&leaf_owner, false),
                new_leaf_owner: &new_leaf_owner,
                merkle_tree: &merkle_tree,
                log_wrapper: &log_wrapper,
                compression_program: &compression_program,
                system_program: &system_program,
            },
            mpl_bubblegum::instructions::TransferInstructionArgs {
                root,
                data_hash,
                creator_hash,
                nonce,
                index,
            },
        );

        transfer_cpi.invoke_signed_with_remaining_accounts(
            &[&[b"cNFT-vault", &[ctx.bumps.leaf_owner]]],
            ctx.remaining_accounts
                .iter()
                .map(|account| (account, false, false))
                .collect::<Vec<_>>()
                .as_slice(),
        )?;

        Ok(())
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
        _proof_2_length: u8,
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

        let tree_config1 = ctx.accounts.tree_authority1.to_account_info();
        let tree_config2 = ctx.accounts.tree_authority2.to_account_info();
        let leaf_owner = ctx.accounts.leaf_owner.to_account_info();
        let new_leaf_owner1 = ctx.accounts.new_leaf_owner1.to_account_info();
        let new_leaf_owner2 = ctx.accounts.new_leaf_owner2.to_account_info();
        let merkle_tree1_info = ctx.accounts.merkle_tree1.to_account_info();
        let merkle_tree2_info = ctx.accounts.merkle_tree2.to_account_info();
        let log_wrapper = ctx.accounts.log_wrapper.to_account_info();
        let compression_program = ctx.accounts.compression_program.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        let signer_seeds: &[&[u8]] = &[b"cNFT-vault", &[ctx.bumps.leaf_owner]];

        // Split remaining accounts into proof1 and proof2
        let (proof1_accounts, proof2_accounts) =
            ctx.remaining_accounts.split_at(proof_1_length as usize);

        msg!("withdrawing cNFT#1");
        let transfer_cpi1 = mpl_bubblegum::instructions::TransferCpi::new(
            &ctx.accounts.bubblegum_program,
            mpl_bubblegum::instructions::TransferCpiAccounts {
                tree_config: &tree_config1,
                leaf_owner: (&leaf_owner, true),
                leaf_delegate: (&leaf_owner, false),
                new_leaf_owner: &new_leaf_owner1,
                merkle_tree: &merkle_tree1_info,
                log_wrapper: &log_wrapper,
                compression_program: &compression_program,
                system_program: &system_program,
            },
            mpl_bubblegum::instructions::TransferInstructionArgs {
                root: root1,
                data_hash: data_hash1,
                creator_hash: creator_hash1,
                nonce: nonce1,
                index: index1,
            },
        );

        transfer_cpi1.invoke_signed_with_remaining_accounts(
            &[signer_seeds],
            proof1_accounts
                .iter()
                .map(|account| (account, false, false))
                .collect::<Vec<_>>()
                .as_slice(),
        )?;

        msg!("withdrawing cNFT#2");
        let transfer_cpi2 = mpl_bubblegum::instructions::TransferCpi::new(
            &ctx.accounts.bubblegum_program,
            mpl_bubblegum::instructions::TransferCpiAccounts {
                tree_config: &tree_config2,
                leaf_owner: (&leaf_owner, true),
                leaf_delegate: (&leaf_owner, false),
                new_leaf_owner: &new_leaf_owner2,
                merkle_tree: &merkle_tree2_info,
                log_wrapper: &log_wrapper,
                compression_program: &compression_program,
                system_program: &system_program,
            },
            mpl_bubblegum::instructions::TransferInstructionArgs {
                root: root2,
                data_hash: data_hash2,
                creator_hash: creator_hash2,
                nonce: nonce2,
                index: index2,
            },
        );

        transfer_cpi2.invoke_signed_with_remaining_accounts(
            &[signer_seeds],
            proof2_accounts
                .iter()
                .map(|account| (account, false, false))
                .collect::<Vec<_>>()
                .as_slice(),
        )?;

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
