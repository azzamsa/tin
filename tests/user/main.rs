use std::sync::Arc;

use anyhow::Result;
use tin::{config::Config, db};

mod create_user_without_full_name;
mod delete_user;
mod duplicate_username;
mod find_user;
mod graphql;
mod keep_existing_full_name;
mod relay;

async fn teardown() -> Result<()> {
    let config = Arc::new(Config::load()?);
    let conn = db::connect(&config.database).await?;
    sqlx::query("delete from user_").execute(&conn).await?;

    Ok(())
}
