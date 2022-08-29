use super::Service;
use crate::health::entities;

impl Service {
    pub async fn find_health(&self) -> Result<entities::Health, crate::Error> {
        let health = entities::Health {
            status: "running".to_string(),
        };
        Ok(health)
    }
}
