use std::future::Future;

use reqwest::RequestBuilder;

pub trait Auth {
    fn request(
        &mut self,
        builder: RequestBuilder,
    ) -> impl Future<Output = Result<RequestBuilder, Box<dyn std::error::Error>>> {
        builder
    }
}

pub struct NoAuth {}

impl NoAuth {
    pub fn new() -> Self {
        NoAuth {}
    }
}

impl Auth for NoAuth {
    async fn request(
        &mut self,
        builder: RequestBuilder,
    ) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        Ok(builder)
    }
}

impl Default for NoAuth {
    fn default() -> Self {
        NoAuth::new()
    }
}
