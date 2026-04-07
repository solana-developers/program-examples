use anchor_lang::prelude::*;

use instructions::*;
pub mod instructions;

declare_id!("DY2T6zhjngLvpkCMReQFzMHc6g4d4bqQWznntroEVDhG");

#[program]
pub mod metadata {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args: TokenMetadataArgs) -> Result<()> {
        process_initialize(ctx, args)
    }

    pub fn update_field(ctx: Context<UpdateField>, args: UpdateFieldArgs) -> Result<()> {
        process_update_field(ctx, args)
    }

    pub fn remove_key(ctx: Context<RemoveKey>, key: String) -> Result<()> {
        process_remove_key(ctx, key)
    }

    pub fn emit(ctx: Context<Emit>) -> Result<()> {
        process_emit(ctx)
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>) -> Result<()> {
        process_update_authority(ctx)
    }
}
