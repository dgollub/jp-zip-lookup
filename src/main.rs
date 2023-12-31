use std::sync::Arc;

use axum::{routing::get, Router};
use color_eyre::eyre::Result;
use figlet_rs::FIGfont;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod constants;
pub mod database;
pub mod responses;
pub mod routes;
pub mod state;
pub mod utils;

use crate::constants::*;
use crate::database::{init_database, DBOptions};
use crate::state::AppState;
use crate::utils::{do_we_have_data, download_data_and_import_into_db};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    color_eyre::install()?;

    // NOTE(dkg): make sure we have some log output
    let log_level = std::env::var("RUST_LOG");
    if log_level.is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // TODO(dkg): filter out sqlx queries _or_ rather make that logging optional via special env var
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let standard_font = FIGfont::standard().expect("Could not load figlet font");
    let figure = standard_font
        .convert(SERVICE_NAME)
        .expect("Could not convert string to figlet font");

    tracing::info!("\n{}", figure);
    tracing::info!("{} version {}", SERVICE_NAME, SERVICE_VERSION);

    // TODO(dkg): try and get rid of the "get(...)" wrapper
    // TODO(dkg): implement middleware that checks the API key (see postgres-setup.sql and seed-data.sql)
    let app = Router::new()
        .route("/", get(routes::index::get))
        .route("/healthcheck", get(routes::healthcheck::get))
        .route("/postcodes", get(routes::postcodes::get_list))
        .route(
            "/postcodes/:postcode",
            get(routes::postcodes::get_addresses_by_postcode),
        )
        .route("/prefectures", get(routes::prefectures::get_list))
        .route(
            "/prefectures/:prefecture_code/cities",
            get(routes::prefectures::cities::get),
        )
        .layer(TraceLayer::new_for_http());

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(routes::errors::handler_404);

    let listening_address = std::env::var(ENV_SERVICE_LISTEN_ADDRESS)
        .unwrap_or(DEFAULT_SERVICE_LISTEN_ADDRESS.to_string());
    let listening_port =
        std::env::var(ENV_SERVICE_LISTEN_PORT).unwrap_or(DEFAULT_SERVICE_LISTEN_PORT.to_string());
    let listening_uri = format!("{}:{}", listening_address, listening_port);
    let listening_uri = listening_uri
        .parse()
        .expect("Could not parse listening interface and/or port");

    let options = DBOptions::new_from_env();

    tracing::info!("---------------------------------------------");
    tracing::info!(
        "Using database at {}:{}/{}",
        &options.host,
        &options.port,
        &options.name
    );
    tracing::info!("---------------------------------------------");

    let db = init_database(options).await?;

    let we_have_data = do_we_have_data(&db).await?;

    if !we_have_data {
        tracing::warn!("No address data found.");

        // If JSZIP_DISABLE_AUTO_INIT_DB=1 we should not try to automatically download and import data.
        let disable_auto_initialize_database = std::env::var("JSZIP_DISABLE_AUTO_INIT_DB")
            .unwrap_or("0".into())
            .eq_ignore_ascii_case("1");

        let info = if disable_auto_initialize_database {
            "disabled"
        } else {
            "enabled"
        };
        tracing::info!("Automatic database initalization is {info}!");

        if disable_auto_initialize_database {
            tracing::warn!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            tracing::warn!("Since automatic database initalization is disabled you will");
            tracing::warn!("need to manually download and import the required data into");
            tracing::warn!("the database.");
            tracing::warn!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        } else {
            download_data_and_import_into_db(&db).await?;
        }
    }

    let app_state = AppState { db };
    let shared_state = Arc::new(app_state);
    let app = app.with_state(shared_state);

    tracing::info!("---------------------------------------------");
    tracing::info!("Running on {}", &listening_uri);
    // TODO(dkg): find a way to do this automagically
    tracing::info!("---------------------------------------------");
    tracing::info!("Available routes:");
    tracing::info!("\t GET /healthcheck");
    tracing::info!("\t GET /postcodes");
    tracing::info!("\t GET /postcodes/:postcode");
    tracing::info!("\t GET /prefectures");
    tracing::info!("\t GET /prefectures/:prefecture_code/cities");

    axum::Server::bind(&listening_uri)
        .serve(app.into_make_service())
        .await
        .expect("Service failed to start");

    Ok(())
}
