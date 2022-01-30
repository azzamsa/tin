use async_graphql::FieldResult;
use sqlx::PgPool;

use super::model;
use super::schema::{CreateUserInput, UpdateUserInput, User};

pub async fn read_all(pool: &PgPool) -> FieldResult<Vec<User>> {
    let users = model::find_all(pool).await?;
    Ok(users)
}
pub async fn read(pool: &PgPool, id: i32) -> FieldResult<User> {
    let user = model::find(pool, id).await?;
    Ok(user)
}
pub async fn create(pool: &PgPool, user_input: CreateUserInput) -> FieldResult<User> {
    let user = model::create(pool, user_input).await?;
    Ok(user)
}
pub async fn update(pool: &PgPool, user_input: UpdateUserInput) -> FieldResult<User> {
    let user = model::update(pool, user_input).await?;
    Ok(user)
}
pub async fn delete(pool: &PgPool, id: i32) -> FieldResult<User> {
    let user = model::delete(pool, id).await?;
    Ok(user)
}
