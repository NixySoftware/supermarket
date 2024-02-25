use std::rc::Rc;

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::internal::auth::Auth;
use crate::internal::util::Nothing;

#[derive(Debug)]
pub enum JsonClientError {
    Request(reqwest::Error),
    Json(serde_json::Value),
    Text(String),
}

pub struct JsonClient {
    client: reqwest::Client,
    url: &'static str,
    auth: Rc<dyn Auth>,
}

impl JsonClient {
    pub fn new(client: reqwest::Client, url: &'static str, auth: Rc<dyn Auth>) -> Self {
        JsonClient { client, url, auth }
    }

    pub async fn request<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        self,
        method: Method,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, JsonClientError> {
        let request_builder = self.client.request(method, format!("{}{}", self.url, url));

        let request_builder = self
            .auth
            .request(request_builder)
            .await
            .query(&query)
            .json(&body);

        let response = match request_builder.send().await {
            Ok(response) => response,
            Err(e) => return Err(JsonClientError::RequestError(e)),
        };

        // TODO: this code is shared between GraphQLClient and RestClient
        if response.status().is_success() {
            match response.json::<R>().await {
                Ok(response) => Ok(response),
                Err(e) => Err(JsonClientError::RequestError(e)),
            }
        } else {
            if let Some(value) = response.headers().get("content-type") {
                if value.to_str().unwrap_or("") == "application/json" {
                    return match response.json::<serde_json::Value>().await {
                        Ok(response) => Err(JsonClientError::JsonError(response)),
                        Err(e) => Err(JsonClientError::RequestError(e)),
                    };
                }
            }

            match response.text().await {
                Ok(response) => Err(JsonClientError::TextError(response)),
                Err(e) => Err(JsonClientError::RequestError(e)),
            }
        }
    }

    pub async fn delete<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        self,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, JsonClientError> {
        self.request(Method::DELETE, url, query, body).await
    }

    pub async fn get<Q: Serialize, R: DeserializeOwned>(
        self,
        url: &str,
        query: Q,
    ) -> Result<R, JsonClientError> {
        self.request(Method::GET, url, query, Nothing).await
    }

    pub async fn head<Q: Serialize, R: DeserializeOwned>(
        self,
        url: &str,
        query: Q,
    ) -> Result<R, JsonClientError> {
        self.request(Method::HEAD, url, query, Nothing).await
    }

    pub async fn patch<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        self,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, JsonClientError> {
        self.request(Method::PATCH, url, query, body).await
    }

    pub async fn post<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        self,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, JsonClientError> {
        self.request(Method::POST, url, query, body).await
    }

    pub async fn put<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        self,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, JsonClientError> {
        self.request(Method::PUT, url, query, body).await
    }
}
