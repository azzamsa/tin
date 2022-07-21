use std::sync::Arc;

use async_graphql::{Context, Error, FieldResult, Object};
use uuid::Uuid;

use super::model::{input, User, UserConnection};
use crate::{context::ServerContext, user::scalar::Id};

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserQuery {
    pub async fn users(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> FieldResult<UserConnection> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;
        let edges = server_ctx
            .user_service
            .find_users(first, after.clone(), last, before.clone())
            .await?;

        let user_connection = UserConnection {
            edges,
            //
            after,
            before,
            first,
            last,
        };

        Ok(user_connection)
    }
    pub async fn user(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<User> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.user_service.find_user(id).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: input::CreateUserInput,
    ) -> FieldResult<User> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.user_service.create_user(input.into()).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
    pub async fn update_user(
        &self,
        ctx: &Context<'_>,
        input: input::UpdateUserInput,
    ) -> FieldResult<User> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.user_service.update_user(input.into()).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
    pub async fn delete_user(&self, ctx: &Context<'_>, id: Id) -> FieldResult<User> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.user_service.delete_user(id).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}
