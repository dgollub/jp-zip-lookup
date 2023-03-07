use std::{cmp, sync::Arc};

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
    // clamp the substring to a value between [2, 5]
    let len: i32 = cmp::max(2, cmp::min(5, prefecture_code.len() as i32));
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
            WHERE substring(gov_code, 1, $1) = $2
            ORDER BY gov_code",
        len,
        &prefecture_code,
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
