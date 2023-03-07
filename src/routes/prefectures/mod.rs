use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use kana::half2kana;
use serde::{Deserialize, Serialize};
use wana_kana::ConvertJapanese;

use crate::state::AppState;

pub mod cities;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Prefecture {
    // NOTE(dkg): due to the "substring(...) AS ..." sqlx thinks the column can be null, hence the Option<> is required
    #[serde(rename(serialize = "prefCode"))]
    pub code: Option<String>,
    #[serde(rename(serialize = "halfWidthKana"))]
    pub half_width_kana: String,
    #[serde(rename(serialize = "fullWidthKana"))]
    pub full_width_kana: Option<String>,
    pub hiragana: Option<String>,
    #[serde(rename(serialize = "pref"))]
    pub kanji: String,
    pub romaji: Option<String>,
}

pub async fn get_list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Prefecture>>, StatusCode> {
    let rows = sqlx::query_as!(
        Prefecture,
        "SELECT DISTINCT
            substring(gov_code, 1, 2) AS code,
            prefecture_kana AS half_width_kana,
            prefecture_kanji AS kanji,
            null AS full_width_kana,
            null AS hiragana,
            null AS romaji
            FROM addresses
            ORDER BY substring(gov_code, 1, 2)",
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
            // TODO(dkg): I think it would make more sense to write a proper import script
            //            that sets those full kana, romaji and hiragana fields when the data
            //            is initially imported, instead of doing this manually for each request.
            let full_width_kana = half2kana(&row.half_width_kana);
            row.hiragana = Some(full_width_kana.to_hiragana());
            row.romaji = Some(full_width_kana.to_romaji());
            row.full_width_kana = Some(full_width_kana);
            row
        })
        .collect();

    Ok(Json(rows))
}
