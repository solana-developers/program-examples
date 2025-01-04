pub mod instruction;
pub mod sdk;

pub mod prelude {
    pub use crate::instruction::*;
    pub use crate::sdk::*;
}

use steel::*;

// TODO: Set program id
declare_id!("FNDnd3ZJptKromzx7h71o67AcR1emryyJPb9LjS8WPVw");
