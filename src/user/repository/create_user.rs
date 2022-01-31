use sqlx;

use super::Repository;
use crate::user::entities;
use crate::{db::Queryer, errors::core::Error};

impl Repository {
    pub async fn create_user<'c, C: Queryer<'c>>(
        &self,
        db: C,
        user: &entities::User,
    ) -> Result<entities::User, Error> {
        const QUERY: &str = "insert into user_ (id, created_at, updated_at, 
                              name, full_name) values ($1, $2, $3, $4, $5) returning *";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(user.id)
            .bind(user.created_at)
            .bind(user.updated_at)
            //
            .bind(&user.name)
            .bind(&user.full_name)
            .fetch_one(db)
            .await
        {
            Err(err) => {
                log::error!("inserting user: {}", &err);
                Err(err.into())
            }
            Ok(user) => Ok(user),
        }
    }
}
