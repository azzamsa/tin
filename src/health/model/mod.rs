use async_graphql::SimpleObject;
use serde::Serialize;
use utoipa::Component;

use crate::health::entities;

#[derive(Debug, SimpleObject, Serialize, Component)]
pub struct Health {
    pub status: String,
}

impl From<entities::Health> for Health {
    fn from(health: entities::Health) -> Self {
        Self {
            status: health.status,
        }
    }
}

#[derive(Debug, Serialize, Component)]
pub struct HealthResponse {
    pub data: Health,
}
