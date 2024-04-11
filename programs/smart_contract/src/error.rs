use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The specified DID already exists.")]
    DidAlreadyExists,
    #[msg("The specified DID does not exist.")]
    DidNotExists,
    #[msg("PDA mismatch error.")]
    PdaMismatch,
}
