use borsh::{BorshDeserialize, BorshSerialize};
use steel::*;
use solana_program::{program::invoke, rent::Rent, system_instruction};

#[program]
mod account_data_program {
    use super::*;

    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> ProgramResult {
        let address_info = AddressInfo::new(name, house_number, street, city);

        let account_span = address_info.try_to_vec()?.len();
        let lamports_required = Rent::get()?.minimum_balance(account_span);

        let create_account_ix = system_instruction::create_account(
            ctx.accounts.payer.key,
            ctx.accounts.address_info_account.key,
            lamports_required,
            account_span as u64,
            ctx.program_id,
        );

        invoke(
            &create_account_ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.address_info_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        address_info.serialize(&mut &mut ctx.accounts.address_info_account.data.borrow_mut()[..])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    pub address_info_account: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}

impl AddressInfo {
    pub fn new(name: String, house_number: u8, street: String, city: String) -> Self {
        AddressInfo {
            name,
            house_number,
            street,
            city,
        }
    }
}
