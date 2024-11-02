use {
    crate::{state::*, EscrowInstruction},
    steel::*,
};

instruction!(EscrowInstruction, MakeOffer);
//  MakeOffer Instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MakeOffer {
    pub id: u64,
    pub token_a_offered_amount: u64,
    pub token_b_wanted_amount: u64,
}

impl MakeOffer {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = MakeOffer::try_from_bytes(data)?;
        let [
            // accounts order
            offer_info,
            token_mint_a,
            token_mint_b,
            maker_token_account_a,
            vault,
            maker,
            payer,
            token_program,
            associated_token_program,
            system_program
        ] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // make sure the maker is a signer
        //
        maker.is_signer()?;

        // make sure the maker is a writable signer
        //
        maker_token_account_a.as_associated_token_account(maker.key, token_mint_a.key)?;

        let offer_seeds = &[Offer::SEEDS, maker.key.as_ref(), &args.id.to_le_bytes()];
        let (offer_address, offer_bump) = Pubkey::find_program_address(offer_seeds, &crate::ID);

        // check we have the right address, derived from the provided seeds
        //
        offer_info.has_address(&offer_address)?;

        // create the offer account
        //
        create_account::<Offer>(offer_info, system_program, payer, &crate::ID, offer_seeds)?;

        // create the vault token account, where the maker will send funds to
        //
        create_associated_token_account(
            payer,
            offer_info,
            vault,
            token_mint_a,
            system_program,
            token_program,
            associated_token_program,
        )?;

        // validate the vault the maker token a will be sent to
        //
        vault.as_associated_token_account(offer_info.key, token_mint_a.key)?;

        // maker transfer token a to the vault
        //
        transfer(
            maker,
            maker_token_account_a,
            vault,
            token_program,
            args.token_a_offered_amount,
        )?;

        let offer = offer_info.as_account_mut::<Offer>(&crate::ID)?;

        // we record our offer data
        //
        *offer = Offer {
            id: args.id,
            bump: offer_bump,
            maker: *maker.key,
            token_b_wanted_amount: args.token_b_wanted_amount,
            token_mint_a: *token_mint_a.key,
            token_mint_b: *token_mint_b.key,
        };

        solana_program::msg!(
            "Token A balance in vault: {}",
            vault.as_token_account()?.amount
        );

        Ok(())
    }
}
