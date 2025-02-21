use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq, Serialize, Deserialize)]
pub struct W3Transaction {
    pub w3_height: String,
    pub w3_address: String,
}

pub enum Table {
    W3Transaction,
}

impl Table {
    pub fn map_table(&self) -> String {
        match self {
            Table::W3Transaction => "w3_transaction".to_string(),
        }
    }
}
