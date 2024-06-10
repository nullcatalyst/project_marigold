use super::Amount;
use serde::{Deserialize, Deserializer};

// Make these types easily swappable.
pub type ClientId = u16;
pub type TransactionId = u32;

// Using an enum to represent types that are so similar (we could use a simple struct with an
// Option<Amount> field instead) is a bit overkill, but in the case that more event types need to be
// handled -- ones that do not have as similar a structure to the existing ones, using an enum will
// be more future-proof, requiring less refactoring.
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Deposit {
        client: ClientId,
        tx: TransactionId,
        amount: Amount,
    },
    Withdrawal {
        client: ClientId,
        tx: TransactionId,
        amount: Amount,
    },
    Dispute {
        client: ClientId,
        tx: TransactionId,
    },
    Resolve {
        client: ClientId,
        tx: TransactionId,
    },
    Chargeback {
        client: ClientId,
        tx: TransactionId,
    },
}

impl Event {
    pub fn client(&self) -> ClientId {
        match self {
            Self::Deposit { client, .. } => *client,
            Self::Withdrawal { client, .. } => *client,
            Self::Dispute { client, .. } => *client,
            Self::Resolve { client, .. } => *client,
            Self::Chargeback { client, .. } => *client,
        }
    }

    pub fn transaction(&self) -> TransactionId {
        match self {
            Self::Deposit { tx, .. } => *tx,
            Self::Withdrawal { tx, .. } => *tx,
            Self::Dispute { tx, .. } => *tx,
            Self::Resolve { tx, .. } => *tx,
            Self::Chargeback { tx, .. } => *tx,
        }
    }

    pub fn amount(&self) -> Option<Amount> {
        match self {
            Self::Deposit { amount, .. } => Some(*amount),
            Self::Withdrawal { amount, .. } => Some(*amount),
            _ => None,
        }
    }
}

// serde doesn't support deserializing tagged enums from csv, so we have to do it manually.
impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct EventData {
            r#type: String,
            client: ClientId,
            tx: TransactionId,
            amount: Option<Amount>,
        }

        let data = EventData::deserialize(deserializer)?;

        match data.r#type.as_str() {
            "deposit" => Ok(Self::Deposit {
                client: data.client,
                tx: data.tx,
                amount: data.amount.map_or_else(
                    || Err(serde::de::Error::custom("missing required field: amount")),
                    |value| Ok(value),
                )?,
            }),
            "withdrawal" => Ok(Self::Withdrawal {
                client: data.client,
                tx: data.tx,
                amount: data.amount.map_or_else(
                    || Err(serde::de::Error::custom("missing required field: amount")),
                    |value| Ok(value),
                )?,
            }),
            "dispute" => Ok(Self::Dispute {
                client: data.client,
                tx: data.tx,
            }),
            "resolve" => Ok(Self::Resolve {
                client: data.client,
                tx: data.tx,
            }),
            "chargeback" => Ok(Self::Chargeback {
                client: data.client,
                tx: data.tx,
            }),
            _ => Err(serde::de::Error::custom("invalid event type")),
        }
    }
}
