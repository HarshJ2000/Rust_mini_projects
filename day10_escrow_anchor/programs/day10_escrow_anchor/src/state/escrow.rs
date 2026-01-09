use anchor_lang::prelude::*;

// Storing states required for Escrow inside an account
#[account]
pub struct EscrowState {
    pub initializer: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub expiry: i64,
    pub bump: u8,
    pub state: EscrowStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Clone, Eq, Debug)]
pub enum EscrowStatus {
    Initialized,
    Deposited,
    Completed,
    Refunded,
    Cancelled,
}

pub trait EscrowLifecycle {
    fn can_deposit(&self) -> bool;
    fn can_withdraw(&self, now: i64) -> bool;
    fn can_complete(&self) -> bool;
    fn can_cancel(&self) -> bool;
}

impl EscrowLifecycle for EscrowState {
    fn can_deposit(&self) -> bool {
        self.state == EscrowStatus::Initialized
    }
    fn can_withdraw(&self, now: i64) -> bool {
        self.state == EscrowStatus::Deposited && now >= self.expiry
    }
    fn can_complete(&self) -> bool {
        self.state == EscrowStatus::Deposited
    }
    fn can_cancel(&self) -> bool {
        matches!(
            self.state,
            EscrowStatus::Initialized | EscrowStatus::Deposited
        )
    }
}
