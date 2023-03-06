use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use kana::half2kana;
use serde::{Deserialize, Serialize};
use wana_kana::ConvertJapanese;

use crate::state::AppState;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct City {
    #[serde(rename(serialize = "govCode"))]
    pub gov_code: String,
    #[serde(rename(serialize = "zipCode"))]
    pub zip_code: String,
    #[serde(rename(serialize = "halfWidthKana"))]
    pub half_width_kana: String,
    #[serde(rename(serialize = "fullWidthKana"))]
    pub full_width_kana: Option<String>,
    pub hiragana: Option<String>,
    #[serde(rename(serialize = "city"))]
    pub kanji: String,
    pub romaji: Option<String>,
}

pub async fn get(
    Path(prefecture_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<City>>, StatusCode> {
    let rows = sqlx::query_as!(
        City,
        "SELECT DISTINCT
            gov_code,
            zip_code,
            town_kana AS half_width_kana,
            town_kanji AS kanji,
            null AS full_width_kana,
            null AS hiragana,
            null AS romaji
            FROM addresses
            WHERE substring(gov_code, 1, 2) = $1
            ORDER BY gov_code",
        &prefecture_code
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        eprintln!("Error executing query: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let rows = rows
        .into_iter()
        .map(|mut row| {
            let full_width_kana = half2kana(&row.half_width_kana);
            row.hiragana = Some(full_width_kana.to_hiragana());
            row.romaji = Some(full_width_kana.to_romaji());
            row.full_width_kana = Some(full_width_kana);
            row
        })
        .collect();

    Ok(Json(rows))
}
