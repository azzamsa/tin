use chrono::Utc;
use ulid::Ulid;

use super::{CreateUserInput, Service};
use crate::{errors, user::entities::User};

impl Service {
    pub async fn create_user(&self, input: CreateUserInput) -> Result<User, errors::Error> {
        let username_exists = self.check_username_exists(&self.db, &input.name).await?;
        if username_exists {
            return Err(errors::core::Error::UsernameAlreadyExists.into());
        }

        let user_input = User {
            id: Ulid::new().into(),
            name: input.name,
            full_name: input.full_name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let user = self.repo.create_user(&self.db, &user_input).await?;

        Ok(user)
    }
}
