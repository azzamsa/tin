use uuid::Uuid;

use super::Service;
use crate::{errors::Error, user::entities};

impl Service {
    pub async fn delete_user(&self, user_id: Uuid) -> Result<entities::User, Error> {
        let user = self.repo.delete_user(&self.db, user_id).await?;

        Ok(user)
    }
}
