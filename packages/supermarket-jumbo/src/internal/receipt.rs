use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::Europe::Amsterdam;
use serde::{Deserialize, Deserializer};
use serde_aux::field_attributes::deserialize_number_from_string;
use supermarket::Identifier;

fn deserialize_purchase_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let naive =
        NaiveDateTime::parse_from_str(&String::deserialize(deserializer)?, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;

    naive
        .and_local_timezone(Amsterdam)
        .single()
        .ok_or(serde::de::Error::custom(
            "Conversion to local timezone is ambiguous.",
        ))
        .map(|dt| dt.with_timezone(&Utc))
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptStore {
    pub id: u64,
    pub name: String,
}

impl Identifier for ReceiptStore {
    fn identifier(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptSummary {
    pub point_balance: u64,
    #[serde(deserialize_with = "deserialize_purchase_date")]
    pub purchase_end_on: DateTime<Utc>,
    pub receipt_source: String,
    pub store: ReceiptStore,
    pub transaction_id: String,
}

impl Identifier for ReceiptSummary {
    fn identifier(&self) -> String {
        self.transaction_id.clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LoyaltyCard {
    pub number: String,
}

impl Identifier for LoyaltyCard {
    fn identifier(&self) -> String {
        self.number.clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptCustomer {
    pub customer_id: String,
    pub loyalty_card: LoyaltyCard,
}

impl Identifier for ReceiptCustomer {
    fn identifier(&self) -> String {
        self.customer_id.clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptPoints {
    pub earned: u64,
    pub new_balance: u64,
    pub old_balance: u64,
    pub redeemed: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptDocuments {
    pub documents: Vec<ReceiptDocumentContainer>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptDocumentContainer {
    pub code_page: String,
    pub device: String,
    pub documents: Vec<ReceiptDocument>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub number_of_documents: u64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptDocument {
    pub codepage: String,
    pub print_sections: Vec<ReceiptPrintSection>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptPrintSection {
    // TODO: unknown inner type
    pub barcode_object: Option<serde_json::Value>,
    pub layout: String,
    pub print_commands: Vec<ReceiptPrintCommand>,
    pub section_id: String,
    pub text_objects: Vec<ReceiptTextObject>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptPrintCommand {
    pub cmd_data: String,
    pub command: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptTextObject {
    pub output_options: String,
    pub text_lines: Vec<ReceiptTextLine>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptTextLine {
    pub line_print_attributes: Vec<ReceiptLinePrintAttribute>,
    pub texts: Vec<ReceiptText>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptLinePrintAttribute {
    pub align: String,
    pub cpl: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptPrintAttribute {
    // TODO: unknown inner types
    pub bold: Option<serde_json::Value>,
    pub double_height: Option<serde_json::Value>,
    pub double_width: Option<serde_json::Value>,
    pub italic: Option<serde_json::Value>,
    pub underline: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReceiptText {
    // TODO: unknown inner type
    pub cpl: Option<serde_json::Value>,
    pub text: String,
    pub print_attributes: Vec<ReceiptPrintAttribute>,
}

fn deserialize_receipt_documents<'de, D>(deserializer: D) -> Result<ReceiptDocuments, D::Error>
where
    D: Deserializer<'de>,
{
    serde_json::from_str::<ReceiptDocuments>(&String::deserialize(deserializer)?)
        .map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase", tag = "type")]
pub enum ReceiptImage {
    #[serde(rename = "JSON", rename_all = "camelCase")]
    Json {
        #[serde(deserialize_with = "deserialize_receipt_documents")]
        image: ReceiptDocuments,
        receipt_points: ReceiptPoints,
    },
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Receipt {
    pub customer_details: ReceiptCustomer,
    pub id: String,
    #[serde(deserialize_with = "deserialize_purchase_date")]
    pub purchase_end_on: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_purchase_date")]
    pub purchase_start_on: DateTime<Utc>,
    pub receipt_source: String,
    pub receipt_image: ReceiptImage,
    pub store: ReceiptStore,
    pub transaction_id: String,
}

impl Identifier for Receipt {
    fn identifier(&self) -> String {
        self.id.clone()
    }
}
