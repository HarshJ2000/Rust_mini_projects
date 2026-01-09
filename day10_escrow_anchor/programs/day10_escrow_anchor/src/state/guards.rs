use crate::errors::EscrowError;
use anchor_lang::prelude::*;

pub fn assert_initializer(signer: &Signer, initializer: &Pubkey) -> Result<()> {
    require_keys_eq!(signer.key(), *initializer, EscrowError::Unauthorized);
    Ok(())
}
