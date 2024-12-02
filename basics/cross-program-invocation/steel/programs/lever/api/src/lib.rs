pub mod instruction;
pub mod state;

use steel::*;

declare_id!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");

pub mod prelude {
    pub use crate::instruction::*;
    pub use crate::state::*;
}
