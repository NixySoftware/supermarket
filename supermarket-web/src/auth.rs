use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use sea_orm::{prelude::Uuid, DatabaseConnection, DbErr, EntityTrait};

use crate::entities::user;

#[derive(Debug, Clone)]
pub enum Credentials {
    Oidc,
}

impl AuthUser for user::Model {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Database(#[from] DbErr),
}

#[derive(Debug, Clone)]
pub struct Backend {
    database: DatabaseConnection,
}

impl Backend {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type Credentials = Credentials;
    type Error = BackendError;
    type User = user::Model;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        todo!()
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = user::Entity::find_by_id(*user_id)
            .one(&self.database)
            .await?;

        Ok(user)
    }
}
