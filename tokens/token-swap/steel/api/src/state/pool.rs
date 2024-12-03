use steel::*;

use crate::{
    consts::{AUTHORITY_SEED, LIQUIDITY_SEED},
    error::TokenSwapError,
};

use super::TokenSwapAccount;

/// Fetch PDA of the pool account.
pub fn pool_pda(amm: Pubkey, mint_a: Pubkey, mint_b: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[amm.as_ref(), mint_a.as_ref(), mint_b.as_ref()],
        &crate::id(),
    )
}

pub fn validate_pool_account(pool: &AccountInfo, mint_a: Pubkey, mint_b: Pubkey) -> ProgramResult {
    let pool_info_data = pool.as_account::<Pool>(&crate::id())?;
    pool.has_owner(&crate::id())?.has_seeds(
        &[
            pool_info_data.amm.as_ref(),
            pool_info_data.mint_a.as_ref(),
            pool_info_data.mint_b.as_ref(),
        ],
        &crate::id(),
    )?;

    if pool_info_data.mint_a != mint_a || pool_info_data.mint_b != mint_b {
        return Err(TokenSwapError::InvalidAccount.into());
    }

    Ok(())
}

pub fn pool_authority_pda(amm: Pubkey, mint_a: Pubkey, mint_b: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            amm.as_ref(),
            mint_a.as_ref(),
            mint_b.as_ref(),
            AUTHORITY_SEED,
        ],
        &crate::id(),
    )
}

pub fn validate_pool_authority(
    pool: &Pool,
    pool_authority: &AccountInfo,
    mint_a: Pubkey,
    mint_b: Pubkey,
) -> ProgramResult {
    pool_authority.has_seeds(
        &[
            pool.amm.as_ref(),
            mint_a.as_ref(),
            mint_b.as_ref(),
            AUTHORITY_SEED,
        ],
        &crate::id(),
    )?;

    Ok(())
}

pub fn mint_liquidity_pda(amm: Pubkey, mint_a: Pubkey, mint_b: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            amm.as_ref(),
            mint_a.as_ref(),
            mint_b.as_ref(),
            LIQUIDITY_SEED,
        ],
        &crate::id(),
    )
}

pub fn validate_mint_liquidity(
    pool: &Pool,
    mint_liquidity: &AccountInfo,
    mint_a: Pubkey,
    mint_b: Pubkey,
) -> ProgramResult {
    mint_liquidity
        .is_writable()?
        .has_seeds(
            &[
                pool.amm.as_ref(),
                mint_a.as_ref(),
                mint_b.as_ref(),
                LIQUIDITY_SEED,
            ],
            &crate::id(),
        )?
        .as_mint()?;

    Ok(())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Pool {
    /// Primary key of the AMM
    pub amm: Pubkey,

    /// Mint of token A
    pub mint_a: Pubkey,

    /// Mint of token B
    pub mint_b: Pubkey,

    pub pool_authority_bump: u8,
}

account!(TokenSwapAccount, Pool);
