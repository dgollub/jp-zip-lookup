use crate::database::PgPool;

pub struct AppState {
    pub db: PgPool,
}
