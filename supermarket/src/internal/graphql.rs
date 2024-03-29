use std::sync::Arc;

use graphql_client::{GraphQLQuery, Response};
use tokio::sync::Mutex;

use crate::internal::auth::Auth;
use crate::internal::client::ClientError;
use crate::internal::json::JsonClient;
use crate::serde::Nothing;

#[derive(Debug)]
pub enum GraphQLClientError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Value),
    TextError(String),
    GraphQLError(Vec<graphql_client::Error>),
}

impl From<ClientError> for GraphQLClientError {
    fn from(error: ClientError) -> Self {
        match error {
            ClientError::RequestError(e) => GraphQLClientError::RequestError(e),
            ClientError::JsonError(e) => GraphQLClientError::JsonError(e),
            ClientError::TextError(e) => GraphQLClientError::TextError(e),
        }
    }
}

pub struct GraphQLClient {
    json_client: JsonClient,
}

impl GraphQLClient {
    pub fn new(
        client: reqwest::Client,
        url: &'static str,
        auth: Arc<Mutex<dyn Auth + Send>>,
    ) -> Self {
        GraphQLClient {
            json_client: JsonClient::new(client, url, auth),
        }
    }

    pub async fn query<Q: GraphQLQuery>(
        &self,
        variables: Q::Variables,
    ) -> Result<Response<Q::ResponseData>, GraphQLClientError> {
        let body = Q::build_query(variables);

        let response = self
            .json_client
            .post::<_, _, Response<Q::ResponseData>>("", Nothing, body)
            .await?;

        match response.errors {
            Some(errors) if !errors.is_empty() => Err(GraphQLClientError::GraphQLError(errors)),
            _ => Ok(response),
        }
    }
}
