use anchor_lang::prelude::*;

use instructions::*;
mod instructions;

declare_id!("BJHEDXSQfD9kBFvhw8ZCGmPFRihzvbMoxoHUKpXdpn4D");

#[program]
pub mod metadata {
    use super::*;

    pub fn initialize(context: Context<Initialize>, args: TokenMetadataArgs) -> Result<()> {
        process_initialize(context, args)
    }

    pub fn update_field(context: Context<UpdateField>, args: UpdateFieldArgs) -> Result<()> {
        process_update_field(context, args)
    }

    pub fn remove_key(context: Context<RemoveKey>, key: String) -> Result<()> {
        process_remove_key(context, key)
    }

    pub fn emit(context: Context<Emit>) -> Result<()> {
        process_emit(context)
    }

    pub fn update_authority(context: Context<UpdateAuthority>) -> Result<()> {
        process_update_authority(context)
    }
}
