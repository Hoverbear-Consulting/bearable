use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Currency {
    Canadian,
    American,
    Chinese,
    European,
    British,
}
