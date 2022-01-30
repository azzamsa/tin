use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::get_env;

pub fn get_pool() -> anyhow::Result<PgPool> {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&get_env("DATABASE_URL")?)
        .context("failed to connect to database")
}
