use axum::Json;

use crate::responses::MessageResponse;

pub async fn get() -> Json<MessageResponse> {
    Json(MessageResponse::new("GET lookup/ZIP".into()))
}
