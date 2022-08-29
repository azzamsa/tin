use super::Service;
use crate::{errors::Error, health::entities};

impl Service {
    pub async fn find_health(&self) -> Result<entities::Health, Error> {
        let health = entities::Health {
            status: "running".to_string(),
        };
        Ok(health)
    }
}
