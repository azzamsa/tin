use crate::db::DbPool;
use async_graphql::FieldResult;

use super::model;
use super::schema::{CreateUserInput, UpdateUserInput, User};

pub async fn read_all(pool: &DbPool) -> FieldResult<Vec<User>> {
    let users = model::find_all(pool)?;
    Ok(users)
}
pub async fn read(pool: &DbPool, id: i32) -> FieldResult<User> {
    let user = model::find(pool, id)?;
    Ok(user)
}
pub async fn create(pool: &DbPool, user_input: CreateUserInput) -> FieldResult<User> {
    let user = model::create(pool, user_input)?;
    Ok(user)
}
pub async fn update(pool: &DbPool, user_input: UpdateUserInput) -> FieldResult<User> {
    let user = model::update(pool, user_input)?;
    Ok(user)
}
pub async fn delete(pool: &DbPool, id: i32) -> FieldResult<User> {
    let user = model::delete(pool, id)?;
    Ok(user)
}
