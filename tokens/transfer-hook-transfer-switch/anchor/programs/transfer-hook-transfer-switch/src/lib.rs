use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod transfer_hook_transfer_switch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let transfer_switch = &mut ctx.accounts.transfer_switch;
        transfer_switch.is_enabled = true;
        transfer_switch.authority = ctx.accounts.payer.key();
        Ok(())
    }

    pub fn toggle_transfer(ctx: Context<ToggleTransfer>, enable: bool) -> Result<()> {
        let transfer_switch = &mut ctx.accounts.transfer_switch;
        transfer_switch.is_enabled = enable;
        Ok(())
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        let transfer_switch = &ctx.accounts.transfer_switch;
        require!(transfer_switch.is_enabled, TransferSwitchError::TransfersDisabled);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 1 + 32)]
    pub transfer_switch: Account<'info, TransferSwitch>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ToggleTransfer<'info> {
    #[account(mut, has_one = authority)]
    pub transfer_switch: Account<'info, TransferSwitch>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferHook<'info> {
    pub transfer_switch: Account<'info, TransferSwitch>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub to: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct TransferSwitch {
    pub is_enabled: bool,
    pub authority: Pubkey,
}

#[error_code]
pub enum TransferSwitchError {
    #[msg("Transfers are currently disabled")]
    TransfersDisabled,
}