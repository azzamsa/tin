use super::Service;
use crate::meta::entities;

impl Service {
    pub async fn find_meta(&self) -> Result<entities::Meta, crate::Error> {
        let meta = entities::Meta {
            build: option_env!("VCS_REVISION").unwrap_or("unknown").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        Ok(meta)
    }
}
