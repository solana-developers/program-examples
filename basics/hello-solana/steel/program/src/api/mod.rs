pub mod instruction;
pub mod sdk;

pub mod prelude {
    pub use super::instruction::*;
    pub use super::sdk::*;
    pub use super::ID;
}

use steel::*;

// TODO Set program id
declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
