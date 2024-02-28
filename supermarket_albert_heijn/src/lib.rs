mod client;
mod internal;

pub use client::AlbertHeijnClient;

#[cfg(test)]
mod tests {
    use self::internal::AlbertHeijnToken;

    use super::*;
    use supermarket::{credentials::Credentials, internal::GraphQLClientError};

    #[tokio::test]
    async fn it_works() -> Result<(), GraphQLClientError> {
        let mut credentials = Credentials::new();
        let client = AlbertHeijnClient::new();

        if let Some(token) = credentials.get::<AlbertHeijnToken>("albert_heijn") {
            client.set_token(token).await
        }

        let member = client.internal.member().await?;
        println!("{:#?}", member);

        let token = client.token().await;
        println!("{:#?}", token);
        credentials.set("albert_heijn", token);

        Ok(())
    }
}
