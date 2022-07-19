use sqlx;
use uuid::Uuid;

use super::Repository;
use crate::{db::Queryer, errors::core::Error, user::entities};

impl Repository {
    pub async fn delete_user<'c, C: Queryer<'c>>(
        &self,
        db: C,
        user_id: Uuid,
    ) -> Result<entities::User, Error> {
        const QUERY: &str = "delete from user_ where id = $1 returning *";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(user_id)
            .fetch_one(db)
            .await
        {
            Err(err) => {
                log::error!("deleting user: {}", &err);
                Err(err.into())
            }
            Ok(user) => Ok(user),
        }
    }
}
