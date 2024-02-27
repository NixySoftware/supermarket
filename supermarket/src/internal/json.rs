use std::sync::Arc;

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

use crate::internal::auth::Auth;
use crate::internal::error::ClientError;
use crate::internal::util::Nothing;

pub struct JsonClient {
    client: reqwest::Client,
    url: &'static str,
    auth: Arc<Mutex<dyn Auth + Send>>,
}

impl JsonClient {
    pub fn new(
        client: reqwest::Client,
        url: &'static str,
        auth: Arc<Mutex<dyn Auth + Send>>,
    ) -> Self {
        JsonClient { client, url, auth }
    }

    pub async fn request<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, ClientError> {
        let mut request_builder = self.client.request(method, format!("{}{}", self.url, url));

        {
            let mut auth = self.auth.lock().await;
            request_builder = auth.request(request_builder).await?
        }

        request_builder = request_builder.query(&query).json(&body);

        let response = match request_builder.send().await {
            Ok(response) => response,
            Err(e) => return Err(ClientError::RequestError(e)),
        };

        if response.status().is_success() {
            match response.json::<R>().await {
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

    pub async fn delete<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, ClientError> {
        self.request(Method::DELETE, url, query, body).await
    }

    pub async fn get<Q: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        query: Q,
    ) -> Result<R, ClientError> {
        self.request(Method::GET, url, query, Nothing).await
    }

    pub async fn head<Q: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        query: Q,
    ) -> Result<R, ClientError> {
        self.request(Method::HEAD, url, query, Nothing).await
    }

    pub async fn patch<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, ClientError> {
        self.request(Method::PATCH, url, query, body).await
    }

    pub async fn post<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, ClientError> {
        self.request(Method::POST, url, query, body).await
    }

    pub async fn put<Q: Serialize, B: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        query: Q,
        body: B,
    ) -> Result<R, ClientError> {
        self.request(Method::PUT, url, query, body).await
    }
}
