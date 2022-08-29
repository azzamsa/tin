use std::sync::Arc;

use async_graphql::{Context, Error, FieldResult, Object};
use uuid::Uuid;

use super::{model, service};
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
    ) -> FieldResult<model::UserConnection> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;
        let user_edges = server_ctx
            .user_service
            .find_users(first, after.clone(), last, before.clone())
            .await?;
        let edges: Vec<model::UserEdge> = user_edges.into_iter().map(|user| user.into()).collect();

        let user_connection = model::UserConnection {
            edges,
            //
            after,
            before,
            first,
            last,
        };

        Ok(user_connection)
    }
    pub async fn user(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<model::User> {
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
        input: model::input::CreateUserInput,
    ) -> FieldResult<model::User> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let service_input = service::CreateUserInput {
            name: input.name,
            full_name: input.full_name,
        };
        let result = server_ctx.user_service.create_user(service_input).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
    pub async fn update_user(
        &self,
        ctx: &Context<'_>,
        input: model::input::UpdateUserInput,
    ) -> FieldResult<model::User> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let service_input = service::UpdateUserInput {
            id: input.id,
            name: input.name,
            full_name: input.full_name,
        };
        let result = server_ctx.user_service.update_user(service_input).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
    pub async fn delete_user(&self, ctx: &Context<'_>, id: Id) -> FieldResult<model::User> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.user_service.delete_user(id).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}
