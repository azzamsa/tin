use async_graphql::SimpleObject;

use crate::domain::meta::entities;

#[derive(Debug, SimpleObject)]
pub struct Meta {
    pub build: String,
    pub version: String,
}

impl From<entities::Meta> for Meta {
    fn from(meta: entities::Meta) -> Self {
        Self {
            build: meta.build,
            version: meta.version,
        }
    }
}
