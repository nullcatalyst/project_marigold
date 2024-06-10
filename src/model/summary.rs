use super::{Amount, ClientId};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Summary {
    pub client: ClientId,
    pub available: Amount,
    pub held: Amount,
    pub total: Amount,
    pub locked: bool,
}

impl PartialOrd for Summary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.client.partial_cmp(&other.client)
    }
}

impl Ord for Summary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.client.cmp(&other.client)
    }
}
