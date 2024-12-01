use {
    crate::{consts::*, error::*, state::*, SteelInstruction},
    fixed::types::I64F64,
    steel::*,
};

instruction!(SteelInstruction, SwapExactTokensForTokens);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SwapExactTokensForTokens {
    pub swap_a: u8,
    pub input_amount: u64,
    pub min_output_amount: u64,
}

impl SwapExactTokensForTokens {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'_>],
        data: &[u8],
    ) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let swap_a = args.swap_a.to_bool()?;
        let input_amount = args.input_amount;
        let min_output_amount = args.min_output_amount;

        let [amm_info, _pool, pool_authority, trader, mint_a, mint_b, pool_account_a, pool_account_b, trader_account_a, trader_account_b, payer, token_program, associated_token_program, system_program, _rent] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Prevent depositing assets the depositor does not own
        let input = if swap_a && input_amount > trader_account_a.as_token_account()?.amount {
            trader_account_a.as_token_account()?.amount
        } else if !swap_a && input_amount > trader_account_b.as_token_account()?.amount {
            trader_account_b.as_token_account()?.amount
        } else {
            input_amount
        };

        let amm = amm_info.as_account::<Amm>(program_id)?;
        let taxed_input = input - input * amm.fee as u64 / 10000;

        let pool_a = pool_account_a.as_token_account()?;
        let pool_b = pool_account_b.as_token_account()?;

        let output = if swap_a {
            I64F64::from_num(taxed_input)
                .checked_mul(I64F64::from_num(pool_b.amount))
                .unwrap()
                .checked_div(
                    I64F64::from_num(pool_a.amount)
                        .checked_add(I64F64::from_num(taxed_input))
                        .unwrap(),
                )
                .unwrap()
        } else {
            I64F64::from_num(taxed_input)
                .checked_mul(I64F64::from_num(pool_a.amount))
                .unwrap()
                .checked_div(
                    I64F64::from_num(pool_b.amount)
                        .checked_add(I64F64::from_num(taxed_input))
                        .unwrap(),
                )
                .unwrap()
        }
        .to_num::<u64>();

        if output < min_output_amount {
            return Err(SteelError::OutputTooSmall.into());
        }

        // Compute the invariant before the trade
        let invariant = pool_a.amount * pool_b.amount;

        let pool_seeds = &[
            amm_info.key.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            AUTHORITY_SEED,
        ];

        let (_, bump) = Pubkey::find_program_address(
            &[
                amm_info.key.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                AUTHORITY_SEED,
            ],
            program_id,
        );

        // init if needed
        if trader_account_a.lamports() == 0 {
            create_associated_token_account(
                payer,
                trader,
                trader_account_a,
                mint_a,
                system_program,
                token_program,
                associated_token_program,
            )?;
        }

        // init if needed
        if trader_account_b.lamports() == 0 {
            create_associated_token_account(
                payer,
                trader,
                trader_account_b,
                mint_b,
                system_program,
                token_program,
                associated_token_program,
            )?;
        }

        // Transfer tokens to the pool
        if swap_a {
            transfer(
                trader,
                trader_account_a,
                pool_account_a,
                token_program,
                input,
            )?;
            transfer_signed_with_bump(
                pool_authority,
                pool_account_b,
                trader_account_b,
                token_program,
                output,
                pool_seeds,
                bump
            )?;
        } else {
            transfer(
                trader,
                trader_account_b,
                pool_account_b,
                token_program,
                input,
            )?;
            transfer_signed_with_bump(
                pool_authority,
                pool_account_a,
                trader_account_a,
                token_program,
                output,
                pool_seeds,
                bump
            )?;
        }

        solana_program::msg!(
            "Traded {} tokens ({} after fees) for {}",
            input,
            taxed_input,
            output
        );

        // Verify the invariant still holds
        // Reload accounts because of the CPIs
        // We tolerate if the new invariant is higher because it means a rounding error for LPs
        let pool_a = pool_account_a.as_token_account()?;
        let pool_b = pool_account_b.as_token_account()?;

        if invariant > pool_a.amount * pool_b.amount {
            return Err(SteelError::InvariantViolated.into());
        }

        Ok(())
    }
}

trait Boolean {
    fn to_bool(&self) -> Result<bool, ProgramError>;
}

impl Boolean for u8 {
    fn to_bool(&self) -> Result<bool, ProgramError> {
        match *self {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}
