use supermarket::{Client, ClientError};

use crate::internal::AlbertHeijnInternalClient;
use crate::internal::AlbertHeijnToken;

pub struct AlbertHeijnClient {
    pub internal: AlbertHeijnInternalClient,
}

impl AlbertHeijnClient {
    pub fn new() -> Self {
        AlbertHeijnClient {
            internal: AlbertHeijnInternalClient::new(),
        }
    }

    pub async fn auth_with_code(&self, code: &str) -> Result<(), ClientError> {
        self.internal.auth_with_code(code).await
    }

    pub async fn auth_with_refresh_token(&self, refresh_token: &str) -> Result<(), ClientError> {
        self.internal.auth_with_code(refresh_token).await
    }

    pub async fn token(&self) -> AlbertHeijnToken {
        self.internal.token().await
    }

    pub async fn set_token(&self, token: AlbertHeijnToken) {
        self.internal.set_token(token).await
    }
}

impl Client for AlbertHeijnClient {}

impl Default for AlbertHeijnClient {
    fn default() -> Self {
        Self::new()
    }
}
