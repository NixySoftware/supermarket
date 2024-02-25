use std::rc::Rc;

use graphql_client::{GraphQLQuery, Response};

use crate::internal::auth::Auth;

#[derive(Debug)]
pub enum GraphQLClientError {
    Request(reqwest::Error),
    Json(serde_json::Value),
    Text(String),
}

pub struct GraphQLClient {
    client: reqwest::Client,
    url: &'static str,
    auth: Rc<dyn Auth>,
}

impl GraphQLClient {
    pub fn new(client: reqwest::Client, url: &'static str, auth: Rc<dyn Auth>) -> Self {
        GraphQLClient { client, url, auth }
    }

    pub async fn query<Q: GraphQLQuery>(
        self,
        variables: Q::Variables,
    ) -> Result<Response<Q::ResponseData>, GraphQLClientError> {
        let body = Q::build_query(variables);

        let request_builder = self.auth.request(self.client.post(self.url)).await?;

        let response = match request_builder.json(&body).send().await {
            Ok(response) => response,
            Err(e) => return Err(GraphQLClientError::RequestError(e)),
        };

        // TODO: this code is shared between GraphQLClient and RestClient
        if response.status().is_success() {
            // TODO: success response doesn't mean GraphQL query has no errors
            match response.json().await {
                Ok(response) => Ok(response),
                Err(e) => Err(GraphQLClientError::RequestError(e)),
            }
        } else {
            if let Some(value) = response.headers().get("content-type") {
                if value.to_str().unwrap_or("") == "application/json" {
                    return match response.json::<serde_json::Value>().await {
                        Ok(response) => Err(GraphQLClientError::JsonError(response)),
                        Err(e) => Err(GraphQLClientError::RequestError(e)),
                    };
                }
            }

            match response.text().await {
                Ok(response) => Err(GraphQLClientError::TextError(response)),
                Err(e) => Err(GraphQLClientError::RequestError(e)),
            }
        }
    }
}
