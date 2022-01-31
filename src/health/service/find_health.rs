use super::Service;
use crate::errors::Error;
use crate::health::entities::Health;

impl Service {
    pub async fn find_health(&self) -> Result<Health, Error> {
        let health = Health {
            status: "running".to_string(),
        };
        Ok(health)
    }
}
