use color_eyre::eyre::Result;
use sqlx::Row;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::constants::{DOWNLOAD_URL, UNZIPPED_NAME, ZIP_NAME};
use crate::database::PgPool;

pub async fn do_we_have_data(db: &PgPool) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) FROM addresses")
        .fetch_one(db)
        .await?;
    let count: i64 = row.get(0);
    Ok(count > 0)
}

pub async fn download_data_and_import_into_db(db: &PgPool) -> Result<()> {
    let folder = Path::new("data");
    if !folder.exists() {
        fs::create_dir(folder)?;
    }

    let mut download_file = PathBuf::from(folder);
    download_file.push(ZIP_NAME);
    let download_file = download_file.as_path();
    if download_file.exists() {
        tracing::warn!(
            "File {} already exists. Removing it first.",
            &download_file.display()
        );
        fs::remove_file(download_file)?;
    }

    tracing::info!(
        "Starting download of {} from {} ...",
        ZIP_NAME,
        DOWNLOAD_URL
    );

    let response = reqwest::get(DOWNLOAD_URL).await?;
    let mut file = std::fs::File::create(download_file)?;
    let mut content = io::Cursor::new(response.bytes().await?);

    std::io::copy(&mut content, &mut file)?;

    let mut unzipped_file = PathBuf::from(folder);
    unzipped_file.push(UNZIPPED_NAME);
    let unzipped_file = unzipped_file.as_path();
    if unzipped_file.exists() {
        tracing::warn!(
            "Unzipped file {} already exists. Removing it first.",
            &unzipped_file.display()
        );
        fs::remove_file(unzipped_file)?;
    }
    let target_dir = PathBuf::from(folder);
    let input_bytes = fs::read(download_file)?;

    zip_extract::extract(io::Cursor::new(input_bytes), &target_dir, true)?;

    tracing::info!("Download successfull. Will start import into database now.");

    let unzipped_file = fs::canonicalize(unzipped_file)?;
    let columns = r#"
        gov_code,
        zip_code_old,
        zip_code,
        prefecture_kana,
        muncipalities_kana,
        town_kana,
        prefecture_kanji,
        muncipalities_kanji,
        town_kanji,
        two_or_more_zip,
        address_numbered,
        with_chome,
        two_or_more_area,
        updated,
        change_reason
    "#;
    let format_options = "FORMAT CSV, DELIMITER ',', HEADER true, ENCODING 'shift-jis'";
    let sql = format!("COPY addresses ({columns}) FROM STDIN ({format_options})");
    let mut copy_in = db.copy_in_raw(&sql).await?;
    let file = tokio::fs::File::open(unzipped_file).await?;
    copy_in.read_from(file).await?;

    let rows_inserted = copy_in.finish().await?;

    tracing::info!("DONE! Imported {} rows.", rows_inserted);

    Ok(())
}
