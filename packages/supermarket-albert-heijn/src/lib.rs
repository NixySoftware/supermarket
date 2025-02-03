mod client;
mod internal;

pub use client::AlbertHeijnClient;

#[cfg(test)]
mod tests {
    use self::internal::AlbertHeijnToken;

    use super::*;
    use supermarket::{credentials::Credentials, internal::GraphQLClientError, Identifier};

    #[ignore]
    #[tokio::test]
    async fn it_works() -> Result<(), GraphQLClientError> {
        let mut credentials = Credentials::new();
        let client = AlbertHeijnClient::new();

        if let Some(token) = credentials.get::<AlbertHeijnToken>("albert_heijn") {
            client.set_token(token).await
        }

        let token = client.token().await;
        credentials.set("albert_heijn", token);

        // let member = client.internal.member().await?;
        // println!("{:#?}", member);

        let receipts = client.internal.receipts().await?;
        // println!("{:#?}", receipts);

        let receipt = client.internal.receipt(&receipts[1].identifier()).await?;
        println!("{:#?}", receipt);

        // let product_categories = client.internal.product_categories().await?;
        // println!("{:#?}", product_categories);

        // let product_subcategories = client
        //     .internal
        //     .product_subcategories(&product_categories[0].identifier())
        //     .await?;
        // println!("{:#?}", product_subcategories);

        // let product_search = client
        //     .internal
        //     .search_products([["bonus", "Bonus"]])
        //     .await?;
        // println!("{:#?}", product_search);

        // let product_category_search = client
        //     .internal
        //     .search_products_by_category(&product_categories[0].identifier())
        //     .await?;
        // println!("{:#?}", product_category_search);

        // let product_search_suggestions = client
        //     .internal
        //     .product_search_suggestions("AARDBEI NED", 10)
        //     .await?;
        // println!("{:#?}", product_search_suggestions);

        Ok(())
    }
}
