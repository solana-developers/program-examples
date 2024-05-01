use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_fee_set, Mint, Token2022, TransferFeeSetTransferFee};

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
}

// Note that there is a 2 epoch delay from when new fee updates take effect
// This is a safely feature built into the extension
// https://github.com/solana-labs/solana-program-library/blob/master/token/program-2022/src/extension/transfer_fee/processor.rs#L92-L109
pub fn process_update_fee(
    ctx: Context<UpdateFee>,
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
) -> Result<()> {
    transfer_fee_set(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferFeeSetTransferFee {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        transfer_fee_basis_points, // transfer fee basis points (% fee per transfer)
        maximum_fee,               // maximum fee (maximum units of token per transfer)
    )?;
    Ok(())
}
