use anchor_lang::error_code;

#[error_code]
pub enum StakingErrors {
    #[msg("The StakingRules Address you have provided is not the right one.")]
    InvalidStakingRules,
    #[msg("The StakingAccount Address you have provided is not the right one.")]
    InvalidStakingAccount,
    #[msg("The Onwer Address you have provided is not the right one.")]
    InvalidOwner,
    #[msg("The Collection Address of the NFT you provided is not the right one.")]
    InvalidCollection,
    #[msg("The decimals of the Reward_per_unix you provided is not the right one.")]
    InvalidDecimals,
    #[msg("The Reward Mint Address you provided is not the right one.")]
    InvalidRewardMint,
}