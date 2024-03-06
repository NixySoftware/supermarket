use supermarket::{Client, ClientError};

use crate::internal::{JumboInternalClient, JumboToken};

pub struct JumboClient {
    pub internal: JumboInternalClient,
}

impl JumboClient {
    pub fn new() -> Self {
        JumboClient {
            internal: JumboInternalClient::new(),
        }
    }

    pub async fn auth_with_code(&self, code: &str, code_verifier: &str) -> Result<(), ClientError> {
        self.internal.auth_with_code(code, code_verifier).await
    }

    pub async fn auth_with_refresh_token(&self, refresh_token: &str) -> Result<(), ClientError> {
        self.internal.auth_with_refresh_token(refresh_token).await
    }

    pub async fn token(&self) -> JumboToken {
        self.internal.token().await
    }

    pub async fn set_token(&self, token: JumboToken) {
        self.internal.set_token(token).await
    }
}

impl Client for JumboClient {}

impl Default for JumboClient {
    fn default() -> Self {
        JumboClient::new()
    }
}
