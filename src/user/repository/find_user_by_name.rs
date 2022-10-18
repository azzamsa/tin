use sqlx;

use super::Repository;
use crate::{db::Queryer, errors::app::Error, user::entities};

impl Repository {
    pub async fn find_user_by_name<'c, C: Queryer<'c>>(
        &self,
        db: C,
        name: &str,
    ) -> Result<entities::User, Error> {
        const QUERY: &str = "SELECT * FROM user_ WHERE name = $1";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(name)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                log::error!("finding user by name: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::UserNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
