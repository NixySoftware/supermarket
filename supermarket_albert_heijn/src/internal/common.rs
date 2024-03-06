use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Address {
    pub city: String,
    pub country_code: String,
    pub house_number: String,
    pub postal_code: String,
    pub street: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CurrencyAmount {
    // TODO: find a better way to represent decimal numbers
    pub amount: f64,
    pub currency: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Image {
    pub height: u64,
    pub url: String,
    pub width: u64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Link {
    pub href: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Links {
    pub current: Link,
    pub first: Link,
    pub last: Link,
    pub next: Link,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Page {
    pub number: u64,
    pub size: u64,
    pub total_elements: u64,
    pub total_pages: u64,
}
