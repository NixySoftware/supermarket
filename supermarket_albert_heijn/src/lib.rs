mod client;
mod internal;

pub use client::AlbertHeijnClient;

#[cfg(test)]
mod tests {
    use super::*;
    use supermarket::internal::GraphQLClientError;

    #[tokio::test]
    async fn it_works() -> Result<(), GraphQLClientError> {
        let client = AlbertHeijnClient::new();
        if let Err(e) = client.auth_with_code("").await {
            return Err(GraphQLClientError::from(e));
        };

        println!("{:#?}", client.internal.token().await);

        let member = client.internal.member().await?;

        println!("{:#?}", member);

        Ok(())
    }
}
