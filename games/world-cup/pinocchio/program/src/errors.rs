use codama::CodamaErrors;
use pinocchio::error::ProgramError;
use thiserror::Error;

impl From<WorldCupError> for ProgramError {
    fn from(e: WorldCupError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[cfg(test)]
impl TryFrom<u32> for WorldCupError {
    type Error = u32;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        match code {
            100 => Ok(Self::NotSigner),
            101 => Ok(Self::AccountNotWritable),
            102 => Ok(Self::NotSystemProgram),
            103 => Ok(Self::NotEnoughAccountKeys),
            104 => Ok(Self::InvalidInstruction),
            105 => Ok(Self::InvalidAccountData),
            106 => Ok(Self::InvalidAccountDiscriminator),
            107 => Ok(Self::ArithmeticOverflow),
            108 => Ok(Self::NotProgramOwned),
            200 => Ok(Self::InvalidConfigPda),
            201 => Ok(Self::ConfigAlreadyExists),
            202 => Ok(Self::Unauthorized),
            203 => Ok(Self::InvalidState),
            204 => Ok(Self::RegistrationClosed),
            205 => Ok(Self::NotYetLocked),
            206 => Ok(Self::InvalidLockTs),
            300 => Ok(Self::InvalidBracketPda),
            301 => Ok(Self::BracketAlreadyExists),
            302 => Ok(Self::InvalidPick),
            303 => Ok(Self::AlreadyFolded),
            400 => Ok(Self::InvalidOraclePda),
            401 => Ok(Self::InvalidGame),
            402 => Ok(Self::InvalidResult),
            403 => Ok(Self::FeederNotDecided),
            404 => Ok(Self::ResultAlreadyPosted),
            405 => Ok(Self::GoalsAlreadyPosted),
            500 => Ok(Self::InvalidVaultPda),
            501 => Ok(Self::OracleNotComplete),
            502 => Ok(Self::NotFullyRefreshed),
            503 => Ok(Self::BracketNotBest),
            504 => Ok(Self::NotWinner),
            505 => Ok(Self::AlreadyClaimed),
            600 => Ok(Self::InvalidEventAuthority),
            601 => Ok(Self::InvalidEventData),
            _ => Err(code),
        }
    }
}

/// Program-specific error codes for the world-cup program.
///
/// - **100--199**: Generic account and data validation errors.
/// - **200--299**: Config / tournament-lifecycle errors.
/// - **300--399**: Bracket errors.
/// - **400--499**: Oracle errors.
/// - **500--599**: Finalize / claim errors.
/// - **600--699**: Event emission errors.
#[derive(Debug, Copy, Clone, Error, CodamaErrors)]
pub enum WorldCupError {
    // --- Generic errors (100--199) ---
    #[error("Account must be a signer")]
    NotSigner = 100,
    #[error("Account must be writable")]
    AccountNotWritable,
    #[error("Expected system program")]
    NotSystemProgram,
    #[error("Not enough account keys provided")]
    NotEnoughAccountKeys,
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Invalid account data")]
    InvalidAccountData,
    #[error("Invalid account discriminator")]
    InvalidAccountDiscriminator,
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    #[error("Account is not owned by this program")]
    NotProgramOwned,

    // --- Config / lifecycle errors (200--299) ---
    #[error("Invalid config PDA derivation")]
    InvalidConfigPda = 200,
    #[error("Config account already exists")]
    ConfigAlreadyExists,
    #[error("Signer is not the tournament admin")]
    Unauthorized,
    #[error("Instruction not allowed in the current tournament state")]
    InvalidState,
    #[error("Registration has closed (kickoff reached)")]
    RegistrationClosed,
    #[error("Lock time has not been reached yet")]
    NotYetLocked,
    #[error("Lock timestamp must be in the future")]
    InvalidLockTs,

    // --- Bracket errors (300--399) ---
    #[error("Invalid bracket PDA derivation")]
    InvalidBracketPda = 300,
    #[error("Bracket already exists for this wallet")]
    BracketAlreadyExists,
    #[error("Bracket pick is out of range or inconsistent")]
    InvalidPick,
    #[error("Bracket has already been folded into the final tally")]
    AlreadyFolded,

    // --- Oracle errors (400--499) ---
    #[error("Invalid oracle PDA derivation")]
    InvalidOraclePda = 400,
    #[error("Game index is out of range")]
    InvalidGame,
    #[error("Result is inconsistent with feeder games")]
    InvalidResult,
    #[error("A feeder game has not been decided yet")]
    FeederNotDecided,
    #[error("Result for this game has already been posted")]
    ResultAlreadyPosted,
    #[error("Round-of-32 goal total has already been posted")]
    GoalsAlreadyPosted,

    // --- Finalize / claim errors (500--599) ---
    #[error("Invalid pot vault PDA derivation")]
    InvalidVaultPda = 500,
    #[error("Oracle is not complete (all games + goals required)")]
    OracleNotComplete,
    #[error("Not every bracket has been refreshed at the final state")]
    NotFullyRefreshed,
    #[error("Provided bracket does not match the winning key")]
    BracketNotBest,
    #[error("Signer is not the recorded winner")]
    NotWinner,
    #[error("Pot has already been claimed or released")]
    AlreadyClaimed,

    // --- Event errors (600--699) ---
    #[error("Invalid event authority PDA")]
    InvalidEventAuthority = 600,
    #[error("Invalid event data")]
    InvalidEventData,
}
