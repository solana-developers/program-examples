use quasar_lang::prelude::*;

/// On-chain address info account with dynamic string fields.
/// Uses Quasar's `String<P, N>` marker type for variable-length string data.
/// The lifetime `'a` is required because the generated code produces `&'a str` accessors.
///
/// Note: Quasar requires all fixed-size fields to precede dynamic (String/Vec) fields.
#[account(discriminator = 1)]
pub struct AddressInfo<'a> {
    pub house_number: u8,
    pub name: String<u8, 50>,
    pub street: String<u8, 50>,
    pub city: String<u8, 50>,
}
