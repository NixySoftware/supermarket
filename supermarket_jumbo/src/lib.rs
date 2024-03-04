mod client;
mod internal;

pub use client::JumboClient;

#[cfg(test)]
mod tests {
    use super::*;
    use supermarket::{internal::GraphQLClientError, serde::Nothing, Identifier};

    #[tokio::test]
    async fn it_works() -> Result<(), GraphQLClientError> {
        let client = JumboClient::new();

        // let product_categories = client.internal.product_categories().await?;
        // println!("{:#?}", product_categories);

        // let product_subcategories = client
        //     .internal
        //     .product_subcategories(&product_categories[0].identifier())
        //     .await?;
        // println!("{:#?}", product_subcategories);

        // let suggestions = client.internal.search_suggestions("test").await?;
        // println!("{:#?}", suggestions);

        let product_search = client.internal.search_products(Nothing).await?;
        println!("{:#?}", product_search);

        Ok(())
    }
}
