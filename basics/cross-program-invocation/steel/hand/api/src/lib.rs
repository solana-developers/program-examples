pub mod instruction;
pub mod sdk;

pub mod prelude {
    pub use crate::instruction::*;
    pub use crate::sdk::*;
}

use steel::*;

// TODO Set program id
declare_id!("2LSt6uKm3YpTogXwEUvSNWLskMdsA6uyNkBFhwkS7sx4");
