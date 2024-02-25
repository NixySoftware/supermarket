use supermarket::Client;

pub struct JumboClient {}

impl Default for JumboClient {
    fn default() -> Self {
        JumboClient::new()
    }
}

impl JumboClient {
    pub fn new() -> Self {
        JumboClient {}
    }
}

impl Client for JumboClient {}
