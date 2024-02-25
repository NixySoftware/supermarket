use supermarket::Client;

pub struct JumboClient {}

impl JumboClient {
    pub fn new() -> Self {
        JumboClient {}
    }
}

impl Client for JumboClient {}

impl Default for JumboClient {
    fn default() -> Self {
        JumboClient::new()
    }
}
