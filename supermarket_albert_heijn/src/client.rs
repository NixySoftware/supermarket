use supermarket::Client;

use crate::internal::AlbertHeijnInternalClient;

pub struct AlbertHeijnClient {
    pub internal: AlbertHeijnInternalClient,
}

impl AlbertHeijnClient {
    pub fn new() -> Self {
        AlbertHeijnClient {
            internal: AlbertHeijnInternalClient::new(),
        }
    }
}

impl Client for AlbertHeijnClient {}

impl Default for AlbertHeijnClient {
    fn default() -> Self {
        Self::new()
    }
}
