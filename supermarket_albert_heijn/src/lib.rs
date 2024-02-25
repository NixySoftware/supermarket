mod client;
mod internal;

pub use client::AlbertHeijnClient;

#[cfg(test)]
mod tests {
    use super::*;
    use supermarket::internal::GraphQLError;

    #[tokio::test]
    async fn it_works() -> Result<(), GraphQLError> {
        let client = AlbertHeijnClient::new();
        let member = client.internal.member().await?;

        println!("{:#?}", member);

        Ok(())
    }
}
