use async_graphql::{Context, FieldResult, Object};

use super::schema::{CreateUserInput, UpdateUserInput, User};
use super::service;

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserQuery {
    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
        let pool = ctx.data()?;
        service::read_all(pool).await
    }
    pub async fn user(&self, ctx: &Context<'_>, id: i32) -> FieldResult<User> {
        let pool = ctx.data()?;
        service::read(pool, id).await
    }
}

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> FieldResult<User> {
        let pool = ctx.data()?;
        service::create(pool, input).await
    }
    pub async fn update_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInput,
    ) -> FieldResult<User> {
        let pool = ctx.data()?;
        service::update(pool, input).await
    }
    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> FieldResult<User> {
        let pool = ctx.data()?;
        service::delete(pool, id).await
    }
}
