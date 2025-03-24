use steel::*;

use super::TokenAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Token {
    pub name: [u8; 32],
    pub symbol: [u8; 8],
    pub uri: [u8; 128],
    pub decimals: u8
}

account!(TokenAccount, Token);
