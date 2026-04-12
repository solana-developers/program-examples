use crate::*;
use quasar_lang::cpi::{InstructionAccount, InstructionView};

/// Maximum number of proof nodes for the merkle tree.
/// Concurrent merkle trees support up to depth 30, but typical depth is 14-20.
const MAX_PROOF_NODES: usize = 24;

/// Total max accounts for the CPI: 7 fixed + proof nodes.
const MAX_CPI_ACCOUNTS: usize = 7 + MAX_PROOF_NODES;

/// Accounts for burning a compressed NFT via mpl-bubblegum CPI.
#[derive(Accounts)]
pub struct BurnCnft<'info> {
    #[account(mut)]
    pub leaf_owner: &'info Signer,
    /// Tree authority PDA (seeds checked by Bubblegum).
    #[account(mut)]
    pub tree_authority: &'info UncheckedAccount,
    /// Merkle tree account modified by the compression program.
    #[account(mut)]
    pub merkle_tree: &'info UncheckedAccount,
    /// SPL Noop log wrapper.
    pub log_wrapper: &'info UncheckedAccount,
    /// SPL Account Compression program.
    #[account(address = SPL_ACCOUNT_COMPRESSION_ID)]
    pub compression_program: &'info UncheckedAccount,
    /// mpl-bubblegum program.
    #[account(address = MPL_BUBBLEGUM_ID)]
    pub bubblegum_program: &'info UncheckedAccount,
    pub system_program: &'info Program<System>,
}

pub fn handle_burn_cnft<'info>(
    accounts: &BurnCnft<'info>, ctx: &CtxWithRemaining<'info, BurnCnft<'info>>,
) -> Result<(), ProgramError> {
    // Parse instruction args from raw data:
    // root(32) + data_hash(32) + creator_hash(32) + nonce(8) + index(4) = 108 bytes
    let data = ctx.data;
    if data.len() < 108 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Build instruction data: discriminator + args
    // 8 + 32 + 32 + 32 + 8 + 4 = 116 bytes
    let mut ix_data = [0u8; 116];
    ix_data[0..8].copy_from_slice(&BURN_DISCRIMINATOR);
    ix_data[8..116].copy_from_slice(&data[0..108]);

    // Collect remaining accounts (proof nodes) into a stack buffer
    let remaining = ctx.remaining_accounts();
    let placeholder = accounts.system_program.to_account_view().clone();
    let mut proof_views: [AccountView; MAX_PROOF_NODES] =
        core::array::from_fn(|_| placeholder.clone());
    let mut proof_count = 0usize;
    for result in remaining.iter() {
        if proof_count >= MAX_PROOF_NODES {
            break;
        }
        proof_views[proof_count] = result?;
        proof_count += 1;
    }

    let total_accounts = 7 + proof_count;

    // Build instruction account metas.
    // Layout matches mpl-bubblegum Burn: tree_authority, leaf_owner (signer),
    // leaf_delegate (= leaf_owner, not signer), merkle_tree, log_wrapper,
    // compression_program, system_program, then proof nodes.
    let sys_addr = accounts.system_program.address();
    let mut ix_accounts: [InstructionAccount; MAX_CPI_ACCOUNTS] = core::array::from_fn(|_| {
        InstructionAccount::readonly(sys_addr)
    });

    ix_accounts[0] = InstructionAccount::readonly(accounts.tree_authority.address());
    ix_accounts[1] = InstructionAccount::readonly_signer(accounts.leaf_owner.address());
    // leaf_delegate = leaf_owner, not a signer in this call
    ix_accounts[2] = InstructionAccount::readonly(accounts.leaf_owner.address());
    ix_accounts[3] = InstructionAccount::writable(accounts.merkle_tree.address());
    ix_accounts[4] = InstructionAccount::readonly(accounts.log_wrapper.address());
    ix_accounts[5] = InstructionAccount::readonly(accounts.compression_program.address());
    ix_accounts[6] = InstructionAccount::readonly(accounts.system_program.address());

    for i in 0..proof_count {
        ix_accounts[7 + i] = InstructionAccount::readonly(proof_views[i].address());
    }

    // Build account views array for the CPI
    let sys_view = accounts.system_program.to_account_view().clone();
    let mut views: [AccountView; MAX_CPI_ACCOUNTS] = core::array::from_fn(|_| sys_view.clone());

    views[0] = accounts.tree_authority.to_account_view().clone();
    views[1] = accounts.leaf_owner.to_account_view().clone();
    views[2] = accounts.leaf_owner.to_account_view().clone(); // leaf_delegate = leaf_owner
    views[3] = accounts.merkle_tree.to_account_view().clone();
    views[4] = accounts.log_wrapper.to_account_view().clone();
    views[5] = accounts.compression_program.to_account_view().clone();
    views[6] = accounts.system_program.to_account_view().clone();

    for i in 0..proof_count {
        views[7 + i] = proof_views[i].clone();
    }

    let instruction = InstructionView {
        program_id: &MPL_BUBBLEGUM_ID,
        data: &ix_data,
        accounts: &ix_accounts[..total_accounts],
    };

    solana_instruction_view::cpi::invoke_with_bounds::<MAX_CPI_ACCOUNTS, AccountView>(
        &instruction,
        &views[..total_accounts],
    )
}
