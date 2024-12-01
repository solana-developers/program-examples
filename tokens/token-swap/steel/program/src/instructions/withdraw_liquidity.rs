use {
    crate::{consts::*, SteelInstruction},
    fixed::types::I64F64,
    steel::*,
};

instruction!(SteelInstruction, WithdrawLiquidity);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WithdrawLiquidity {
    pub amount: u64,
}

impl WithdrawLiquidity {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'_>],
        data: &[u8],
    ) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let amount = args.amount;

        let [amm_info, _pool, pool_authority, depositor, mint_liquidity, mint_a, mint_b, pool_account_a, pool_account_b, depositor_account_liquidity, depositor_account_a, depositor_account_b, _payer, token_program, _associated_token_program, _system_program, _rent] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

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


        // Transfer tokens from the pool
        let amount_a = I64F64::from_num(amount)
            .checked_mul(I64F64::from_num(pool_account_a.as_token_account()?.amount))
            .unwrap()
            .checked_div(I64F64::from_num(
                mint_liquidity.as_mint()?.supply + MINIMUM_LIQUIDITY,
            ))
            .unwrap()
            .floor()
            .to_num::<u64>();

        transfer_signed_with_bump(
            pool_authority,
            pool_account_a,
            depositor_account_a,
            token_program,
            amount_a,
            pool_seeds,
            bump,
        )?;

        // Transfer tokens from the pool
        let amount_b = I64F64::from_num(amount)
            .checked_mul(I64F64::from_num(pool_account_b.as_token_account()?.amount))
            .unwrap()
            .checked_div(I64F64::from_num(
                mint_liquidity.as_mint()?.supply + MINIMUM_LIQUIDITY,
            ))
            .unwrap()
            .floor()
            .to_num::<u64>();

        transfer_signed_with_bump(
            pool_authority,
            pool_account_b,
            depositor_account_b,
            token_program,
            amount_b,
            pool_seeds,
            bump
        )?;

        burn(
            depositor_account_liquidity,
            mint_liquidity,
            depositor,
            token_program,
            amount,
            // pool_seeds,
            // bump
        )
    }
}
