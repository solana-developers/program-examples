use anchor_lang::prelude::*;

declare_id!("5FjsgRt8fkWv22pyksKcVBkieh1J15qEv7WfSP4CNLyx");

mod state;
mod instructions;
mod constants;
mod errors;

use state::*;
use instructions::*;
use errors::*;

#[program]
pub mod cnft_candy_machine {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u32, price_sol: Option<u64>, price_spl: Option<u64>, spl_address: Option<Pubkey>, max_depth: u32, max_buffer_size: u32) -> Result<()> {
        ctx.accounts.init_config(total_supply, price_sol, price_spl, spl_address, &ctx.bumps)?;
        ctx.accounts.init_tree(max_depth, max_buffer_size)
    }

    pub fn set_tree_status(ctx: Context<SetTreeStatus>, status: TreeStatus) -> Result<()> {
        ctx.accounts.set_tree_status(status)
    }

    pub fn create_collection(ctx: Context<CreateCollection>, name: String, symbol: String, uri: String) -> Result<()> {
        ctx.accounts.create_collection(name, symbol, uri)
    }

    pub fn add_allow_list(ctx: Context<AllowList>, user: Pubkey, amount: u8) -> Result<()> {
        ctx.accounts.add(user, amount)
    }

    pub fn mint<'info>(ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>, name: String, symbol: String, uri: String, pay_sol: bool) -> Result<()> {
        ctx.accounts.mint_cnft(name, symbol, uri, pay_sol, ctx.remaining_accounts)
    }
}
