use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageResponse {
    message: String,
}

impl MessageResponse {
    pub fn new(msg: String) -> Self {
        Self { message: msg }
    }
}
