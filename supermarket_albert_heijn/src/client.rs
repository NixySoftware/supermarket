use async_trait::async_trait;
use supermarket::{
    receipt::{Receipt, ReceiptSummary},
    Client, ClientError, Identifier,
};

use crate::internal::{AlbertHeijnInternalClient, AlbertHeijnToken};

pub struct AlbertHeijnClient {
    pub internal: AlbertHeijnInternalClient,
}

impl AlbertHeijnClient {
    pub fn new() -> Self {
        AlbertHeijnClient {
            internal: AlbertHeijnInternalClient::new(),
        }
    }

    pub async fn auth_with_code(&self, code: &str) -> Result<(), ClientError> {
        self.internal.auth_with_code(code).await
    }

    pub async fn auth_with_refresh_token(&self, refresh_token: &str) -> Result<(), ClientError> {
        self.internal.auth_with_refresh_token(refresh_token).await
    }

    pub async fn token(&self) -> AlbertHeijnToken {
        self.internal.token().await
    }

    pub async fn set_token(&self, token: AlbertHeijnToken) {
        self.internal.set_token(token).await
    }
}

#[async_trait]
impl Client for AlbertHeijnClient {
    async fn receipts(&self) -> Result<Vec<ReceiptSummary>, ClientError> {
        Ok(self
            .internal
            .receipts()
            .await?
            .iter()
            .map(|r| ReceiptSummary {
                id: r.identifier(),
                created_at: r.transaction_moment,
            })
            .collect())
    }

    async fn receipt(&self, receipt_id: &str) -> Result<Receipt, ClientError> {
        self.internal.receipt(receipt_id).await.map(|r| Receipt {
            id: r.identifier(),
            created_at: r.transaction_moment,
            products: vec![],
        })
    }
}

impl Default for AlbertHeijnClient {
    fn default() -> Self {
        Self::new()
    }
}
