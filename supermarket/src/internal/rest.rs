use reqwest::{IntoUrl, RequestBuilder};

use crate::internal::auth::Auth;

#[derive(Debug)]
pub enum RestError {
    Request(reqwest::Error),
    Json(serde_json::Value),
    Text(String),
}

pub struct RestClient {
    client: reqwest::Client,
    auth: Box<dyn Auth>,
}

impl RestClient {
    pub fn new(client: reqwest::Client, auth: Box<dyn Auth>) -> Self {
        RestClient { client, auth }
    }

    // TODO: perform the request and parse errors instead of returning the builder

    pub fn delete<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.auth.request(self.client.delete(url))
    }

    pub fn head<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.auth.request(self.client.head(url))
    }

    pub fn get<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.auth.request(self.client.get(url))
    }

    pub fn patch<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.auth.request(self.client.patch(url))
    }

    pub fn post<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.auth.request(self.client.post(url))
    }

    pub fn put<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.auth.request(self.client.put(url))
    }
}
