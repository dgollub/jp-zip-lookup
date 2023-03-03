#![allow(dead_code)]

pub const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");
pub const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");

// Environment variable names
pub const ENV_SERVICE_LISTEN_PORT: &str = "SERVICE_LISTEN_PORT";
pub const ENV_SERVICE_LISTEN_ADDRESS: &str = "SERVICE_LISTEN_ADDRESS";

// Environment variable default values
pub const DEFAULT_SERVICE_LISTEN_PORT: u16 = 8900;
pub const DEFAULT_SERVICE_LISTEN_ADDRESS: &str = "0.0.0.0";

pub const DEFAULT_DB_HOST: &str = "localhost";
pub const DEFAULT_DB_PORT: u16 = 5432;
pub const DEFAULT_DB_NAME: &str = "jpzip";
pub const DEFAULT_DB_USER: &str = "jpzip";
pub const DEFAULT_DB_PASSWORD: &str = "jpzip";
