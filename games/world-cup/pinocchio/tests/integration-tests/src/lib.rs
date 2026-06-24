pub use world_cup::*;

pub mod utils;

pub mod tests {
    pub use crate::utils::{asserts, constants, idl, pda};

    pub mod utils {
        pub use crate::utils::test_helpers::*;
    }
}

#[cfg(test)]
mod test_account_meta;
#[cfg(test)]
mod test_claim;
#[cfg(test)]
mod test_close_bracket;
#[cfg(test)]
mod test_finalize;
#[cfg(test)]
mod test_init_config;
#[cfg(test)]
mod test_lock;
#[cfg(test)]
mod test_post_goals;
#[cfg(test)]
mod test_post_result;
#[cfg(test)]
mod test_refresh_score;
#[cfg(test)]
mod test_submit_bracket;
