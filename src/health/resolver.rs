use std::sync::Arc;

use async_graphql::{Context, Error, FieldResult, Object};

use super::model::Health;
use crate::context::ServerContext;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    pub async fn health(&self, ctx: &Context<'_>) -> FieldResult<Health> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.health_service.find_health().await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}
