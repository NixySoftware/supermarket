use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};
use supermarket::internal::{Auth, GraphQLClient, JsonClient, NoAuth};
use tokio::sync::Mutex;

const API_URL: &str = "https://mobileapi.jumbo.com";
const GRAPHQL_API_URL: &str = "https://www.jumbo.com/api/graphql";

const APP_NAME: &str = "Jumbo";
const APP_SOURCE: &str = "JUMBO-APP";
const APP_VERSION: &str = "11.1.0";

fn new_api_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert("x-source", HeaderValue::from_static(APP_SOURCE));

    reqwest::Client::builder()
        .default_headers(headers)
        .gzip(true)
        .user_agent(format!("{}/{}", APP_NAME, APP_VERSION))
        .build()
        .unwrap()
}

fn new_graphql_api_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert("x-source", HeaderValue::from_static(APP_SOURCE));

    reqwest::Client::builder()
        .default_headers(headers)
        .gzip(true)
        .user_agent(format!("{}/{}", APP_NAME, APP_VERSION))
        .build()
        .unwrap()
}
pub struct JumboInternalClient {
    // auth: Arc<Mutex<JumboAuth>>,
    graphql_client: GraphQLClient,
    json_client: JsonClient,
}

impl JumboInternalClient {
    pub fn new() -> Self {
        // let auth = Arc::new(Mutex::new(JumboAuth::new(JsonClient::new(
        //     new_api_client(),
        //     API_URL,
        //     Arc::new(Mutex::new(NoAuth::new())),
        // ))));

        let auth = Arc::new(Mutex::new(NoAuth::new()));

        JumboInternalClient {
            // auth: Arc::clone(&auth),
            graphql_client: GraphQLClient::new(
                new_graphql_api_client(),
                GRAPHQL_API_URL,
                Arc::clone(&auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
            json_client: JsonClient::new(
                new_api_client(),
                API_URL,
                Arc::clone(&auth) as Arc<Mutex<dyn Auth + Send>>,
            ),
        }
    }
}

impl Default for JumboInternalClient {
    fn default() -> Self {
        Self::new()
    }
}
