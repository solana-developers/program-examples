use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Frezze period not passed")]
    FreezePeriodNotPassed,
    #[msg("Max stake reached")]
    MaxStakeReached,
}