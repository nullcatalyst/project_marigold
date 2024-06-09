use crate::model::{Account, Amount, AmountOpError, Event};

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

    pub fn apply(&mut self, ev: Event, account: &mut Account) -> Result<(), AmountOpError> {
        match ev {
            Event::Deposit { amount, .. } => {
                // This breaks our assumption that this isn't an externally facing service, and that
                // all of the input is valid.
                if self.amount.is_some() {
                    panic!("Transaction already has an amount");
                }

                let new_amount = amount;
                account.deposit(amount)?;

                self.amount = Some(new_amount);
            }
            Event::Withdrawal { amount, .. } => {
                // This breaks our assumption that this isn't an externally facing service, and that
                // all of the input is valid.
                if self.amount.is_some() {
                    panic!("Transaction already has an amount");
                }

                let new_amount = (-amount)?;
                account.withdraw(amount)?;

                self.amount = Some(new_amount);
            }
            Event::Dispute { .. } => self.disputed = true,
            Event::Resolve { .. } => self.resolved = true,
            Event::Chargeback { .. } => {
                self.chargebacked = true;
                account.lock();
            }
        }

        Ok(())
    }
}
