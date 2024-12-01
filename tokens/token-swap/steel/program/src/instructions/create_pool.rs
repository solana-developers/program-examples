use {
    crate::{consts::*, state::*, SteelInstruction},
    solana_program::{msg, program_pack::Pack},
    spl_token::state::Mint,
    steel::*,
};

instruction!(SteelInstruction, CreatePool);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreatePool {}

impl CreatePool {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [amm, pool_info, pool_authority, mint_liquidity, mint_a, mint_b, pool_account_a, pool_account_b, payer, token_program, associated_token_program, system_program, rent] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.is_writable()?;
        system_program.is_program(&system_program::ID)?;
        mint_liquidity.has_seeds(
            &[
                amm.key.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                LIQUIDITY_SEED,
            ],
            program_id,
        )?;

        create_account::<Pool>(
            pool_info,
            system_program,
            payer,
            program_id,
            &[amm.key.as_ref(), mint_a.key.as_ref(), mint_b.key.as_ref()],
        )?;

        // First create the account for the Mint
        //
        let (_, bump) = Pubkey::find_program_address(
            &[
                amm.key.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                LIQUIDITY_SEED,
            ],
            program_id,
        );

        allocate_account_with_bump(
            mint_liquidity,
            system_program,
            payer,
            Mint::LEN,
            token_program.key,
            &[
                amm.key.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                LIQUIDITY_SEED,
            ],
            bump,
        )?;

        // Now initialize that account as a Mint (standard Mint)
        //
        msg!("Initializing mint account...");
        msg!("Mint: {}", mint_liquidity.key);

        initialize_mint_signed_with_bump(
            mint_liquidity,
            pool_authority,
            Some(pool_authority),
            token_program,
            rent,
            6,
            &[
                amm.key.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                LIQUIDITY_SEED,
            ],
            bump,
        )?;

        msg!("Mint success: {}", mint_liquidity.key);

        create_associated_token_account(
            payer,
            pool_authority,
            pool_account_a,
            mint_a,
            system_program,
            token_program,
            associated_token_program,
        )?;

        create_associated_token_account(
            payer,
            pool_authority,
            pool_account_b,
            mint_b,
            system_program,
            token_program,
            associated_token_program,
        )?;

        let pool = pool_info.as_account_mut::<Pool>(program_id)?;

        pool.amm = *amm.key;
        pool.mint_a = *mint_a.key;
        pool.mint_b = *mint_b.key;

        Ok(())
    }
}
