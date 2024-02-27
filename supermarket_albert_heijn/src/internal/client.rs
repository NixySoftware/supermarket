use std::sync::Arc;

use async_trait::async_trait;
use reqwest::RequestBuilder;
use serde::Deserialize;
use supermarket::internal::{Auth, ClientError, GraphQLClient, JsonClient, NoAuth, Nothing};
use tokio::sync::Mutex;

use crate::internal::member::get_member;
use crate::internal::member::GetMember;

const API_URL: &str = "https://api.ah.nl";
const GRAPHQL_API_URL: &str = "https://api.ah.nl/graphql";
const OAUTH_CLIENT_ID: &str = "appie-android";

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

#[derive(Deserialize)]
struct Token {
    access_token: String,
    refresh_token: String,
    expires_in: isize,
}

struct AlbertHeijnAuth {
    json_client: JsonClient,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl AlbertHeijnAuth {
    fn new(json_client: JsonClient) -> Self {
        AlbertHeijnAuth {
            json_client,
            access_token: None,
            // TODO: store when access token expires
            refresh_token: None,
        }
    }

    async fn request_anonymous_token(&mut self) -> Result<String, ClientError> {
        let token = self
            .json_client
            .post::<_, _, Token>(
                "/mobile-auth/v1/auth/token/anonymous",
                Nothing,
                [("clientId", OAUTH_CLIENT_ID)],
            )
            .await?;

        let access_token = token.access_token.clone();

        self.access_token = Some(token.access_token);
        self.refresh_token = Some(token.refresh_token);
        // TODO: convert expires_in to chrono?

        Ok(access_token)
    }

    async fn request_token(&mut self, code: &str) -> Result<String, ClientError> {
        let token = self
            .json_client
            .post::<_, _, Token>(
                "/mobile-auth/v1/auth/token",
                Nothing,
                [("clientId", OAUTH_CLIENT_ID), ("code", &code)],
            )
            .await?;

        let access_token = token.access_token.clone();

        self.access_token = Some(token.access_token);
        self.refresh_token = Some(token.refresh_token);
        // TODO: convert expires_in to chrono?

        Ok(access_token)
    }

    async fn refresh_token(&mut self) -> Result<String, ClientError> {
        if let Some(refresh_token) = &self.refresh_token {
            let token = self
                .json_client
                .post::<_, _, Token>(
                    "/mobile-auth/v1/auth/token/refresh",
                    Nothing,
                    [
                        ("clientId", OAUTH_CLIENT_ID),
                        ("refreshToken", refresh_token),
                    ],
                )
                .await?;

            let access_token = token.access_token.clone();

            self.access_token = Some(token.access_token);
            self.refresh_token = Some(token.refresh_token);
            // TODO: convert expires_in to chrono?

            Ok(access_token)
        } else {
            Err(ClientError::TextError(String::from("No refresh token")))
        }
    }
}

#[async_trait]
impl Auth for AlbertHeijnAuth {
    async fn request(&mut self, builder: RequestBuilder) -> Result<RequestBuilder, ClientError> {
        if let Some(access_token) = &self.access_token {
            Ok(builder.bearer_auth(access_token))
        } else if self.refresh_token.is_some() {
            let access_token = self.refresh_token().await?;

            Ok(builder.bearer_auth(access_token))
        } else {
            let access_token = self.request_anonymous_token().await?;

            Ok(builder.bearer_auth(access_token))
        }
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

    pub async fn member(self) -> Result<Option<get_member::GetMemberMember>, ClientError> {
        let response = self
            .graphql_client
            .query::<GetMember>(get_member::Variables {})
            .await?;

        // TODO: actually do something with errors/nulls
        Ok(response.data.unwrap().member)
    }
}
