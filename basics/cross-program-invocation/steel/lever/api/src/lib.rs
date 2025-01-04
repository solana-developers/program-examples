pub mod consts;
pub mod error;
pub mod instruction;
pub mod sdk;
pub mod state;
pub mod utils;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*;
    pub use crate::utils::*;
}

use steel::*;

// TODO Set program id
declare_id!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");
