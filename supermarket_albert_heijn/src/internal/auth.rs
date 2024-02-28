use std::collections::HashMap;

use async_trait::async_trait;
use chrono::prelude::*;
use chrono::TimeDelta;
use reqwest::RequestBuilder;
use serde::Deserialize;
use supermarket::internal::{Auth, ClientError, JsonClient, Nothing};

const OAUTH_CLIENT_ID: &str = "appie-android";

#[derive(Deserialize)]
struct Token {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

pub struct AlbertHeijnAuth {
    json_client: JsonClient,
    access_token: Option<String>,
    access_token_expires_at: Option<DateTime<Local>>,
    refresh_token: Option<String>,
}

impl AlbertHeijnAuth {
    pub fn new(json_client: JsonClient) -> Self {
        AlbertHeijnAuth {
            json_client,
            access_token: None,
            access_token_expires_at: None,
            refresh_token: None,
        }
    }

    pub fn set_refresh_token(&mut self, refresh_token: String) {
        self.refresh_token = Some(refresh_token);
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
            // TODO: check if access token is already expired

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