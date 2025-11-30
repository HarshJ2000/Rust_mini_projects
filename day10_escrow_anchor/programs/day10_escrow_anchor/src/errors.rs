use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Amount cannot be zero!!!!")]
    InvalidAmount,

    #[msg("Escrow is expired!!!!!")]
    ExpiredEscrow,

    #[msg("Unauthorized User!!!!!!")]
    Unauthorized,
}
