use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use supermarket::internal::{
    Auth, ClientError, GraphQLClient, GraphQLClientError, JsonClient, NoAuth,
};
use supermarket::serde::Nothing;
use tokio::sync::Mutex;

use crate::internal::auth::{AlbertHeijnAuth, AlbertHeijnToken};
use crate::internal::member::*;
use crate::internal::product::*;
use crate::internal::receipt::*;

const API_URL: &str = "https://api.ah.nl";
const GRAPHQL_API_URL: &str = "https://api.ah.nl/graphql";

const APP_NAME: &str = "Appie";
const APP_NAME_GRAPHQL: &str = "appie-android";
const APP_VERSION: &str = "8.60.1";
const APP_LOCALE: &str = "nl_NL";

fn new_api_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert("x-application", HeaderValue::from_static("AHWEBSHOP"));
    headers.insert("x-clientname", HeaderValue::from_static(APP_NAME));
    headers.insert("x-clientversion", HeaderValue::from_static(APP_VERSION));

    reqwest::Client::builder()
        .default_headers(headers)
        .gzip(true)
        .user_agent(format!("{}/{}", APP_NAME, APP_VERSION))
        .build()
        .unwrap()
}

fn new_graphql_api_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert("x-client-name", HeaderValue::from_static(APP_NAME_GRAPHQL));
    headers.insert("x-client-version", HeaderValue::from_static(APP_VERSION));
    headers.insert("x-locale", HeaderValue::from_static(APP_LOCALE));

    reqwest::Client::builder()
        .default_headers(headers)
        .gzip(true)
        .user_agent(format!("{}/{}", APP_NAME, APP_VERSION))
        .build()
        .unwrap()
}

pub struct AlbertHeijnInternalClient {
    auth: Arc<Mutex<AlbertHeijnAuth>>,
    graphql_client: GraphQLClient,
    json_client: JsonClient,
}

impl AlbertHeijnInternalClient {
    pub fn new() -> Self {
        let auth = Arc::new(Mutex::new(AlbertHeijnAuth::new(JsonClient::new(
            new_api_client(),
            API_URL,
            Arc::new(Mutex::new(NoAuth::new())),
        ))));

        AlbertHeijnInternalClient {
            auth: Arc::clone(&auth),
            graphql_client: GraphQLClient::new(
                new_graphql_api_client(),
                GRAPHQL_API_URL,
                Arc::clone(&auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
            json_client: JsonClient::new(
                new_api_client(),
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

    pub async fn product_categories(&self) -> Result<Vec<ProductCategory>, ClientError> {
        self.json_client
            .get::<_, Vec<ProductCategory>>(
                "/mobile-services/v1/product-shelves/categories",
                Nothing,
            )
            .await
    }

    pub async fn product_subcategories(
        &self,
        category_id: &str,
    ) -> Result<Vec<ProductCategory>, ClientError> {
        let result = self
            .json_client
            .get::<_, ProductSubcategories>(
                &format!(
                    "/mobile-services/v1/product-shelves/categories/{}/sub-categories",
                    category_id
                ),
                Nothing,
            )
            .await?;

        Ok(result.children)
    }

    pub async fn search_products<Q: Serialize>(
        &self,
        query: Q,
    ) -> Result<ProductSearch, ClientError> {
        self.json_client
            .get::<Q, ProductSearch>("/mobile-services/product/search/v2", query)
            .await
    }

    pub async fn search_products_by_category(
        &self,
        category_id: &str,
    ) -> Result<ProductSearch, ClientError> {
        self.search_products([["taxonomyId", category_id]]).await
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

impl Default for AlbertHeijnInternalClient {
    fn default() -> Self {
        Self::new()
    }
}
