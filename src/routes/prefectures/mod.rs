use axum::Json;

use crate::responses::MessageResponse;

pub mod cities;

pub async fn get_list() -> Json<MessageResponse> {
    Json(MessageResponse::new("GET /prefectures".into()))
}
