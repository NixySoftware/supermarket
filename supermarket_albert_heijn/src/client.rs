use supermarket::{Client, ClientError};

use crate::internal::AlbertHeijnInternalClient;

pub struct AlbertHeijnClient {
    pub internal: AlbertHeijnInternalClient,
}

impl AlbertHeijnClient {
    pub fn new() -> Self {
        AlbertHeijnClient {
            internal: AlbertHeijnInternalClient::new(),
        }
    }

    pub async fn auth_with_code(self, code: String) -> Result<(), ClientError> {
        self.internal.auth_with_code(code).await
    }

    pub async fn auth_with_refresh_token(self, refresh_token: String) -> Result<(), ClientError> {
        self.internal.auth_with_code(refresh_token).await
    }
}

impl Client for AlbertHeijnClient {}

impl Default for AlbertHeijnClient {
    fn default() -> Self {
        Self::new()
    }
}
