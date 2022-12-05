use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::responses::MessageResponse;

pub async fn handler_404() -> impl IntoResponse {
    let msg = Json(MessageResponse::new("not found".into()));
    (StatusCode::NOT_FOUND, msg)
}
