pub use sea_orm_migration::prelude::*;

mod base;
mod m20240314_090725_create_auth;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240314_090725_create_auth::Migration)]
    }
}
