use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum BillingMethod {
    /// ACH
    AutomatedClearingHouse,
    /// Interac E-Transfer
    Interac,
    /// From Transferwise
    Transferwise,
    /// The cold, solid stuff
    Cash,
}
