use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Invalid escrow state for this operation!!!!")]
    InvalidState,

    #[msg("Amount cannot be zero!!!!")]
    InvalidAmount,

    #[msg("Escrow is expired!!!!!")]
    ExpiredEscrow,

    #[msg("Unauthorized User!!!!!!")]
    Unauthorized,

    #[msg("Escrow Already Initialized!!!!!!")]
    AlreadyInitialized,
}
