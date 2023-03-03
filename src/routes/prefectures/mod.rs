use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

pub mod cities;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Prefecture {
    // NOTE(dkg): due to the "substring(...) AS ..." sqlx thinks the column can be null, hence the Option<> is required
    #[serde(rename(serialize = "prefCode"))]
    pub code: Option<String>,
    #[serde(rename(serialize = "halfWidthKana"))]
    pub half_width_kana: String,
    #[serde(rename(serialize = "pref"))]
    pub kanji: String,
}

pub async fn get_list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Prefecture>>, StatusCode> {
    let db = &state.db;

    // TODO(dkg): add hiragana and fullwidth katakana
    let rows = sqlx::query_as!(
        Prefecture,
        "SELECT DISTINCT
            substring(gov_code, 1, 2) AS code,
            prefecture_kana AS half_width_kana,
            prefecture_kanji AS kanji
            FROM addresses
            ORDER BY substring(gov_code, 1, 2)",
    )
    .fetch_all(db)
    .await
    .map_err(|err| {
        eprintln!("Error executing query: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rows))
}
