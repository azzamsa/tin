use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::health::entities;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
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

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub data: Health,
}
