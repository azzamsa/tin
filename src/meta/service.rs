use async_graphql::FieldResult;

use super::schema::Meta;

pub async fn read() -> FieldResult<Meta> {
    let meta = Meta {
        build: option_env!("VCS_REVISION").unwrap_or("unknown").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(meta)
}
