use async_graphql::{FieldResult, Object};

use super::schema::Health;
use super::service;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    pub async fn health(&self) -> FieldResult<Health> {
        service::read().await
    }
}
