use super::Store;
use sqlx::PgPool;

#[derive(Debug)]
pub struct PsqlStore {
    pub pool: PgPool,
}

impl PsqlStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Store for PsqlStore {}
