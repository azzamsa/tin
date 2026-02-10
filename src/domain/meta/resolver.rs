use std::sync::Arc;

use async_graphql::{Context, FieldResult, Object};
use frunk_core::labelled::Transmogrifier;

use super::model;
use crate::context::ServerContext;

#[derive(Default)]
pub struct MetaQuery;

#[Object]
impl MetaQuery {
    pub async fn meta(&self, ctx: &Context<'_>) -> FieldResult<model::Meta> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        let result = ctx.meta_service.get_meta().await?;
        Ok(result.transmogrify())
    }
}
