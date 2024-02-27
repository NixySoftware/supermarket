use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::prelude::*;
use chrono::TimeDelta;
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
    expires_in: i64,
}

struct AlbertHeijnAuth {
    json_client: JsonClient,
    access_token: Option<String>,
    access_token_expires_at: Option<DateTime<Local>>,
    refresh_token: Option<String>,
}

impl AlbertHeijnAuth {
    fn new(json_client: JsonClient) -> Self {
        AlbertHeijnAuth {
            json_client,
            access_token: None,
            access_token_expires_at: None,
            refresh_token: None,
        }
    }

    fn process_token(&mut self, token: Token) -> String {
        let access_token = token.access_token.clone();

        self.access_token = Some(token.access_token);
        self.access_token_expires_at = Some(Local::now() + TimeDelta::seconds(token.expires_in));
        self.refresh_token = Some(token.refresh_token);

        access_token
    }

    async fn request_anonymous_token(&mut self) -> Result<String, ClientError> {
        let token = self
            .json_client
            .post::<_, _, Token>(
                "/mobile-auth/v1/auth/token/anonymous",
                Nothing,
                HashMap::from([("clientId", OAUTH_CLIENT_ID)]),
            )
            .await?;

        Ok(self.process_token(token))
    }

    async fn request_token(&mut self, code: &str) -> Result<String, ClientError> {
        let token = self
            .json_client
            .post::<_, _, Token>(
                "/mobile-auth/v1/auth/token",
                Nothing,
                HashMap::from([("clientId", OAUTH_CLIENT_ID), ("code", &code)]),
            )
            .await?;

        Ok(self.process_token(token))
    }

    async fn refresh_token(&mut self) -> Result<String, ClientError> {
        if let Some(refresh_token) = &self.refresh_token {
            let token = self
                .json_client
                .post::<_, _, Token>(
                    "/mobile-auth/v1/auth/token/refresh",
                    Nothing,
                    HashMap::from([
                        ("clientId", OAUTH_CLIENT_ID),
                        ("refreshToken", refresh_token),
                    ]),
                )
                .await?;

            Ok(self.process_token(token))
        } else {
            Err(ClientError::TextError(String::from("No refresh token")))
        }
    }
}

#[async_trait]
impl Auth for AlbertHeijnAuth {
    async fn request(&mut self, builder: RequestBuilder) -> Result<RequestBuilder, ClientError> {
        if let Some(access_token) = &self.access_token {
            // TODO: check if access token if already expired

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
