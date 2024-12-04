pub mod instruction;
pub mod sdk;

pub mod prelude {
    pub use crate::instruction::*;
    pub use crate::sdk::*;
}

use steel::*;

// TODO Set program id
declare_id!("Bi5N7SUQhpGknVcqPTzdFFVueQoxoUu8YTLz75J6fT8A");
