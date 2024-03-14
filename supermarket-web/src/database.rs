use std::env;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn get_database_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    Database::connect(database_url).await
}
