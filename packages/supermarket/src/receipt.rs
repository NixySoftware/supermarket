use chrono::{DateTime, Utc};

use crate::client::Identifier;

#[derive(Debug)]
pub struct ReceiptSummary {
    pub id: String,
    pub created_at: DateTime<Utc>,
    // TODO: add optional total?
}

impl Identifier for ReceiptSummary {
    fn identifier(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug)]
pub struct Receipt {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub products: Vec<ReceiptProduct>,
}

impl Identifier for Receipt {
    fn identifier(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug)]
pub struct ReceiptProduct {}
