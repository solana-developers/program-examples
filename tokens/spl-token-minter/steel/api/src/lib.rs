pub mod consts;
pub mod instruction;
pub mod sdk;
pub mod utils;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::utils::*;
}

use steel::*;

// TODO Set program id
declare_id!("8V26fyhrQobKbvkRCV3KvT6jZQLzviovdARfGrw8kUdG");
