use chrono::{DateTime, Utc};
use serde::Deserialize;
use supermarket::Identifier;

use crate::internal::common::{Address, CurrencyAmount};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptTotal {
    pub amount: CurrencyAmount,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptSummary {
    pub store_address: Address,
    pub total: ReceiptTotal,
    pub total_discount: CurrencyAmount,
    pub transaction_id: String,
    pub transaction_moment: DateTime<Utc>,
}

impl Identifier for ReceiptSummary {
    fn identifier(&self) -> String {
        self.transaction_id.clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase", tag = "type")]
pub enum ReceiptUiItem {
    #[serde(rename = "ah-logo", rename_all = "camelCase")]
    AhLogo { style: String },

    #[serde(rename_all = "camelCase")]
    Divider { center_text: Option<String> },

    #[serde(rename = "four-text-column", rename_all = "camelCase")]
    FourTextColumn {
        first: Option<String>,
        second: Option<String>,
        third: Option<String>,
        fourth: Option<String>,
    },

    #[serde(rename_all = "camelCase")]
    Product {
        amount: Option<String>,
        description: String,
        indicator: Option<String>,
        price: Option<String>,
        quantity: Option<String>,
    },

    #[serde(rename = "products-header", rename_all = "camelCase")]
    ProductsHeader {},

    #[serde(rename_all = "camelCase")]
    Spacer {},

    #[serde(rename_all = "camelCase")]
    Subtotal {
        amount: String,
        quantity: String,
        text: String,
    },

    #[serde(rename = "tech-info", rename_all = "camelCase")]
    TechInfo {
        date_time: DateTime<Utc>,
        lane: u64,
        operator: Option<String>,
        store: u64,
        transaction: u64,
    },

    #[serde(rename_all = "camelCase")]
    Text {
        // TODO: this can only be LEFT, CENTER, RIGHT
        alignment: String,
        is_bold: bool,
        value: String,
    },

    #[serde(rename_all = "camelCase")]
    Total { label: String, price: String },

    #[serde(rename_all = "camelCase")]
    Vat {
        center: String,
        left: String,
        right: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Receipt {
    pub receipt_ui_items: Vec<ReceiptUiItem>,
    pub store_id: u64,
    #[serde(default)]
    pub transaction_id: String,
    pub transaction_moment: DateTime<Utc>,
}

impl Identifier for Receipt {
    fn identifier(&self) -> String {
        self.transaction_id.clone()
    }
}
