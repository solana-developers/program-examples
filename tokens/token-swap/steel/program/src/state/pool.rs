use steel::*;

use super::SteelAccount;

account!(SteelAccount, Pool);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Pool {
    /// Primary key of the AMM
    pub amm: Pubkey,

    /// Mint of token A
    pub mint_a: Pubkey,

    /// Mint of token B
    pub mint_b: Pubkey,
}
