use async_trait::async_trait;
use reqwest::RequestBuilder;

use crate::internal::client::ClientError;

#[async_trait]
pub trait Auth {
    async fn request(&mut self, builder: RequestBuilder) -> Result<RequestBuilder, ClientError> {
        Ok(builder)
    }
}

pub struct NoAuth {}

impl NoAuth {
    pub fn new() -> Self {
        NoAuth {}
    }
}

#[async_trait]
impl Auth for NoAuth {
    async fn request(&mut self, builder: RequestBuilder) -> Result<RequestBuilder, ClientError> {
        Ok(builder)
    }
}

impl Default for NoAuth {
    fn default() -> Self {
        NoAuth::new()
    }
}
