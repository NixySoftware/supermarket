use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Data<T> {
    pub data: Vec<T>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub offset: u64,
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
pub struct Price {
    pub amount: u64,
    pub currency: String,
}
