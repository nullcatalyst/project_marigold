use crate::{Amount, ClientId};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Output {
    pub client: ClientId,
    pub available: Amount,
    pub held: Amount,
    pub total: Amount,
    pub locked: bool,
}

impl PartialOrd for Output {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.client.partial_cmp(&other.client)
    }
}

impl Ord for Output {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.client.cmp(&other.client)
    }
}
