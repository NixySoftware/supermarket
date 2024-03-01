use serde::Deserialize;
use supermarket::Identifier;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub height: u64,
    pub url: String,
    pub width: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductCategory {
    pub id: u64,
    pub images: Vec<Image>,
    pub name: String,
    pub nix18: bool,
    pub slugified_name: String,
}

impl Identifier for ProductCategory {
    fn identifier(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSubcategory {
    pub parent: ProductCategory,
    pub children: Vec<ProductCategory>,
}
