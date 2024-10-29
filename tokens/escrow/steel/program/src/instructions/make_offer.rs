use {
    crate::{state::*, SteelInstruction},
    steel::*,
};

instruction!(SteelInstruction, MakeOffer);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MakeOffer {
    pub id: u64,
    pub token_a_offered_amount: u64,
    pub token_b_wanted_amount: u64,
}

impl MakeOffer {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = MakeOffer::try_from_bytes(data)?;
        let [offer_info, token_mint_a, token_mint_b, maker_token_account_a, vault, maker, payer, token_program, associated_token_program, system_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        maker.is_writable()?;
        system_program.is_program(&system_program::ID)?;

        create_account::<Offer>(
            offer_info,
            system_program,
            payer,
            program_id,
            &[
                b"offer",
                maker.key.as_ref(),
                args.id.to_le_bytes().as_ref(),
            ],
        )?;

        create_associated_token_account(
            payer,
            offer_info,
            vault,
            token_mint_a,
            system_program,
            token_program,
            associated_token_program,
        )?;

        transfer(
            maker,
            maker_token_account_a,
            vault,
            token_program,
            args.token_a_offered_amount,
        )?;

        let offer = offer_info.as_account_mut::<Offer>(program_id)?;

        let (_, bump) = Pubkey::find_program_address(
            &[
                b"offer",
                maker.key.as_ref(),
                offer.id.to_be_bytes().as_ref(),
            ],
            program_id,
        );

        offer.id = args.id;
        offer.maker = *maker.key;
        offer.token_mint_a = *token_mint_a.key;
        offer.token_mint_b = *token_mint_b.key;
        offer.token_b_wanted_amount = args.token_b_wanted_amount;
        offer.bump = bump;

        solana_program::msg!("Token A balance in vault: {}", vault.as_token_account()?.amount);

        Ok(())
    }
}
