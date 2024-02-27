#[derive(Debug)]
pub enum ClientError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Value),
    TextError(String),
}
