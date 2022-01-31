use uuid::Uuid;

use super::Service;
use crate::errors::Error;
use crate::user::entities::User;

impl Service {
    pub async fn delete_user(&self, user_id: Uuid) -> Result<User, Error> {
        let user = self.repo.delete_user(&self.db, user_id).await?;

        Ok(user)
    }
}
