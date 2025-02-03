use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use supermarket::internal::{
    Auth, ClientError, GraphQLClient, GraphQLClientError, JsonClient, NoAuth,
};
use supermarket::serde::Nothing;
use tokio::sync::Mutex;

use crate::internal::auth::{JumboAuth, JumboToken};
use crate::internal::product::*;
use crate::internal::profile::*;
use crate::internal::receipt::*;
use crate::internal::search::*;

const AUTH_API_URL: &str = "https://auth.jumbo.com";
const API_URL: &str = "https://mobileapi.jumbo.com";
const GRAPHQL_API_URL: &str = "https://www.jumbo.com/api/graphql";
const LOYALTY_API_URL: &str = "https://loyalty-app.jumbo.com/api";
const LOYALTY_GRAPHQL_API_URL: &str = "https://loyalty-app.jumbo.com/api/graphql";

const APP_NAME: &str = "Jumbo";
const APP_SOURCE: &str = "JUMBO-APP";
const APP_VERSION: &str = "11.1.0";

fn new_auth_api_client() -> reqwest::Client {
    let headers = HeaderMap::new();

    reqwest::Client::builder()
        .default_headers(headers)
        .gzip(true)
        .user_agent(format!("{}/{}", APP_NAME, APP_VERSION))
        .build()
        .expect("Client should build")
}

fn new_api_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert("x-source", HeaderValue::from_static(APP_SOURCE));

    reqwest::Client::builder()
        .default_headers(headers)
        .gzip(true)
        .user_agent(format!("{}/{}", APP_NAME, APP_VERSION))
        .build()
        .expect("Client should build")
}

fn new_graphql_api_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert("x-source", HeaderValue::from_static(APP_SOURCE));

    reqwest::Client::builder()
        .default_headers(headers)
        .gzip(true)
        .user_agent(format!("{}/{}", APP_NAME, APP_VERSION))
        .build()
        .expect("Client should build")
}

pub struct JumboInternalClient {
    auth: Arc<Mutex<JumboAuth>>,
    graphql_client: GraphQLClient,
    json_client: JsonClient,
    loyalty_json_client: JsonClient,
    loyalty_graphql_client: GraphQLClient,
}

impl JumboInternalClient {
    pub fn new() -> Self {
        let no_auth = Arc::new(Mutex::new(NoAuth::new()));
        let auth = Arc::new(Mutex::new(JumboAuth::new(JsonClient::new(
            new_auth_api_client(),
            AUTH_API_URL,
            Arc::new(Mutex::new(NoAuth::new())),
        ))));

        JumboInternalClient {
            auth: Arc::clone(&auth),
            graphql_client: GraphQLClient::new(
                new_graphql_api_client(),
                GRAPHQL_API_URL,
                Arc::clone(&no_auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
            json_client: JsonClient::new(
                new_api_client(),
                API_URL,
                Arc::clone(&no_auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
            loyalty_json_client: JsonClient::new(
                new_api_client(),
                LOYALTY_API_URL,
                Arc::clone(&auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
            loyalty_graphql_client: GraphQLClient::new(
                new_graphql_api_client(),
                LOYALTY_GRAPHQL_API_URL,
                Arc::clone(&auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
        }
    }

    pub async fn token(&self) -> JumboToken {
        self.auth.lock().await.token()
    }

    pub async fn set_token(&self, token: JumboToken) {
        self.auth.lock().await.set_token(token)
    }

    pub async fn auth_with_code(&self, code: &str, code_verifier: &str) -> Result<(), ClientError> {
        let mut auth = self.auth.lock().await;
        auth.request_token(code.to_string(), code_verifier.to_string())
            .await?;

        Ok(())
    }

    pub async fn auth_with_refresh_token(&self, refresh_token: &str) -> Result<(), ClientError> {
        let mut auth = self.auth.lock().await;
        auth.set_token(JumboToken {
            access_token: None,
            refresh_token: Some(refresh_token.to_string()),
        });
        auth.refresh_token().await?;

        Ok(())
    }

    pub async fn profile(
        &self,
    ) -> Result<Option<get_profile::GetProfileProfile>, GraphQLClientError> {
        let response: graphql_client::Response<get_profile::ResponseData> = self
            .loyalty_graphql_client
            .query::<GetProfile>(get_profile::Variables {})
            .await?;

        Ok(response.data.unwrap().profile)
    }

    pub async fn product_categories(&self) -> Result<Vec<ProductCategory>, ClientError> {
        let result = self
            .json_client
            .get::<_, ProductCategories>("/v17/categories", [["withThematicAisles", "true"]])
            .await?;

        Ok(result.categories.data)
    }

    pub async fn product_subcategories(
        &self,
        category_id: &str,
    ) -> Result<Vec<ProductCategory>, ClientError> {
        let result = self
            .json_client
            .get::<_, ProductCategories>("/v17/categories", [["id", category_id]])
            .await?;

        Ok(result.categories.data)
    }

    pub async fn search_suggestions(
        &self,
        query: &str,
    ) -> Result<
        Vec<get_search_suggestions::GetSearchSuggestionsSearchSuggestionsKeywords>,
        GraphQLClientError,
    > {
        let response = self
            .graphql_client
            .query::<GetSearchSuggestions>(get_search_suggestions::Variables {
                input: Some(get_search_suggestions::SearchSuggestionsInput {
                    search_terms: query.to_string(),
                }),
            })
            .await?;

        Ok(response.data.unwrap().search_suggestions.keywords)
    }

    // TODO: return a paginator that can make requests instead of directly performing a single one

    pub async fn search_products<Q: Serialize>(
        &self,
        query: Q,
    ) -> Result<ProductSearch, ClientError> {
        self.json_client
            .get::<Q, ProductSearch>("/v17/search", query)
            .await
    }

    pub async fn receipts(&self) -> Result<Vec<ReceiptSummary>, ClientError> {
        self.loyalty_json_client
            .get::<_, Vec<ReceiptSummary>>("/receipt/customer/overviews", Nothing)
            .await
    }

    pub async fn receipt(&self, receipt_id: &str) -> Result<Receipt, ClientError> {
        self.loyalty_json_client
            .get::<_, Receipt>(&format!("/receipt/{}", receipt_id), Nothing)
            .await
    }
}

impl Default for JumboInternalClient {
    fn default() -> Self {
        Self::new()
    }
}
