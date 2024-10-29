use steel::*;

use crate::prelude::*;

pub fn make_offer(
    maker: Pubkey,
    token_mint_a: Pubkey,
    token_mint_b: Pubkey,
    maker_token_account_a: Pubkey,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Instruction {
    let (offer_key, bump) = Pubkey::find_program_address(
        &[OFFER, maker.as_ref(), &id.to_le_bytes()],
        &crate::ID,
    );
    let vault = spl_associated_token_account::get_associated_token_address(&offer_key, &token_mint_a);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(maker, true),
            AccountMeta::new_readonly(token_mint_a, false),
            AccountMeta::new_readonly(token_mint_b, false),
            AccountMeta::new(maker_token_account_a, false),
            AccountMeta::new(offer_key, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: MakeOffer {
            id,
            token_a_offered_amount,
            token_b_wanted_amount,
            bump
        }.to_bytes(),
    }
}


pub fn take_offer(
    taker: Pubkey,
    maker: Pubkey,
    token_mint_a: Pubkey,
    token_mint_b: Pubkey,
    taker_token_account_a: Pubkey,
    taker_token_account_b: Pubkey,
    maker_token_account_b: Pubkey,
    id: u64,
) -> Instruction {
    let (offer_key, _) = offer_pda(maker, id);
    let vault = spl_associated_token_account::get_associated_token_address(&offer_key, &token_mint_a);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(taker, true),
            AccountMeta::new(maker, false),
            AccountMeta::new_readonly(token_mint_a, false),
            AccountMeta::new_readonly(token_mint_b, false),
            AccountMeta::new(taker_token_account_a, false),
            AccountMeta::new(taker_token_account_b, false),
            AccountMeta::new(maker_token_account_b, false),
            AccountMeta::new(offer_key, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: TakeOffer {}.to_bytes(),
    }
}

//put this is in best file
// Helper function to get the PDA for an offer
pub fn get_offer_address(maker: &Pubkey, id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[OFFER, maker.as_ref(), &id.to_le_bytes()],
        &crate::ID,
    )
}

pub fn get_offer_address_from_seeds(seeds: &[&[u8]]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        seeds,
        &crate::ID,
    )
}

// Helper function to get the vault address for an offer
pub fn get_vault_address(offer: &Pubkey, token_mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(offer, token_mint)
}