use anchor_lang::prelude::*;

use crate::{errors::*, state::Amm};

pub fn handle_create_amm(mut context: Context<CreateAmmAccountConstraints>, id: Pubkey, fee: u16) -> Result<()> {
    let amm = &mut context.accounts.amm;
    amm.id = id;
    amm.admin = context.accounts.admin.key();
    amm.fee = fee;

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: Pubkey, fee: u16)]
pub struct CreateAmmAccountConstraints<'info> {
    #[account(
        init,
        payer = payer,
        space = Amm::DISCRIMINATOR.len() + Amm::INIT_SPACE,
        seeds = [
            id.as_ref()
        ],
        bump,
        constraint = fee < 10000 @ TutorialError::InvalidFee,
    )]
    pub amm: Account<'info, Amm>,

    /// The admin of the AMM
    /// CHECK: Read only, delegatable creation
    pub admin: AccountInfo<'info>,

    /// The account paying for all rents
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Solana ecosystem accounts
    pub system_program: Program<'info, System>,
}
