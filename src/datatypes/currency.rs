use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Currency {
    Canadian,
    American,
    Chinese,
    European,
    British,
}
