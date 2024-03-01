use std::sync::Arc;

use supermarket::internal::{Auth, GraphQLClient, GraphQLClientError, JsonClient, NoAuth};
use supermarket::internal::{ClientError, Nothing};
use tokio::sync::Mutex;

use crate::internal::auth::{AlbertHeijnAuth, AlbertHeijnToken};
use crate::internal::member::*;
use crate::internal::receipt::*;

const API_URL: &str = "https://api.ah.nl";
const GRAPHQL_API_URL: &str = "https://api.ah.nl/graphql";

pub struct AlbertHeijnInternalClient {
    auth: Arc<Mutex<AlbertHeijnAuth>>,
    graphql_client: GraphQLClient,
    json_client: JsonClient,
}

impl Default for AlbertHeijnInternalClient {
    fn default() -> Self {
        Self::new()
    }
}

impl AlbertHeijnInternalClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Appie/8.60.1")
            .build()
            .unwrap();

        let auth = Arc::new(Mutex::new(AlbertHeijnAuth::new(JsonClient::new(
            client.clone(),
            API_URL,
            Arc::new(Mutex::new(NoAuth::new())),
        ))));

        AlbertHeijnInternalClient {
            auth: Arc::clone(&auth),
            graphql_client: GraphQLClient::new(
                client.clone(),
                GRAPHQL_API_URL,
                Arc::clone(&auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
            json_client: JsonClient::new(
                client.clone(),
                API_URL,
                Arc::clone(&auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
        }
    }

    pub async fn token(&self) -> AlbertHeijnToken {
        self.auth.lock().await.token()
    }

    pub async fn set_token(&self, token: AlbertHeijnToken) {
        self.auth.lock().await.set_token(token)
    }

    pub async fn auth_with_code(&self, code: &str) -> Result<(), ClientError> {
        let mut auth = self.auth.lock().await;
        auth.request_token(String::from(code)).await?;

        Ok(())
    }

    pub async fn auth_with_refresh_token(&self, refresh_token: &str) -> Result<(), ClientError> {
        let mut auth = self.auth.lock().await;
        auth.set_refresh_token(String::from(refresh_token));
        auth.refresh_token().await?;

        Ok(())
    }

    pub async fn member(&self) -> Result<Option<get_member::GetMemberMember>, GraphQLClientError> {
        let response = self
            .graphql_client
            .query::<GetMember>(get_member::Variables {})
            .await?;

        Ok(response.data.unwrap().member)
    }

    pub async fn receipts(&self) -> Result<Vec<ReceiptSummary>, ClientError> {
        self.json_client
            .get::<_, Vec<ReceiptSummary>>("/mobile-services/v1/receipts", Nothing)
            .await
    }

    pub async fn receipt(&self, receipt_id: &str) -> Result<Receipt, ClientError> {
        let mut receipt = self
            .json_client
            .get::<_, Receipt>(
                &format!("/mobile-services/v2/receipts/{}", receipt_id),
                Nothing,
            )
            .await?;

        receipt.transaction_id = receipt_id.to_string();

        Ok(receipt)
    }
}
