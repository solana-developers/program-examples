use quasar_lang::prelude::*;

/// User favourites stored on-chain.
///
/// The Anchor version also stores `hobbies: Vec<String>`, but Quasar doesn't
/// support nested dynamic types (Vec<String>). We keep number + color, which
/// demonstrates fixed + dynamic field mixing in Quasar.
#[account(discriminator = 1)]
pub struct Favorites<'a> {
    pub number: u64,
    pub color: String<u8, 50>,
}
