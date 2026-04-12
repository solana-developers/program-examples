use crate::*;
use quasar_lang::cpi::{InstructionAccount, InstructionView, Seed, Signer};

/// Maximum proof nodes per tree.
const MAX_PROOF_NODES: usize = 24;

/// 8 fixed accounts + proof nodes per CPI call.
const MAX_CPI_ACCOUNTS: usize = 8 + MAX_PROOF_NODES;

/// Transfer args byte length: root(32) + data_hash(32) + creator_hash(32) + nonce(8) + index(4).
const TRANSFER_ARGS_LEN: usize = 108;

/// Accounts for withdrawing two compressed NFTs from the vault in one transaction.
/// Each cNFT can be from a different merkle tree.
#[derive(Accounts)]
pub struct WithdrawTwo<'info> {
    /// Tree authority PDA for tree 1.
    #[account(mut)]
    pub tree_authority1: &'info UncheckedAccount,
    /// Vault PDA that owns the cNFTs — signs both transfers.
    #[account(seeds = [b"cNFT-vault"], bump)]
    pub leaf_owner: &'info UncheckedAccount,
    /// Recipient for cNFT 1.
    pub new_leaf_owner1: &'info UncheckedAccount,
    /// Merkle tree for cNFT 1.
    #[account(mut)]
    pub merkle_tree1: &'info UncheckedAccount,
    /// Tree authority PDA for tree 2.
    #[account(mut)]
    pub tree_authority2: &'info UncheckedAccount,
    /// Recipient for cNFT 2.
    pub new_leaf_owner2: &'info UncheckedAccount,
    /// Merkle tree for cNFT 2.
    #[account(mut)]
    pub merkle_tree2: &'info UncheckedAccount,
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

