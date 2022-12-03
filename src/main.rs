use axum::{response::Redirect, routing::get, Router};
use figlet_rs::FIGfont;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod constants;
pub mod responses;
pub mod routes;

use constants::*;

#[tokio::main]
async fn main() {
    // NOTE(dkg): make sure we have some log output
    let log_level = std::env::var("RUST_LOG");
    if log_level.is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

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
    let app = Router::new()
        .route("/", get(routes::index::get))
        .route("/healthcheck", get(routes::healthcheck::get))
        .route(
            "/lookup",
            get(|| async { Redirect::permanent("/lookup/zip") }),
        )
        .route("/lookup/zip", get(routes::lookup::zip::get))
        .layer(TraceLayer::new_for_http());

    let listening_address = std::env::var(ENV_SERVICE_LISTEN_ADDRESS)
        .unwrap_or(DEFAULT_SERVICE_LISTEN_ADDRESS.to_string());
    let listening_port =
        std::env::var(ENV_SERVICE_LISTEN_PORT).unwrap_or(DEFAULT_SERVICE_LISTEN_PORT.to_string());
    let listening_uri = format!("{}:{}", listening_address, listening_port);
    let listening_uri = listening_uri
        .parse()
        .expect("Could not parse listening interface and/or port");

    tracing::info!("Running on {}", &listening_uri);
    axum::Server::bind(&listening_uri)
        .serve(app.into_make_service())
        .await
        .expect("Service failed to start");
}
