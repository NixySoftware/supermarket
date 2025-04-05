use async_trait::async_trait;

use crate::{
    ClientError,
    receipt::{Receipt, ReceiptSummary},
};

pub trait Identifier {
    fn identifier(&self) -> String;
}

#[async_trait]
pub trait Client {
    async fn receipts(&self) -> Result<Vec<ReceiptSummary>, ClientError>;

    async fn receipt(&self, receipt_id: &str) -> Result<Receipt, ClientError>;
}
