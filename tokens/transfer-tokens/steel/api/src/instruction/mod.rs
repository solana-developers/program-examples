pub mod create;
pub mod mint_nft;
pub mod mint_spl;
pub mod transfer;

pub use create::*;
pub use mint_nft::*;
pub use mint_spl::*;
pub use transfer::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    Create = 0,
    MintNft = 1,
    MintSpl = 2,
    TransferTokens = 3,
}
