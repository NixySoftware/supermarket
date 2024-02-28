use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ClientError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Value),
    TextError(String),
}

#[derive(Serialize, Deserialize)]
pub struct Nothing;
