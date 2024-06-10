use super::{Amount, AmountOpError};

#[derive(Default, Debug, Clone, Copy)]
pub struct Account {
    total: Amount,
    held: Amount,
    locked: bool,
}

impl Account {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total(&self) -> Amount {
        self.total
    }

    pub fn held(&self) -> Amount {
        self.held
    }

    pub fn available(&self) -> Amount {
        // This should never fail, since `total` should always be greater than or equal to `held`,
        // and `held` should never be negative.
        (self.total - self.held).unwrap_or_default()
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn deposit(&mut self, amount: Amount) -> Result<(), AmountOpError> {
        match self.total + amount {
            Ok(value) => {
                self.total = value;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn withdraw(&mut self, amount: Amount) -> Result<(), AmountOpError> {
        match self.total - amount {
            Ok(value) => {
                self.total = value;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn hold(&mut self, amount: Amount) -> Result<(), AmountOpError> {
        match self.held + amount {
            Ok(value) => {
                self.held = value;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn release(&mut self, amount: Amount) -> Result<(), AmountOpError> {
        match self.held - amount {
            Ok(value) => {
                self.held = value;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn chargeback(&mut self, amount: Amount, held: bool) -> Result<(), AmountOpError> {
        self.lock();

        match (
            self.total - amount,
            if held {
                self.held - amount
            } else {
                Ok(self.held)
            },
        ) {
            (Ok(new_total), Ok(new_held)) => {
                self.total = new_total;
                self.held = new_held;
                Ok(())
            }
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}
