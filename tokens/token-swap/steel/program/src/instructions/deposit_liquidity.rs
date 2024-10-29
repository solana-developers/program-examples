use {
    crate::{consts::*, error::*, state::*, SteelInstruction},
    fixed::types::I64F64,
    solana_program::msg,
    steel::*,
};

instruction!(SteelInstruction, DepositLiquidity);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DepositLiquidity {
    pub amount_a: u64,
    pub amount_b: u64,
}

impl DepositLiquidity {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'_>],
        data: &[u8],
    ) -> ProgramResult {
        let args = DepositLiquidity::try_from_bytes(data)?;

        let amount_a = args.amount_a;
        let amount_b = args.amount_b;

        let [_amm, pool, pool_authority, depositor, mint_liquidity, mint_a, mint_b, pool_account_a, pool_account_b, depositor_account_liquidity, depositor_account_a, depositor_account_b, payer, token_program, associated_token_program, system_program, _rent] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if depositor_account_liquidity.lamports() == 0 {
            create_associated_token_account(
                payer,
                depositor,
                depositor_account_liquidity,
                mint_liquidity,
                system_program,
                token_program,
                associated_token_program,
            )?;
        }

        // Prevent depositing assets the depositor does not own
        let mut amount_a = if amount_a > depositor_account_a.as_token_account()?.amount {
            depositor_account_a.as_token_account()?.amount
        } else {
            amount_a
        };

        let mut amount_b = if amount_b > depositor_account_b.as_token_account()?.amount {
            depositor_account_a.as_token_account()?.amount
        } else {
            amount_b
        };

        // Making sure they are provided in the same proportion as existing liquidity
        let pool_a = pool_account_a.as_token_account()?;
        let pool_b = pool_account_b.as_token_account()?;

        // Defining pool creation like this allows attackers to frontrun pool creation with bad ratios
        let pool_creation = pool_a.amount == 0 && pool_b.amount == 0;

        (amount_a, amount_b) = if pool_creation {
            // Add as is if there is no liquidity
            (amount_a, amount_b)
        } else {
            let ratio = I64F64::from_num(pool_a.amount)
                .checked_mul(I64F64::from_num(pool_b.amount))
                .unwrap();
            if pool_a.amount > pool_b.amount {
                (
                    I64F64::from_num(amount_b)
                        .checked_mul(ratio)
                        .unwrap()
                        .to_num::<u64>(),
                    amount_b,
                )
            } else {
                (
                    amount_a,
                    I64F64::from_num(amount_a)
                        .checked_div(ratio)
                        .unwrap()
                        .to_num::<u64>(),
                )
            }
        };

        // Computing the amount of liquidity about to be deposited
        let mut liquidity = I64F64::from_num(amount_a)
            .checked_mul(I64F64::from_num(amount_b))
            .unwrap()
            .sqrt()
            .to_num::<u64>();

        // Lock some minimum liquidity on the first deposit
        if pool_creation {
            if liquidity < MINIMUM_LIQUIDITY {
                return Err(SteelError::DepositTooSmall.into());
            }

            liquidity -= MINIMUM_LIQUIDITY;
        }

        // Transfer tokens to the pool
        transfer(
            depositor,
            depositor_account_a,
            pool_account_a,
            token_program,
            amount_a,
        )?;
        transfer(
            depositor,
            depositor_account_b,
            pool_account_b,
            token_program,
            amount_b,
        )?;

        let pool = pool.as_account::<Pool>(program_id)?;

        msg!("pool amm: {}", pool.amm.to_string());

        let (_, bump) = Pubkey::find_program_address(
            &[
                pool.amm.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                AUTHORITY_SEED,
            ],
            program_id,
        );

        mint_to_signed_with_bump(
            mint_liquidity,
            depositor_account_liquidity,
            pool_authority,
            token_program,
            liquidity,
            &[
                pool.amm.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                AUTHORITY_SEED,
            ],
            bump,
        )
    }
}
