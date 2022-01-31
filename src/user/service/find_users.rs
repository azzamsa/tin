use super::Service;
use crate::errors::Error;
use crate::user::entities::User;

impl Service {
    pub async fn find_users(&self) -> Result<Vec<User>, Error> {
        let users = self.repo.find_all_users(&self.db).await?;

        Ok(users)
    }
}
