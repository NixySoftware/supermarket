mod client;
mod internal;

pub use client::AlbertHeijnClient;

#[cfg(test)]
mod tests {
    use super::*;
    use supermarket::internal::ClientError;

    #[tokio::test]
    async fn it_works() -> Result<(), ClientError> {
        let client = AlbertHeijnClient::new();
        let member = client.internal.member().await?;

        println!("{:#?}", member);

        Ok(())
    }
}
