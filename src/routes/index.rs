use axum::Json;

use crate::{
    constants::{SERVICE_NAME, SERVICE_VERSION},
    responses::MessageResponse,
};

pub async fn get() -> Json<MessageResponse> {
    Json(MessageResponse::new(format!(
        "Service {} (version {}) is up and running.",
        SERVICE_NAME, SERVICE_VERSION
    )))
}
