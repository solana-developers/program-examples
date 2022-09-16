use anchor_lang::prelude::*;
use anchor_lang::solana_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod instructions;
use instructions::*;

mod counter;
use counter::*;

mod tree_info;
use tree_info::*;

use spl_account_compression::Node;

pub fn replace_leaf<'info>(
    seed: &Pubkey,
    bump: u8,
    account_compression_program: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    merkle_tree: &AccountInfo<'info>,
    spl_noop_program: &AccountInfo<'info>,
    remaining_accounts: &[AccountInfo<'info>],
    root_node: Node,
    previous_leaf: Node,
    new_leaf: Node,
    index: u32,
) -> Result<()> {
    let seeds = &[seed.as_ref(), &[bump]];
    let authority_pda_signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        account_compression_program.clone(),
        spl_account_compression::cpi::accounts::Modify {
            authority: authority.clone(),
            merkle_tree: merkle_tree.clone(),
            log_wrapper: spl_noop_program.clone(),
        },
        authority_pda_signer,
    )
    .with_remaining_accounts(remaining_accounts.to_vec());
    spl_account_compression::cpi::replace_leaf(cpi_ctx, root_node, previous_leaf, new_leaf, index)
}

pub fn append_leaf<'info>(
    seed: &Pubkey,
    bump: u8,
    account_compression_program: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    merkle_tree: &AccountInfo<'info>,
    spl_noop_program: &AccountInfo<'info>,
    leaf_node: Node,
) -> Result<()> {
    let seeds = &[seed.as_ref(), &[bump]];
    let authority_pda_signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        account_compression_program.clone(),
        spl_account_compression::cpi::accounts::Modify {
            authority: authority.clone(),
            merkle_tree: merkle_tree.clone(),
            log_wrapper: spl_noop_program.clone(),
        },
        authority_pda_signer,
    );
    spl_account_compression::cpi::append(cpi_ctx, leaf_node)
}

pub fn initialize_tree<'info>(
    max_depth: u32,
    max_buffer_size: u32,
    seed: &Pubkey,
    bump: u8,
    account_compression_program: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    merkle_tree: &AccountInfo<'info>,
    spl_noop_program: &AccountInfo<'info>,
) -> Result<()> {
    let seeds = &[seed.as_ref(), &[bump]];
    let authority_pda_signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        account_compression_program.to_account_info(),
        spl_account_compression::cpi::accounts::Initialize {
            authority: authority.clone(),
            merkle_tree: merkle_tree.clone(),
            log_wrapper: spl_noop_program.clone(),
        },
        authority_pda_signer,
    );
    spl_account_compression::cpi::init_empty_merkle_tree(cpi_ctx, max_depth, max_buffer_size)
}

#[program]
pub mod counter_compressed {
    use super::*;

    pub fn initialize_global_counter_tree(
        ctx: Context<InitializeGlobalCounterTree>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        ctx.accounts.tree_authority.num_counters = 0;
        initialize_tree(
            max_depth,
            max_buffer_size,
            &ctx.accounts.merkle_tree.key,
            *ctx.bumps.get("tree_authority").unwrap(),
            &ctx.accounts.account_compression_program.to_account_info(),
            &ctx.accounts.tree_authority.to_account_info(),
            &ctx.accounts.merkle_tree.to_account_info(),
            &ctx.accounts.spl_noop_program.to_account_info(),
        )
    }

    /// Executes append
    pub fn initialize_compressed_counter<'info>(
        ctx: Context<'_, '_, '_, 'info, AddCounterCompressed<'info>>,
    ) -> Result<()> {
        let counter = Counter::new(
            ctx.accounts.merkle_tree.key,
            ctx.accounts.tree_authority.num_counters,
        );
        let new_leaf = hash_counter(counter)?;

        ctx.accounts.tree_authority.num_counters += 1;
        append_leaf(
            &ctx.accounts.merkle_tree.key,
            *ctx.bumps.get("tree_authority").unwrap(),
            &ctx.accounts.account_compression_program.to_account_info(),
            &ctx.accounts.tree_authority.to_account_info(),
            &ctx.accounts.merkle_tree.to_account_info(),
            &ctx.accounts.spl_noop_program.to_account_info(),
            new_leaf,
        )
    }

    /// Executes replace
    pub fn increment_compressed_counter<'info>(
        ctx: Context<'_, '_, '_, 'info, ModifyCounterCompressed<'info>>,
        counter_state: Counter,
        root_node: [u8; 32],
    ) -> Result<()> {
        let merkle_tree = &ctx.accounts.merkle_tree;
        let tree_authority = &ctx.accounts.tree_authority;
        let leaf_index: u32 = counter_state.id as u32;
        let previous_leaf = hash_counter(counter_state)?;

        let mut new_counter_state = counter_state.clone();
        new_counter_state.count += 1;
        let new_leaf = hash_counter(new_counter_state)?;

        let bump = ctx.bumps.get("tree_authority").unwrap();
        replace_leaf(
            &merkle_tree.key,
            *bump,
            &ctx.accounts.account_compression_program.to_account_info(),
            &tree_authority.to_account_info(),
            &ctx.accounts.merkle_tree.to_account_info(),
            &ctx.accounts.spl_noop_program.to_account_info(),
            ctx.remaining_accounts,
            root_node,
            previous_leaf,
            new_leaf,
            leaf_index,
        )
    }

    /// Executes replace to EMPTY
    pub fn decompress_counter<'info>(
        ctx: Context<'_, '_, '_, 'info, DecompressCounter<'info>>,
        counter_state: Counter,
        root_node: [u8; 32],
    ) -> Result<()> {
        let merkle_tree = &ctx.accounts.merkle_tree;
        let tree_authority = &ctx.accounts.tree_authority;
        let leaf_index: u32 = counter_state.id as u32;
        let previous_leaf = hash_counter(counter_state)?;

        ctx.accounts.counter.set_inner(counter_state);
        let bump = ctx.bumps.get("tree_authority").unwrap();
        replace_leaf(
            &merkle_tree.key,
            *bump,
            &ctx.accounts.account_compression_program.to_account_info(),
            &tree_authority.to_account_info(),
            &ctx.accounts.merkle_tree.to_account_info(),
            &ctx.accounts.spl_noop_program.to_account_info(),
            ctx.remaining_accounts,
            root_node,
            previous_leaf,
            [0; 32],
            leaf_index,
        )
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        ctx.accounts.counter.count += 1;
        Ok(())
    }

    /// Executes a replace to full
    pub fn compress_counter<'info>(
        ctx: Context<'_, '_, '_, 'info, CompressCounter<'info>>,
        root_node: [u8; 32],
    ) -> Result<()> {
        // TODO: close Counter account
        replace_leaf(
            &ctx.accounts.merkle_tree.key,
            *ctx.bumps.get("tree_authority").unwrap(),
            &ctx.accounts.account_compression_program.to_account_info(),
            &ctx.accounts.tree_authority.to_account_info(),
            &ctx.accounts.merkle_tree.to_account_info(),
            &ctx.accounts.spl_noop_program.to_account_info(),
            ctx.remaining_accounts,
            root_node,
            [0; 32],
            hash_counter(*ctx.accounts.counter)?,
            ctx.accounts.counter.id,
        )
    }
}
