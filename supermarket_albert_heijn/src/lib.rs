mod client;
mod internal;

pub use client::AlbertHeijnClient;

#[cfg(test)]
mod tests {
    use self::internal::AlbertHeijnToken;

    use super::*;
    use supermarket::{credentials::Credentials, internal::GraphQLClientError, Identifier};

    #[tokio::test]
    async fn it_works() -> Result<(), GraphQLClientError> {
        let mut credentials = Credentials::new();
        let client = AlbertHeijnClient::new();

        if let Some(token) = credentials.get::<AlbertHeijnToken>("albert_heijn") {
            client.set_token(token).await
        }

        let member = client.internal.member().await?;
        println!("{:#?}", member);

        let receipts = client.internal.receipts().await?;
        // println!("{:#?}", receipts);

        let receipt = client.internal.receipt(&receipts[0].identifier()).await?;
        println!("{:#?}", receipt);

        let token = client.token().await;
        credentials.set("albert_heijn", token);

        Ok(())
    }
}
