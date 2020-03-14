use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::datatypes::{LineItem, Client};

#[derive(Debug, Deserialize, Serialize)]
pub struct Invoice {
    date_issued: DateTime<Utc>,
    customer: Client,
    lines: Vec<LineItem>,
}
