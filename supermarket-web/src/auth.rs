use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};

#[derive(Debug, Clone)]
pub struct Credentials {}

#[derive(Debug, Clone)]
pub struct User {}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        todo!()
    }

    fn session_auth_hash(&self) -> &[u8] {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    // #[error(transparent)]
    // Discovery(DiscoveryError<AsyncHttpClientError>),
}

#[derive(Debug, Clone)]
pub struct Backend {}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type Credentials = Credentials;
    type Error = BackendError;
    type User = User;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        todo!()
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        todo!()
    }
}
