use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Local, TimeDelta};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use supermarket::internal::{Auth, ClientError, JsonClient};
use supermarket::serde::Nothing;

const OAUTH_CLIENT_ID: &str = "ZVa0cW0LadbDHINgrBLuEAp5amVBKQh1";
const OAUTH_REDIRECT_URI: &str = "https://loyalty-app.jumbo.com/home";

#[derive(Deserialize, Debug)]
struct Token {
    access_token: String,
    expires_in: i64,
    // id_token: String,
    refresh_token: String,
    // scope: String,
    // token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JumboToken {
    pub access_token: Option<(String, DateTime<Local>)>,
    pub refresh_token: Option<String>,
}

pub struct JumboAuth {
    json_client: JsonClient,
    access_token: Option<(String, DateTime<Local>)>,
    refresh_token: Option<String>,
}

impl JumboAuth {
    pub fn new(json_client: JsonClient) -> Self {
        JumboAuth {
            json_client,
            access_token: None,
            refresh_token: None,
        }
    }

    pub fn token(&self) -> JumboToken {
        JumboToken {
            access_token: self.access_token.clone(),
            refresh_token: self.refresh_token.clone(),
        }
    }

    pub fn set_token(&mut self, token: JumboToken) {
        self.access_token = token.access_token;
        self.refresh_token = token.refresh_token;
    }

    fn process_token(&mut self, token: Token) -> String {
        let access_token = token.access_token.clone();

        self.access_token = Some((
            token.access_token,
            Local::now() + TimeDelta::seconds(token.expires_in),
        ));
        self.refresh_token = Some(token.refresh_token);

        println!("{:#?} | {:#?}", self.access_token, self.refresh_token);

        access_token
    }

    pub async fn request_token(
        &mut self,
        code: String,
        code_verifier: String,
    ) -> Result<String, ClientError> {
        let token = self
            .json_client
            .post::<_, _, Token>(
                "/oauth/token",
                Nothing,
                HashMap::from([
                    ("client_id", OAUTH_CLIENT_ID),
                    ("code", &code),
                    ("code_verifier", &code_verifier),
                    ("grant_type", "authorization_code"),
                    ("redirect_uri", OAUTH_REDIRECT_URI),
                ]),
            )
            .await?;

        Ok(self.process_token(token))
    }

    pub async fn refresh_token(&mut self) -> Result<String, ClientError> {
        if let Some(refresh_token) = &self.refresh_token {
            let token = self
                .json_client
                .post::<_, _, Token>(
                    "/oauth/token",
                    Nothing,
                    HashMap::from([
                        ("client_id", OAUTH_CLIENT_ID),
                        ("grant_type", "refresh_token"),
                        ("refresh_token", refresh_token),
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
impl Auth for JumboAuth {
    async fn request(&mut self, builder: RequestBuilder) -> Result<RequestBuilder, ClientError> {
        if let Some((access_token, expires_at)) = &self.access_token {
            if *expires_at > Local::now() {
                Ok(builder.bearer_auth(access_token))
            } else {
                let access_token = self.refresh_token().await?;

                Ok(builder.bearer_auth(access_token))
            }
        } else if self.refresh_token.is_some() {
            let access_token = self.refresh_token().await?;

            Ok(builder.bearer_auth(access_token))
        } else {
            Ok(builder)
        }
    }
}
