use steel::*;

use crate::prelude::*;

pub fn make_offer(
    maker: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Instruction {
    let (maker_token_account_a, _) = Pubkey::find_program_address(
        &[maker.as_ref(), spl_token::ID.as_ref(), mint_a.as_ref()],
        &spl_token::ID,
    );

    let offer = offer_pda(maker, id).0;
    let (vault, _) = Pubkey::find_program_address(
        &[offer.as_ref(), spl_token::ID.as_ref(), mint_a.as_ref()],
        &spl_token::ID,
    );
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(maker, true),
            AccountMeta::new_readonly(mint_a, false),
            AccountMeta::new_readonly(mint_b, false),
            AccountMeta::new(maker_token_account_a, false),
            AccountMeta::new(offer, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
        ],
        data: MakeOffer {
            id: id.to_le_bytes(),
            token_a_offered_amount: token_a_offered_amount.to_le_bytes(),
            token_b_wanted_amount: token_b_wanted_amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn take_offer(
    taker: Pubkey,
    maker: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    offer: Pubkey,
) -> Instruction {
    let (taker_token_account_a, _) = Pubkey::find_program_address(
        &[taker.as_ref(), spl_token::ID.as_ref(), mint_a.as_ref()],
        &spl_token::ID,
    );
    let (taker_token_account_b, _) = Pubkey::find_program_address(
        &[taker.as_ref(), spl_token::ID.as_ref(), mint_b.as_ref()],
        &spl_token::ID,
    );
    let (maker_token_account_b, _) = Pubkey::find_program_address(
        &[maker.as_ref(), spl_token::ID.as_ref(), mint_b.as_ref()],
        &spl_token::ID,
    );
    let (vault, _) = Pubkey::find_program_address(
        &[offer.as_ref(), spl_token::ID.as_ref(), mint_a.as_ref()],
        &spl_token::ID,
    );
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(taker, true),
            AccountMeta::new(maker, false),
            AccountMeta::new_readonly(mint_a, false),
            AccountMeta::new_readonly(mint_b, false),
            AccountMeta::new(taker_token_account_a, false),
            AccountMeta::new(taker_token_account_b, false),
            AccountMeta::new(maker_token_account_b, false),
            AccountMeta::new(offer, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
        ],
        data: TakerOffer {}.to_bytes(),
    }
}
