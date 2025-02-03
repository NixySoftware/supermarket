use serde::Deserialize;
use supermarket::Identifier;

use crate::internal::common::{Data, Image, PaginatedData, Price};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductCategories {
    pub categories: Data<ProductCategory>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductCategory {
    pub background_image_url: Option<String>,
    pub cat_id: String,
    pub cat_path: String,
    pub foreground_image_url: String,
    pub id: String,
    pub sub_categories_count: u64,
    pub thematic_aisle: bool,
    pub title: String,
}

impl Identifier for ProductCategory {
    fn identifier(&self) -> String {
        self.id.clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductSearch {
    // TODO: unknown inner type
    pub advertisements: Data<serde_json::Value>,
    pub filters: Filters,
    // TODO: unknown inner type
    pub horizontal_filters: Data<serde_json::Value>,
    pub products: PaginatedData<Product>,
    // TODO: unknown inner type
    pub product_lists: Data<serde_json::Value>,
    pub sort_options: Data<SortOption>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Product {
    // TODO: unknown inner type
    pub allergens: Option<serde_json::Value>,
    pub available: bool,
    pub availability: ProductAvailability,
    pub badge: Option<ProductBadge>,
    pub badges_to_display: ProductBadges,
    #[serde(rename = "crossSellSKUList")]
    pub cross_sell_sku_list: Vec<String>,
    pub id: String,
    pub image_info: Option<ProductImageInfo>,
    pub nix_product: bool,
    pub prices: ProductPrices,
    pub product_type: String,
    pub promotion: Option<ProductPromotion>,
    pub quantity: Option<String>,
    pub quantity_options: Vec<ProductQuantityOptions>,
    pub title: String,
    pub sample: bool,
    // TODO: unknown inner type
    pub surcharges: Vec<serde_json::Value>,
}

impl Identifier for Product {
    fn identifier(&self) -> String {
        self.id.clone()
    }
}

#[expect(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductAllergens {
    pub contains: Vec<ProductAllergen>,
    pub may_contain: Vec<ProductAllergen>,
}

#[expect(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductAllergen {
    pub key: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductAvailability {
    pub availability: String,
    pub sku: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductBadge {
    pub image: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductBadges {
    pub left_top: Option<ProductBadge>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductBadgeImage {
    pub main: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductImageInfo {
    pub primary_view: Vec<Image>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductPrices {
    pub price: Price,
    pub unit_price: Option<ProductUnitPrice>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductUnitPrice {
    pub unit: String,
    pub price: Price,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductPromotion {
    pub badge_image: ProductBadgeImage,
    pub enable_promo_assistance: bool,
    // TODO: timestamp
    pub from_date: u64,
    pub id: String,
    pub label: String,
    pub image: String,
    pub name: String,
    pub offline: bool,
    // TODO: unknown type
    pub sticker_badges: serde_json::Value,
    pub summary: String,
    pub tags: Vec<ProductPromotionTag>,
    // TODO: timestamp
    pub to_date: u64,
    pub validity_period: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductPromotionTag {
    pub r#type: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProductQuantityOptions {
    pub amount_step: u64,
    pub default_amount: u64,
    pub maximum_amount: u64,
    pub minimum_amount: u64,
    pub unit: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Filters {
    pub data: Vec<Filter>,
    pub active_filter_count: u64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Filter {
    pub title: String,
    pub r#type: String,
    pub items: Vec<FilterItem>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FilterItem {
    pub cat_id: Option<String>,
    pub cat_path: Option<String>,
    pub count: u64,
    pub dimension_name: String,
    pub filters: String,
    pub is_category: bool,
    pub thematic_aisle: Option<bool>,
    pub title: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SortOption {
    pub sort: String,
    pub title: String,
}
