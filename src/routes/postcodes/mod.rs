use axum::{extract::Path, Json};

use crate::responses::MessageResponse;

pub async fn get_list() -> Json<MessageResponse> {
    Json(MessageResponse::new("GET /postcodes".into()))
}

pub async fn get_by_postcode(Path(postcode): Path<String>) -> Json<MessageResponse> {
    Json(MessageResponse::new(format!("GET /postcodes/{postcode}")))
}
