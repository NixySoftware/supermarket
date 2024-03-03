use supermarket::Client;

use crate::internal::JumboInternalClient;

pub struct JumboClient {
    pub internal: JumboInternalClient,
}

impl JumboClient {
    pub fn new() -> Self {
        JumboClient {
            internal: JumboInternalClient::new(),
        }
    }
}

impl Client for JumboClient {}

impl Default for JumboClient {
    fn default() -> Self {
        JumboClient::new()
    }
}
