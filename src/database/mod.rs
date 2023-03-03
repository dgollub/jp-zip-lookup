use std::env;

use sqlx::Pool;
use sqlx::{postgres::PgPoolOptions, Postgres};

use crate::constants::{
    DEFAULT_DB_HOST, DEFAULT_DB_NAME, DEFAULT_DB_PASSWORD, DEFAULT_DB_PORT, DEFAULT_DB_USER,
};

pub type PgPool = Pool<Postgres>;

#[derive(Default, Debug)]
pub struct DBOptions {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl DBOptions {
    pub fn new_from_env() -> Self {
        let host = env::var("JPZIP_DB_HOST").unwrap_or(DEFAULT_DB_HOST.into());
        let port = env::var("JPZIP_DB_PORT").unwrap_or(DEFAULT_DB_PORT.to_string());
        let name = env::var("JPZIP_DB_NAME").unwrap_or(DEFAULT_DB_NAME.into());
        let user = env::var("JPZIP_DB_USER").unwrap_or(DEFAULT_DB_USER.into());
        let password = env::var("JPZIP_DB_PASSWORD").unwrap_or(DEFAULT_DB_PASSWORD.into());
        let port = port.parse::<u16>().unwrap_or(DEFAULT_DB_PORT);

        DBOptions {
            host,
            port,
            name,
            user,
            password,
        }
    }
}

pub async fn init_database(options: DBOptions) -> Result<PgPool, sqlx::Error> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        options.user, options.password, options.host, options.port, options.name
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    Ok(pool)
}
