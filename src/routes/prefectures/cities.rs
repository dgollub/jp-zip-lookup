use axum::{extract::Path, Json};

use crate::responses::MessageResponse;

pub async fn get(Path(prefecture_code): Path<String>) -> Json<MessageResponse> {
    Json(MessageResponse::new(format!(
        "GET /prefectures/{prefecture_code}/cities"
    )))
}
