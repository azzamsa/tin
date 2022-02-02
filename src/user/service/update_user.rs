use chrono::Utc;

use super::{Service, UpdateUserInput};
use crate::errors;
use crate::user::entities::User;

impl Service {
    pub async fn update_user(&self, input: UpdateUserInput) -> Result<User, errors::Error> {
        let username_exists = self.check_username_exists(&self.db, &input.name).await?;
        if username_exists {
            return Err(errors::core::Error::UsernameAlreadyExists.into());
        }

        let user_input = User {
            id: input.id,
            name: input.name,
            full_name: input.full_name,
            updated_at: Utc::now(),
            // FIXME
            created_at: Utc::now(),
        };

        let user = self.repo.update_user(&self.db, &user_input).await?;

        Ok(user)
    }
}
