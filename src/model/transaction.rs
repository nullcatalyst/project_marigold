use super::{Account, Amount, AmountOpError, Event};

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

                if self.disputed && !self.resolved {
                    if self.chargebacked {
                        account.chargeback(new_amount, false)?;
                    } else {
                        account.hold(new_amount)?;
                    }
                }

                self.amount = Some(new_amount);
            }
            Event::Withdrawal { amount, .. } => {
                // This breaks our assumption that this isn't an externally facing service, and that
                // all of the input is valid.
                if self.amount.is_some() {
                    panic!("Transaction already has an amount");
                }

                let new_amount = (-amount)?;
                if !self.chargebacked {
                    account.withdraw(amount)?;
                }

                if self.disputed && !self.resolved {
                    if self.chargebacked {
                        account.chargeback(new_amount, false)?;
                    } else {
                        account.hold(new_amount)?;
                    }
                }

                self.amount = Some(new_amount);
            }
            Event::Dispute { .. } => {
                self.disputed = true;

                if let Some(amount) = self.amount {
                    match (self.resolved, self.chargebacked) {
                        (false, false) => account.hold(amount)?,
                        (true, false) => account.release(amount)?,
                        (_, true) => account.chargeback(amount, false)?,
                    }
                }
            }
            Event::Resolve { .. } => {
                self.resolved = true;

                if let Some(amount) = self.amount {
                    match (self.disputed, self.chargebacked) {
                        (true, false) => account.release(amount)?,
                        _ => {}
                    }
                }
            }
            Event::Chargeback { .. } => {
                self.chargebacked = true;

                if let Some(amount) = self.amount {
                    match (self.disputed, self.resolved) {
                        (true, false) => account.chargeback(amount, true)?,
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
