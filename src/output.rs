use crate::{Amount, ClientId};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Output {
    pub client: ClientId,
    pub available: Amount,
    pub held: Amount,
    pub total: Amount,
    pub locked: bool,
}
