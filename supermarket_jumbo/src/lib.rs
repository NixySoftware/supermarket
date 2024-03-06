mod client;
mod internal;

pub use client::JumboClient;

#[cfg(test)]
mod tests {
    use self::internal::JumboToken;

    use super::*;
    use supermarket::{credentials::Credentials, internal::GraphQLClientError};

    #[tokio::test]
    async fn it_works() -> Result<(), GraphQLClientError> {
        let mut credentials = Credentials::new();
        let client = JumboClient::new();

        if let Some(token) = credentials.get::<JumboToken>("jumbo") {
            client.set_token(token).await
        }

        // let profile = client.internal.profile().await?;
        // println!("{:#?}", profile);

        let receipts = client.internal.receipts().await?;
        println!("{:#?}", receipts);

        // let receipt = client.internal.receipt(&receipts[0].identifier()).await?;
        // println!("{:#?}", receipt);

        let token = client.token().await;
        credentials.set("jumbo", token);

        // let product_categories = client.internal.product_categories().await?;
        // println!("{:#?}", product_categories);

        // let product_subcategories = client
        //     .internal
        //     .product_subcategories(&product_categories[0].identifier())
        //     .await?;
        // println!("{:#?}", product_subcategories);

        // let suggestions = client.internal.search_suggestions("test").await?;
        // println!("{:#?}", suggestions);

        // let product_search = client.internal.search_products(Nothing).await?;
        // println!("{:#?}", product_search);

        Ok(())
    }
}
