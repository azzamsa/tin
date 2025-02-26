use chrono::Utc;
use ulid::Ulid;

use super::{CreateUserInput, Service};
use crate::domain::user::{Error, entities};

impl Service {
    pub async fn create_user(
        &self,
        input: CreateUserInput,
    ) -> Result<entities::User, crate::Error> {
        let username_exists = self.check_username_exists(&self.db, &input.name).await?;
        if username_exists {
            return Err(Error::UsernameAlreadyExists.into());
        }

        let user_input = entities::User {
            id: Ulid::new().into(),
            name: input.name,
            email: input.email,
            full_name: input.full_name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let user = self.repo.create_user(&self.db, &user_input).await?;
        self.notify_user(&user.email).await?;

        Ok(user)
    }
}
