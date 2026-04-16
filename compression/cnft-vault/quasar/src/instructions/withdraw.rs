use crate::*;
use quasar_lang::cpi::{InstructionAccount, InstructionView, Seed, Signer};

/// Maximum proof nodes for the merkle tree.
const MAX_PROOF_NODES: usize = 24;

/// 8 fixed accounts + proof nodes.
const MAX_CPI_ACCOUNTS: usize = 8 + MAX_PROOF_NODES;

/// Transfer args byte length: root(32) + data_hash(32) + creator_hash(32) + nonce(8) + index(4).
const TRANSFER_ARGS_LEN: usize = 108;

/// Accounts for withdrawing a single compressed NFT from the vault.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// Tree authority PDA (seeds checked by Bubblegum).
    #[account(mut)]
    pub tree_authority: &'info UncheckedAccount,
    /// Vault PDA that owns the cNFT — signs the transfer via invoke_signed.
    #[account(seeds = [b"cNFT-vault"], bump)]
    pub leaf_owner: &'info UncheckedAccount,
    /// New owner to receive the cNFT.
    pub new_leaf_owner: &'info UncheckedAccount,
    /// Merkle tree account.
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

/// Build mpl-bubblegum Transfer instruction data from raw args.
fn build_transfer_data(args: &[u8]) -> [u8; 8 + TRANSFER_ARGS_LEN] {
    let mut ix_data = [0u8; 8 + TRANSFER_ARGS_LEN];
    ix_data[0..8].copy_from_slice(&TRANSFER_DISCRIMINATOR);
    ix_data[8..].copy_from_slice(args);
    ix_data
}

pub fn handle_withdraw_cnft<'info>(
    accounts: &Withdraw<'info>, ctx: &CtxWithRemaining<'info, Withdraw<'info>>,
) -> Result<(), ProgramError> {
    let data = ctx.data;
    if data.len() < TRANSFER_ARGS_LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    let ix_data = build_transfer_data(&data[0..TRANSFER_ARGS_LEN]);

    // Collect proof nodes
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

    let total_accounts = 8 + proof_count;

    // Build instruction account metas matching mpl-bubblegum Transfer layout:
    // tree_config, leaf_owner (signer/PDA), leaf_delegate, new_leaf_owner,
    // merkle_tree, log_wrapper, compression_program, system_program, then proofs.
    let sys_addr = accounts.system_program.address();
    let mut ix_accounts: [InstructionAccount; MAX_CPI_ACCOUNTS] =
        core::array::from_fn(|_| InstructionAccount::readonly(sys_addr));

    ix_accounts[0] = InstructionAccount::readonly(accounts.tree_authority.address());
    ix_accounts[1] = InstructionAccount::readonly_signer(accounts.leaf_owner.address());
    // leaf_delegate = leaf_owner, not an additional signer
    ix_accounts[2] = InstructionAccount::readonly(accounts.leaf_owner.address());
    ix_accounts[3] = InstructionAccount::readonly(accounts.new_leaf_owner.address());
    ix_accounts[4] = InstructionAccount::writable(accounts.merkle_tree.address());
    ix_accounts[5] = InstructionAccount::readonly(accounts.log_wrapper.address());
    ix_accounts[6] = InstructionAccount::readonly(accounts.compression_program.address());
    ix_accounts[7] = InstructionAccount::readonly(accounts.system_program.address());

    for i in 0..proof_count {
        ix_accounts[8 + i] = InstructionAccount::readonly(proof_views[i].address());
    }

    // Build account views
    let sys_view = accounts.system_program.to_account_view().clone();
    let mut views: [AccountView; MAX_CPI_ACCOUNTS] =
        core::array::from_fn(|_| sys_view.clone());

    views[0] = accounts.tree_authority.to_account_view().clone();
    views[1] = accounts.leaf_owner.to_account_view().clone();
    views[2] = accounts.leaf_owner.to_account_view().clone();
    views[3] = accounts.new_leaf_owner.to_account_view().clone();
    views[4] = accounts.merkle_tree.to_account_view().clone();
    views[5] = accounts.log_wrapper.to_account_view().clone();
    views[6] = accounts.compression_program.to_account_view().clone();
    views[7] = accounts.system_program.to_account_view().clone();

    for i in 0..proof_count {
        views[8 + i] = proof_views[i].clone();
    }

    let instruction = InstructionView {
        program_id: &MPL_BUBBLEGUM_ID,
        data: &ix_data,
        accounts: &ix_accounts[..total_accounts],
    };

    // PDA signer seeds: ["cNFT-vault", bump]
    let bump_bytes = [ctx.bumps.leaf_owner];
    let seeds: [Seed; 2] = [
        Seed::from(b"cNFT-vault" as &[u8]),
        Seed::from(&bump_bytes as &[u8]),
    ];
    let signer = Signer::from(&seeds as &[Seed]);

    solana_instruction_view::cpi::invoke_signed_with_bounds::<MAX_CPI_ACCOUNTS, AccountView>(
        &instruction,
        &views[..total_accounts],
        &[signer],
    )
}