impl<'info> WithdrawTwo<'info> {
    #[allow(clippy::too_many_lines)]
    pub fn withdraw_two_cnfts(
        &self,
        ctx: &CtxWithRemaining<'info, WithdrawTwo<'info>>,
    ) -> Result<(), ProgramError> {
        // Parse instruction args:
        // args1(108) + proof_1_length(1) + args2(108) + _proof_2_length(1) = 218 bytes
        let data = ctx.data;
        if data.len() < 218 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let args1 = &data[0..TRANSFER_ARGS_LEN];
        let proof_1_length = data[TRANSFER_ARGS_LEN] as usize;
        let args2 = &data[TRANSFER_ARGS_LEN + 1..TRANSFER_ARGS_LEN * 2 + 1];
        // _proof_2_length at data[217] — not needed, remaining after proof1 is proof2

        // PDA signer seeds
        let bump_bytes = [ctx.bumps.leaf_owner];
        let seeds: [Seed; 2] = [
            Seed::from(b"cNFT-vault" as &[u8]),
            Seed::from(&bump_bytes as &[u8]),
        ];
        let signer = Signer::from(&seeds as &[Seed]);

        // Collect all remaining accounts (proof1 ++ proof2)
        let remaining = ctx.remaining_accounts();
        let placeholder = self.system_program.to_account_view().clone();
        let mut all_proofs: [AccountView; MAX_PROOF_NODES * 2] =
            core::array::from_fn(|_| placeholder.clone());
        let mut total_proofs = 0usize;
        for result in remaining.iter() {
            if total_proofs >= MAX_PROOF_NODES * 2 {
                break;
            }
            all_proofs[total_proofs] = result?;
            total_proofs += 1;
        }

        // Split into proof1 and proof2
        let proof1_count = proof_1_length.min(total_proofs);
        let proof2_count = total_proofs.saturating_sub(proof1_count);

        // --- Withdraw cNFT #1 ---
        log("withdrawing cNFT#1");
        {
            let mut ix_data = [0u8; 8 + TRANSFER_ARGS_LEN];
            ix_data[0..8].copy_from_slice(&TRANSFER_DISCRIMINATOR);
            ix_data[8..].copy_from_slice(args1);

            let total_accounts = 8 + proof1_count;
            let sys_addr = self.system_program.address();
            let mut ix_accounts: [InstructionAccount; MAX_CPI_ACCOUNTS] =
                core::array::from_fn(|_| InstructionAccount::readonly(sys_addr));

            ix_accounts[0] = InstructionAccount::readonly(self.tree_authority1.address());
            ix_accounts[1] = InstructionAccount::readonly_signer(self.leaf_owner.address());
            ix_accounts[2] = InstructionAccount::readonly(self.leaf_owner.address());
            ix_accounts[3] = InstructionAccount::readonly(self.new_leaf_owner1.address());
            ix_accounts[4] = InstructionAccount::writable(self.merkle_tree1.address());
            ix_accounts[5] = InstructionAccount::readonly(self.log_wrapper.address());
            ix_accounts[6] = InstructionAccount::readonly(self.compression_program.address());
            ix_accounts[7] = InstructionAccount::readonly(self.system_program.address());

            for i in 0..proof1_count {
                ix_accounts[8 + i] = InstructionAccount::readonly(all_proofs[i].address());
            }

            let sys_view = self.system_program.to_account_view().clone();
            let mut views: [AccountView; MAX_CPI_ACCOUNTS] =
                core::array::from_fn(|_| sys_view.clone());

            views[0] = self.tree_authority1.to_account_view().clone();
            views[1] = self.leaf_owner.to_account_view().clone();
            views[2] = self.leaf_owner.to_account_view().clone();
            views[3] = self.new_leaf_owner1.to_account_view().clone();
            views[4] = self.merkle_tree1.to_account_view().clone();
            views[5] = self.log_wrapper.to_account_view().clone();
            views[6] = self.compression_program.to_account_view().clone();
            views[7] = self.system_program.to_account_view().clone();

            for i in 0..proof1_count {
                views[8 + i] = all_proofs[i].clone();
            }

            let instruction = InstructionView {
                program_id: &MPL_BUBBLEGUM_ID,
                data: &ix_data,
                accounts: &ix_accounts[..total_accounts],
            };

            solana_instruction_view::cpi::invoke_signed_with_bounds::<
                MAX_CPI_ACCOUNTS,
                AccountView,
            >(&instruction, &views[..total_accounts], &[signer.clone()])?;
        }

        // --- Withdraw cNFT #2 ---
        log("withdrawing cNFT#2");
        {
            let mut ix_data = [0u8; 8 + TRANSFER_ARGS_LEN];
            ix_data[0..8].copy_from_slice(&TRANSFER_DISCRIMINATOR);
            ix_data[8..].copy_from_slice(args2);

            let total_accounts = 8 + proof2_count;
            let sys_addr = self.system_program.address();
            let mut ix_accounts: [InstructionAccount; MAX_CPI_ACCOUNTS] =
                core::array::from_fn(|_| InstructionAccount::readonly(sys_addr));

            ix_accounts[0] = InstructionAccount::readonly(self.tree_authority2.address());
            ix_accounts[1] = InstructionAccount::readonly_signer(self.leaf_owner.address());
            ix_accounts[2] = InstructionAccount::readonly(self.leaf_owner.address());
            ix_accounts[3] = InstructionAccount::readonly(self.new_leaf_owner2.address());
            ix_accounts[4] = InstructionAccount::writable(self.merkle_tree2.address());
            ix_accounts[5] = InstructionAccount::readonly(self.log_wrapper.address());
            ix_accounts[6] = InstructionAccount::readonly(self.compression_program.address());
            ix_accounts[7] = InstructionAccount::readonly(self.system_program.address());

            let proof2_start = proof1_count;
            for i in 0..proof2_count {
                ix_accounts[8 + i] =
                    InstructionAccount::readonly(all_proofs[proof2_start + i].address());
            }

            let sys_view = self.system_program.to_account_view().clone();
            let mut views: [AccountView; MAX_CPI_ACCOUNTS] =
                core::array::from_fn(|_| sys_view.clone());

            views[0] = self.tree_authority2.to_account_view().clone();
            views[1] = self.leaf_owner.to_account_view().clone();
            views[2] = self.leaf_owner.to_account_view().clone();
            views[3] = self.new_leaf_owner2.to_account_view().clone();
            views[4] = self.merkle_tree2.to_account_view().clone();
            views[5] = self.log_wrapper.to_account_view().clone();
            views[6] = self.compression_program.to_account_view().clone();
            views[7] = self.system_program.to_account_view().clone();

            for i in 0..proof2_count {
                views[8 + i] = all_proofs[proof2_start + i].clone();
            }

            let instruction = InstructionView {
                program_id: &MPL_BUBBLEGUM_ID,
                data: &ix_data,
                accounts: &ix_accounts[..total_accounts],
            };

            solana_instruction_view::cpi::invoke_signed_with_bounds::<
                MAX_CPI_ACCOUNTS,
                AccountView,
            >(&instruction, &views[..total_accounts], &[signer])?;
        }

        log("successfully sent cNFTs");
        Ok(())
    }
}
