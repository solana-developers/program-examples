use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        Mint, 
        Token
    }
};
use crate::{
    state::{
        Config, 
        TreeStatus
    }, 
    CustomError
};
use mpl_bubblegum::{
    instructions::CreateTreeConfigCpiBuilder, 
    ID as BUBBLEGUM_ID
};
use spl_account_compression::ID as SPL_ACCOUNT_COMPRESSION_ID;
use spl_noop::ID as SPL_NOOP_ID;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [b"config", authority.key().as_ref()],
        bump,
        space = Config::INIT_SPACE
    )]
    pub config: Box<Account<'info, Config>>,
    pub allow_mint: Option<Account<'info, Mint>>,
    #[account(
        init,
        payer = authority,
        seeds = [b"collection", config.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = config,
        mint::freeze_authority = config,
    )]
    pub collection: Account<'info, Mint>,
    /// CHECK: Tree Config checks will be performed by the Bubblegum Program
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    /// CHECK: Unitialized Merkle Tree Account. Initialization will be performed by the Bubblegum Program 
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: SPL NOOP Program checked by the corresponding address
    #[account(address = SPL_NOOP_ID)]
    pub log_wrapper: UncheckedAccount<'info>,
    /// CHECK: Bubblegum Program checked by the corresponding address
    #[account(address = BUBBLEGUM_ID)]
    pub bubblegum_program: UncheckedAccount<'info>,
    /// CHECK: SPL Account Compression Program checked by the corresponding address
    #[account(address = SPL_ACCOUNT_COMPRESSION_ID)]
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn init_config(&mut self, total_supply: u32, price_sol: Option<u64>, price_spl: Option<u64>, spl_address: Option<Pubkey>, bumps: &InitializeBumps) -> Result<()> {
        // Check if there is a mint in the allow mint account and return the key or None
        let allow_mint = match self.allow_mint.clone() {
            Some(value) => Some(value.key()),
            None => None,
        };

        // Check if there is a price and address for the SPL token and return the key or None
        let (price_spl, spl_address) = match price_spl.is_some() && spl_address.is_some() {
            true => (price_spl, spl_address),
            false => {
                // If one is true and the other is false, return an error
                require!(price_spl.is_none() && spl_address.is_none(), CustomError::InvalidSPLSettings);
                // If both are false, return None
                (None, None)
            },
        };

        self.config.set_inner(
            // Initialize the config account
            Config {
                authority: self.authority.key(),
                allow_list: vec![],
                allow_mint,
                collection: self.collection.key(),
                total_supply,
                current_supply: 0,
                price_sol,
                price_spl,
                spl_address,
                status: TreeStatus::Private,
                bump: bumps.config, 
            },
        );
        Ok(())
    }

    pub fn init_tree(&mut self, max_depth: u32, max_buffer_size: u32) -> Result<()> {
        // Create the seeds for the CPI call
        let seeds = &[
            &b"config"[..], 
            &self.authority.key.as_ref(),
            &[self.config.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        // Accounts for CPI calls
        let bubblegum_program = &self.bubblegum_program.to_account_info();
        let tree_config = &self.tree_config.to_account_info();
        let merkle_tree = &self.merkle_tree.to_account_info();
        let tree_creator = &self.config.to_account_info();
        let payer = &self.authority.to_account_info();
        let log_wrapper = &self.log_wrapper.to_account_info();
        let compression_program = &self.compression_program.to_account_info();
        let system_program = &self.system_program.to_account_info();

        // CPI call to create the tree config
        CreateTreeConfigCpiBuilder::new(bubblegum_program)
            .tree_config(tree_config)
            .merkle_tree(merkle_tree)
            .payer(payer)
            .tree_creator(tree_creator)
            .log_wrapper(log_wrapper)
            .compression_program(compression_program)
            .system_program(system_program)
            .max_depth(max_depth)
            .max_buffer_size(max_buffer_size)
            .public(false)
            .invoke_signed(signer_seeds)?;          
        
        Ok(())
    }
}