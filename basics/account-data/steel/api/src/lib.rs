pub mod consts;
pub mod instruction;
pub mod sdk;
pub mod state;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*;
}

use steel::*;

// Set your Program ID
declare_id!("Dw6Yq7TZSHdaqB2nKjsxuDrdp5xYCuZaVKFZb5vp5Y4Y");
