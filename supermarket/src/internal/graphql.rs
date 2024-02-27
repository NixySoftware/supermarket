use std::sync::Arc;

use graphql_client::{GraphQLQuery, Response};
use tokio::sync::Mutex;

use crate::internal::auth::Auth;
use crate::internal::error::ClientError;

pub struct GraphQLClient {
    client: reqwest::Client,
    url: &'static str,
    auth: Arc<Mutex<dyn Auth + Send>>,
}

impl GraphQLClient {
    pub fn new(
        client: reqwest::Client,
        url: &'static str,
        auth: Arc<Mutex<dyn Auth + Send>>,
    ) -> Self {
        GraphQLClient { client, url, auth }
    }

    pub async fn query<Q: GraphQLQuery>(
        self,
        variables: Q::Variables,
    ) -> Result<Response<Q::ResponseData>, ClientError> {
        let body = Q::build_query(variables);

        let request_builder = self
            .auth
            .lock()
            .await
            .request(self.client.post(self.url))
            .await?;

        let response = match request_builder.json(&body).send().await {
            Ok(response) => response,
            Err(e) => return Err(ClientError::RequestError(e)),
        };

        // TODO: this code is shared between GraphQLClient and RestClient
        if response.status().is_success() {
            // TODO: success response doesn't mean GraphQL query has no errors
            match response.json().await {
                Ok(response) => Ok(response),
                Err(e) => Err(ClientError::RequestError(e)),
            }
        } else {
            if let Some(value) = response.headers().get("content-type") {
                if value.to_str().unwrap_or("") == "application/json" {
                    return match response.json::<serde_json::Value>().await {
                        Ok(response) => Err(ClientError::JsonError(response)),
                        Err(e) => Err(ClientError::RequestError(e)),
                    };
                }
            }

            match response.text().await {
                Ok(response) => Err(ClientError::TextError(response)),
                Err(e) => Err(ClientError::RequestError(e)),
            }
        }
    }
}
