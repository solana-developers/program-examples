use steel::*;

use super::SteelAccount;

/*
The #[repr()] attribute is part of Rust's core language features.
It's not from any external crate or library - it's built into the Rust compiler itself.
Purpose: This attribute is used to control the memory layout of structs.
Specifically, #[repr(packed)] tells the compiler to lay out the fields of a struct in memory without any padding between them.

Sometimes you can get error with transmutation this is due to the #[repr(C)] attribute

#[repr(C)] tells the Rust compiler to use a memory layout that's compatible with the C programming language's struct layout.
Origin: Like #[repr(packed)], #[repr(C)] is a built-in Rust attribute, part of the language itself.

By default, Rust doesn't guarantee any particular field order or padding in structs.

The compiler is free to reorder fields and add padding as it sees fit for optimization.

C-Compatible Layout: With #[repr(C)]:

Fields are laid out in the order they're written in the struct definition.
Padding is added between fields to ensure proper alignment according to C ABI rules.

#[repr(C)]: Gives a C-compatible layout, which is often sufficient and safer than packed

You can add manual padding: Adding padding fields to align the struct without using packed but that will mean paying rent for unused space

*/
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}

account!(SteelAccount, Offer);