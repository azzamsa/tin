use std::sync::Arc;

use anyhow::Result;
use graph::{config::Config, db};

mod graphql;
pub mod schema;
//
mod create_user;
mod create_user_without_full_name;
mod delete_user;
mod duplicate_username;
mod find_user;
mod keep_existing_full_name;
mod relay;
mod update_user;

async fn teardown() -> Result<()> {
    let config = Arc::new(Config::load()?);
    let conn = db::connect(&config.database).await?;
    sqlx::query("delete from user_").execute(&conn).await?;

    Ok(())
}
