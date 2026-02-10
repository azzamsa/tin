use std::sync::Arc;

use async_graphql::{Context, FieldResult, Object};
use axum::{Json, response::IntoResponse};
use frunk_core::labelled::Transmogrifier;

use super::model;
use crate::context::ServerContext;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    pub async fn health(&self, ctx: &Context<'_>) -> FieldResult<model::Health> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        let result = ctx.health_service.get_health().await?;
        Ok(result.transmogrify())
    }
}

/// Test server health without invoking many
/// moving parts.
#[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "server is running", body = model::HealthResponse),
        ),
    )]
pub async fn health() -> impl IntoResponse {
    let data = model::Health {
        status: "running".into(),
    };
    let response = model::HealthResponse { data };
    Json(response)
}
