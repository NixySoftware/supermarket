use async_trait::async_trait;
use supermarket::{
    receipt::{Receipt, ReceiptSummary},
    Client, ClientError, Identifier,
};

use crate::internal::{JumboInternalClient, JumboToken};

pub struct JumboClient {
    pub internal: JumboInternalClient,
}

impl JumboClient {
    pub fn new() -> Self {
        JumboClient {
            internal: JumboInternalClient::new(),
        }
    }

    pub async fn auth_with_code(&self, code: &str, code_verifier: &str) -> Result<(), ClientError> {
        self.internal.auth_with_code(code, code_verifier).await
    }

    pub async fn auth_with_refresh_token(&self, refresh_token: &str) -> Result<(), ClientError> {
        self.internal.auth_with_refresh_token(refresh_token).await
    }

    pub async fn token(&self) -> JumboToken {
        self.internal.token().await
    }

    pub async fn set_token(&self, token: JumboToken) {
        self.internal.set_token(token).await
    }
}

#[async_trait]
impl Client for JumboClient {
    async fn receipts(&self) -> Result<Vec<ReceiptSummary>, ClientError> {
        Ok(self
            .internal
            .receipts()
            .await?
            .iter()
            .map(|r| ReceiptSummary {
                id: r.identifier(),
                created_at: r.purchase_end_on,
            })
            .collect())
    }

    async fn receipt(&self, receipt_id: &str) -> Result<Receipt, ClientError> {
        self.internal.receipt(receipt_id).await.map(|r| Receipt {
            id: r.identifier(),
            created_at: r.purchase_end_on,
            products: vec![],
        })
    }
}

impl Default for JumboClient {
    fn default() -> Self {
        JumboClient::new()
    }
}
