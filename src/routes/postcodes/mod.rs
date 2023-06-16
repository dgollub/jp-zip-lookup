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

/// Returns the list of all postal/zip codes for all prefectures as a JSON string array
pub async fn get_list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<String>>, StatusCode> {
    // NOTE(dkg): the <_> denotes an anonymous struct with one member, "zip_code: String"
    let rows: Vec<_> = sqlx::query!("SELECT DISTINCT zip_code FROM addresses ORDER BY zip_code")
        .fetch_all(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Error executing query: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows.into_iter().map(|r| r.zip_code).collect()))
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressByPostcode {
    pub zip_code: String,
    pub muncipalities_kanji: String,
    pub muncipalities_kana: String,
    pub muncipalities_kana_full: Option<String>,
    pub muncipalities_hiragana: Option<String>,
    pub muncipalities_romaji: Option<String>,
    pub town_kanji: String,
    pub town_kana: String,
    pub town_kana_full: Option<String>,
    pub town_hiragana: Option<String>,
    pub town_romaji: Option<String>,
}

pub async fn get_addresses_by_postcode(
    Path(postcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<AddressByPostcode>>, StatusCode> {
    // TODO(dkg): figure out how LIKE '%$1%' works with this query_as! macro,
    //            since I could not get it to work ....
    let len: i32 = cmp::max(2, cmp::min(7, postcode.len() as i32));
    let rows = sqlx::query_as!(
        AddressByPostcode,
        "SELECT
        zip_code,
        muncipalities_kanji,
        muncipalities_kana,
        null AS muncipalities_kana_full,
        null AS muncipalities_hiragana,
        null AS muncipalities_romaji,
        town_kanji,
        town_kana,
        null AS town_kana_full,
        null AS town_hiragana,
        null AS town_romaji
    FROM addresses
    WHERE substring(zip_code, 1, $1) = $2
    ORDER BY zip_code",
        len,
        &postcode
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
            let full_width_kana = half2kana(&row.muncipalities_kana);
            row.muncipalities_hiragana = Some(full_width_kana.to_hiragana());
            row.muncipalities_romaji = Some(full_width_kana.to_romaji());
            row.muncipalities_kana_full = Some(full_width_kana);

            let full_width_kana = half2kana(&row.town_kana);
            row.town_hiragana = Some(full_width_kana.to_hiragana());
            row.town_romaji = Some(full_width_kana.to_romaji());
            row.town_kana_full = Some(full_width_kana);
            row
        })
        .collect();

    Ok(Json(rows))
}
