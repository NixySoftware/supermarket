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
        let member = client.internal.member().await?;

        println!("{:#?}", member);

        Ok(())
    }
}
