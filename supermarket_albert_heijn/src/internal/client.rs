use std::rc::Rc;

use reqwest::RequestBuilder;
use serde::Deserialize;
use supermarket::internal::{
    Auth, GraphQLClient, GraphQLClientError, JsonClient, JsonClientError, NoAuth, Nothing,
};

use crate::internal::member::get_member;
use crate::internal::member::GetMember;

const API_URL: &str = "https://api.ah.nl";
const GRAPHQL_API_URL: &str = "https://api.ah.nl/graphql";
const OAUTH_CLIENT_ID: &str = "appie-android";

pub struct AlbertHeijnInternalClient {
    auth: Rc<AlbertHeijnAuth>,
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
            json_client: json_client,
            access_token: None,
            // TODO: store when access token expires
            refresh_token: None,
        }
    }

    async fn request_anonymous_token(self) -> Result<Token, JsonClientError> {
        self.json_client
            .post(
                "/mobile-auth/v1/auth/token/anonymous",
                Nothing,
                [("clientId", OAUTH_CLIENT_ID)],
            )
            .await
    }

    async fn request_token(self, code: &str) -> Result<Token, JsonClientError> {
        self.json_client
            .post::<_, _, Token>(
                "/mobile-auth/v1/auth/token",
                Nothing,
                [("clientId", OAUTH_CLIENT_ID), ("code", &code)],
            )
            .await
    }

    async fn refresh_token(&mut self) -> Result<(), JsonClientError> {
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

            self.access_token = Some(token.access_token);
            self.refresh_token = Some(token.refresh_token);
            // TODO: convert expires_in to chrono?

            Ok(())
        } else {
            Err(JsonClientError::TextError(String::from("No refresh token")))
        }
    }
}

impl Auth for AlbertHeijnAuth {
    async fn request(&mut self, builder: RequestBuilder) -> RequestBuilder {
        if let Some(access_token) = &self.access_token {
            builder.bearer_auth(access_token)
        } else if let Some(_) = &self.refresh_token {
            self.refresh_token().await;

            builder.bearer_auth(access_token)
        } else {
            self.request_anonymous_token().await;

            builder
        }
    }
}

impl AlbertHeijnInternalClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Appie/8.60.1")
            .build()
            .unwrap();

        let auth = Rc::new(AlbertHeijnAuth::new(JsonClient::new(
            client.clone(),
            API_URL,
            Rc::new(NoAuth::new()),
        )));

        AlbertHeijnInternalClient {
            auth: Rc::clone(&auth),
            graphql_client: GraphQLClient::new(
                client.clone(),
                GRAPHQL_API_URL,
                Rc::clone(&auth) as Rc<dyn Auth>,
            ),
            json_client: JsonClient::new(client.clone(), API_URL, Rc::clone(&auth) as Rc<dyn Auth>),
        }
    }

    pub async fn member(self) -> Result<Option<get_member::GetMemberMember>, GraphQLClientError> {
        let response = self
            .graphql_client
            .query::<GetMember>(get_member::Variables {})
            .await?;

        // TODO: actually do something with errors/nulls
        Ok(response.data.unwrap().member)
    }
}
