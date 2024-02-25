use std::rc::Rc;

use graphql_client::{GraphQLQuery, Response};

use crate::internal::auth::Auth;

#[derive(Debug)]
pub enum GraphQLError {
    Request(reqwest::Error),
    Json(serde_json::Value),
    Text(String),
}

pub struct GraphQLClient {
    client: Rc<reqwest::Client>,
    url: &'static str,
    auth: Rc<dyn Auth>,
}

impl GraphQLClient {
    pub fn new(client: Rc<reqwest::Client>, url: &'static str, auth: Rc<dyn Auth>) -> Self {
        GraphQLClient { client, url, auth }
    }

    pub async fn query<Q: GraphQLQuery>(
        self,
        variables: Q::Variables,
    ) -> Result<Response<Q::ResponseData>, GraphQLError> {
        let body = Q::build_query(variables);

        let request_builder = self.auth.request(self.client.post(self.url));

        let response = match request_builder.json(&body).send().await {
            Ok(response) => response,
            Err(e) => return Err(GraphQLError::Request(e)),
        };

        if response.status().is_success() {
            // TODO: success response doesn't mean GraphQL query has no errors
            match response.json().await {
                Ok(response) => Ok(response),
                Err(e) => Err(GraphQLError::Request(e)),
            }
        } else {
            if let Some(value) = response.headers().get("content-type") {
                if value.to_str().unwrap_or("") == "application/json" {
                    return match response.json::<serde_json::Value>().await {
                        Ok(response) => Err(GraphQLError::Json(response)),
                        Err(e) => Err(GraphQLError::Request(e)),
                    };
                }
            }

            match response.text().await {
                Ok(response) => Err(GraphQLError::Text(response)),
                Err(e) => Err(GraphQLError::Request(e)),
            }
        }
    }
}
