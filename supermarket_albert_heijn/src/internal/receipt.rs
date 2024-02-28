use chrono::prelude::*;
use serde::Deserialize;

// TODO: address and currency can probably be moved to a more generic file

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: String,
    pub country_code: String,
    pub house_number: String,
    pub postal_code: String,
    pub street: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyAmount {
    // TODO: find a better way to represent decimal numbers
    pub amount: f64,
    pub currency: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptTotal {
    pub amount: CurrencyAmount,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptSummary {
    pub store_address: Address,
    pub total: ReceiptTotal,
    pub total_discount: CurrencyAmount,
    pub transaction_id: String,
    pub transaction_moment: DateTime<Local>,
}
