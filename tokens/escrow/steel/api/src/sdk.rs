use steel::*;

use crate::prelude::*;

pub fn make_offer(
    maker: Pubkey,
    token_mint_a: Pubkey,
    token_mint_b: Pubkey,
    maker_token_account_a: Pubkey,
    vault: Pubkey,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(maker, true),
            AccountMeta::new_readonly(token_mint_a, false),
            AccountMeta::new_readonly(token_mint_b, false),
            AccountMeta::new_readonly(maker_token_account_a, false),
            AccountMeta::new(offer_pda(maker, id).0, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: MakeOffer {
            token_a_offered_amount: token_a_offered_amount.to_le_bytes(),
            id: id.to_le_bytes(),
            token_b_wanted_amount: token_b_wanted_amount.to_le_bytes(),
        }
        .to_bytes(),
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
    vault: Pubkey,
    id: u64,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(taker, true),
            AccountMeta::new(maker, false),
            AccountMeta::new_readonly(token_mint_a, false),
            AccountMeta::new_readonly(token_mint_b, false),
            AccountMeta::new_readonly(taker_token_account_a, false),
            AccountMeta::new_readonly(taker_token_account_b, false),
            AccountMeta::new_readonly(maker_token_account_b, false),
            AccountMeta::new(offer_pda(maker, id).0, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: TakeOffer {}.to_bytes(),
    }
}
