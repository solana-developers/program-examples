use anchor_lang::error_code;

#[error_code]
pub enum ProgramErrorCode {
    #[msg("Invalid ed25519 instruction")]
    Invalid25519Instruction,
    #[msg("Invalid admin signature")]
    InvalidAdminSignature,
}
