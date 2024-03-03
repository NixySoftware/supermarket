use chrono::prelude::*;
use serde::Deserialize;
use supermarket::Identifier;

// TODO: move generic structs to a separate file?

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Image {
    pub height: u64,
    pub url: String,
    pub width: u64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Page {
    pub number: u64,
    pub size: u64,
    pub total_elements: u64,
    pub total_pages: u64,
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
pub struct Filter {
    pub boolean_filter: bool,
    pub id: String,
    pub label: String,
    pub options: Vec<FilterOption>,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FilterOption {
    pub count: u64,
    pub display: bool,
    pub id: String,
    pub label: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
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
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductSubcategories {
    pub parent: ProductCategory,
    pub children: Vec<ProductCategory>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductSearch {
    // TODO: unknown inner type
    pub ads: Vec<serde_json::Value>,
    pub configuration: serde_json::Value,
    pub filters: Vec<Filter>,
    pub links: Links,
    pub page: Page,
    pub products: Vec<Product>,
    pub sort_on: Vec<String>,
    // TODO: unknown inner type
    pub taxonomy_nodes: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductDiscountLabel {
    pub code: String,
    pub count: Option<u64>,
    pub default_description: String,
    pub free_count: Option<u64>,
    // TODO: handle decimals better
    pub price: Option<f64>,
    // TODO: could just be integers?
    pub percentage: Option<f64>,
    pub precise_percentage: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductVirtualBundleItem {
    pub product_id: u64,
    pub quantity: u64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Product {
    pub action_id: Option<String>,
    pub auction_id: Option<String>,
    pub available_online: bool,
    pub brand: String,
    // #[serde(default, with = "iso_date_option")]
    // pub bonus_end_date: Option<DateTime<Utc>>,
    pub bonus_end_date: Option<NaiveDate>,
    pub bonus_mechanism: Option<String>,
    pub bonus_period_description: Option<String>,
    // #[serde(default, with = "iso_date_option")]
    // pub bonus_start_date: Option<DateTime<Utc>>,
    pub bonus_start_date: Option<NaiveDate>,
    pub bonus_segment_description: Option<String>,
    pub bonus_segment_id: Option<u64>,
    // TODO: handle decimals better
    pub current_price: Option<f64>,
    pub description_full: String,
    pub description_highlights: String,
    pub discount_labels: Vec<ProductDiscountLabel>,
    pub discount_type: Option<String>,
    pub extra_descriptions: Vec<String>,
    pub has_list_price: Option<bool>,
    pub hq_id: u64,
    pub images: Vec<Image>,
    pub is_bonus: bool,
    pub is_bonus_price: Option<bool>,
    pub is_infinite_bonus: bool,
    pub is_orderable: bool,
    pub is_previously_bought: bool,
    pub is_sample: bool,
    pub is_sponsored: bool,
    pub is_stapel_bonus: bool,
    pub is_virtual_bundle: bool,
    pub main_category: String,
    pub multiple_item_promotion: Option<bool>,
    pub nix18: bool,
    pub nutriscore: Option<String>,
    pub order_availability_status: String,
    // TODO: handle decimals better
    pub price_before_bonus: f64,
    pub product_count: Option<u64>,
    pub promotion_type: Option<String>,
    pub property_icons: Vec<String>,
    pub sales_unit_size: String,
    pub segment_type: Option<String>,
    pub shop_type: String,
    pub sub_category: String,
    pub title: String,
    pub unit_price_description: Option<String>,
    pub virtual_bundle_items: Option<Vec<ProductVirtualBundleItem>>,
    pub webshop_id: u64,
}

impl Identifier for Product {
    fn identifier(&self) -> String {
        self.webshop_id.to_string()
    }
}
