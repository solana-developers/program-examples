pub mod consts;
pub mod instruction;
pub mod state;
pub mod sdk;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*; 
}

use steel::*;

declare_id!("FFJjpuXzZeBM8k1aTzzUrV9tgboUWtAaKH6U2QudoH2K"); 
