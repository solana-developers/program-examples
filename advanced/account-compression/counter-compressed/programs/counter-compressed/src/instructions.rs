use anchor_lang::prelude::*;

use crate::{
    Counter, 
    TreeInfo, 
    TREE_INFO_SIZE, 
    COUNTER_PREFIX, 
    COUNTER_SIZE
};

// Compressed instructions
#[derive(Accounts)]
pub struct InitializeGlobalCounterTree<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds=[merkle_tree.key.as_ref()], 
        bump,
        space=TREE_INFO_SIZE,
        payer=payer
    )]
    pub tree_authority: Account<'info, TreeInfo>,
    /// CHECK: this will be initialized via the AC program
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK: this will be checked via the CPI call
    pub spl_noop_program: AccountInfo<'info>,
    /// CHECK: this will be checked via the CPI call
    pub account_compression_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCounterCompressed<'info> {
    #[account(seeds=[merkle_tree.key.as_ref()], bump)]
    pub tree_authority: Account<'info, TreeInfo>,
    /// CHECK: checked by AC CPI
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK: checked by AC CPI
    pub spl_noop_program: AccountInfo<'info>,
    /// CHECK: checked by AC CPI
    pub account_compression_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddCounterCompressed<'info> {
    #[account(mut, seeds=[merkle_tree.key.as_ref()], bump)]
    pub tree_authority: Account<'info, TreeInfo>,
    /// CHECK: checked by AC CPI
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK: checked by AC CPI
    pub spl_noop_program: AccountInfo<'info>,
    /// CHECK: checked by AC CPI
    pub account_compression_program: AccountInfo<'info>,
}


#[derive(Accounts)]
#[instruction(counter_state: Counter)]
pub struct DecompressCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds=[merkle_tree.key.as_ref(), COUNTER_PREFIX.as_bytes(), &counter_state.id.to_le_bytes()],
        bump,
        space=COUNTER_SIZE,
        payer=payer
    )]
    pub counter: Account<'info, Counter>,
    #[account(seeds=[merkle_tree.key.as_ref()], bump)]
    pub tree_authority: Account<'info, TreeInfo>,
    /// CHECK: checked by AC CPI
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK: checked by AC CPI
    pub spl_noop_program: AccountInfo<'info>,
    /// CHECK: checked by AC CPI
    pub account_compression_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

// Decompressed instructions
#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        mut, 
        seeds=[
            &counter.tree.as_ref(), 
            COUNTER_PREFIX.as_bytes(),
            &counter.id.to_le_bytes()
        ],
        bump
    )]
    pub counter: Account<'info, Counter>
}

#[derive(Accounts)]
pub struct CompressCounter<'info> {
    #[account(mut, close=claimer)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    /// CHECK: this is the account to send reclaimed lamports to
    pub claimer: AccountInfo<'info>,
    #[account(seeds=[merkle_tree.key.as_ref()], bump)]
    pub tree_authority: Account<'info, TreeInfo>,
    #[account(mut)]
    /// CHECK: this will be checked by compression CPI
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK: this will be checked by compression CPI
    pub spl_noop_program: AccountInfo<'info>,
    /// CHECK: this will be checked via the CPI call
    pub account_compression_program: AccountInfo<'info>
}

