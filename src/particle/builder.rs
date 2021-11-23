use serde::*;

use super::Request;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Builder {
    pub requests: Vec<Request>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            requests: Vec::new(),
        }
    }

    pub fn add_request(&mut self, request: Request) {
        self.requests.push(request);
    }
}
