use crate::AmountOpError;

use super::{Amount, Event};

#[derive(Default, Debug, Clone, Copy)]
pub struct Transaction {
    amount: Option<Amount>,
    disputed: bool,
    resolved: bool,
    chargebacked: bool,
}

impl Transaction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn amount(&self) -> Amount {
        self.amount.unwrap_or_default()
    }

    pub fn is_disputed(&self) -> bool {
        self.disputed
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved
    }

    pub fn is_chargebacked(&self) -> bool {
        self.chargebacked
    }

    pub fn apply(&mut self, ev: Event) -> Result<(), AmountOpError> {
        match ev {
            Event::Deposit { amount, .. } => {
                self.amount = Some(amount);
            }
            Event::Withdrawal { amount, .. } => {
                self.amount = match -amount {
                    Ok(value) => Some(value),
                    Err(err) => return Err(err),
                };
            }
            Event::Dispute { .. } => self.disputed = true,
            Event::Resolve { .. } => self.resolved = true,
            Event::Chargeback { .. } => self.chargebacked = true,
        }

        Ok(())
    }
}
