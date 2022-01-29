use async_graphql::{FieldResult, Object};

use super::schema::Meta;
use super::service;

#[derive(Default)]
pub struct MetaQuery;

#[Object]
impl MetaQuery {
    pub async fn meta(&self) -> FieldResult<Meta> {
        service::read().await
    }
}
