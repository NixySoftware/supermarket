use std::rc::Rc;

use reqwest::RequestBuilder;
use supermarket::internal::{Auth, GraphQLClient, GraphQLError};

use crate::internal::member::get_member;
use crate::internal::member::GetMember;

const AUTH_URL: &str = "https://login.ah.nl";
const API_URL: &str = "https://api.ah.nl";
const GRAPHQL_API_URL: &str = "https://api.ah.nl/graphql";

pub struct AlbertHeijnInternalClient {
    client: Rc<reqwest::Client>,
    auth: Rc<AlbertHeijnAuth>,
    graphql_client: GraphQLClient,
}

impl Default for AlbertHeijnInternalClient {
    fn default() -> Self {
        Self::new()
    }
}

struct AlbertHeijnAuth {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl AlbertHeijnAuth {
    fn new() -> Self {
        AlbertHeijnAuth {
            access_token: None,
            refresh_token: None,
        }
    }
}

impl Auth for AlbertHeijnAuth {
    fn request(&self, builder: RequestBuilder) -> RequestBuilder {
        // TODO: refresh is access_token is None
        // TODO: request_token should be required for the internal client?

        if let Some(access_token) = self.access_token.clone() {
            builder.bearer_auth(access_token)
        } else {
            builder
        }
    }
}

impl AlbertHeijnInternalClient {
    pub fn new() -> Self {
        // TODO: keeping a reference to client and auth is probably unnecessary, so that could simplify the Rc stuff

        let client = Rc::new(
            reqwest::Client::builder()
                .user_agent("Appie/8.60.1")
                .build()
                .unwrap(),
        );
        let auth = Rc::new(AlbertHeijnAuth::new());

        AlbertHeijnInternalClient {
            client: Rc::clone(&client),
            auth: Rc::clone(&auth),
            graphql_client: GraphQLClient::new(
                Rc::clone(&client),
                GRAPHQL_API_URL,
                Rc::clone(&auth) as Rc<dyn Auth>,
            ),
        }
    }

    pub async fn member(self) -> Result<Option<get_member::GetMemberMember>, GraphQLError> {
        let response = self
            .graphql_client
            .query::<GetMember>(get_member::Variables {})
            .await?;

        // TODO: actually do something with errors/nulls
        Ok(response.data.unwrap().member)
    }
}
