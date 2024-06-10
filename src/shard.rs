use crate::{Account, AmountOpError, ClientId, Event, Output, Transaction, TransactionId};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Display, Formatter, Result as FmtResult},
    rc::Rc,
};

// In the future, ideally it would be nice to be able to have multiple shards, and to be able to
// merge or reconcile them. But that seems out of scope of this project.
#[derive(Default, Debug, Clone)]
pub struct Shard {
    accounts: HashMap<ClientId, Account>,
    transactions: HashMap<TransactionId, Transaction>,
    errors: Vec<ShardError>,
}

impl Shard {
    pub fn new() -> Self {
        Self::default()
    }

    // @returns a snapshot of the account, in its current state, if it exists
    pub fn get_account(&self, client: ClientId) -> Option<Account> {
        self.accounts.get(&client).copied()
    }

    // @returns a snapshot of the transaction, in its current state, if it exists
    pub fn get_transaction(&self, tx: TransactionId) -> Option<Transaction> {
        self.transactions.get(&tx).copied()
    }

    pub fn errors(&self) -> &[ShardError] {
        &self.errors
    }

    pub fn push_event(&mut self, event: Event) {
        let client_id = event.client();
        let tx_id = event.transaction();

        let account = self.accounts.entry(client_id).or_default();
        let transaction = self.transactions.entry(tx_id).or_default();

        if let Err(err) = transaction.apply(event, account) {
            self.push_error(ShardError::TransactionOprror {
                tx: tx_id,
                reason: err,
            });
        }
    }

    pub fn push_error(&mut self, err: ShardError) {
        self.errors.push(err);
    }

    pub fn generate_output(&self) -> Vec<Output> {
        self.accounts
            .iter()
            .map(|(client, account)| Output {
                client: *client,
                available: account.available(),
                held: account.held(),
                total: account.total(),
                locked: account.is_locked(),
            })
            .collect()
    }

    pub fn generate_output_sorted(&self) -> Vec<Output> {
        self.accounts
            .iter()
            .map(|(client, account)| Output {
                client: *client,
                available: account.available(),
                held: account.held(),
                total: account.total(),
                locked: account.is_locked(),
            })
            .collect::<BinaryHeap<Output>>()
            .into_sorted_vec()
    }

    pub fn reconcile(&mut self, _other: &Self) {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub enum ShardError {
    // The csv::Error type is not Clone, so we wrap it in an Rc to make it Clone.
    CsvParseError(Rc<csv::Error>),

    TransactionOprror {
        tx: TransactionId,
        reason: AmountOpError,
    },
}

impl Display for ShardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::CsvParseError(e) => write!(f, "CSV parse error: {}", e.as_ref()),
            Self::TransactionOprror { tx, reason } => {
                if let Some(rhs) = reason.rhs {
                    write!(
                        f,
                        "Transaction {} failed due to arithmetic overflow: {} {} {}",
                        tx, reason.lhs, reason.op, rhs
                    )
                } else {
                    write!(
                        f,
                        "Transaction {} failed due to arithmetic overflow: {}{}",
                        tx, reason.op, reason.lhs
                    )
                }
            }
        }
    }
}
