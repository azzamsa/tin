pub mod schema;

use anyhow::Context;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::get_env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> anyhow::Result<DbPool> {
    let url = &get_env("DATABASE_URL")?;
    let manager = ConnectionManager::new(url);
    Pool::builder()
        .build(manager)
        .context("failed to get database pool")
}
