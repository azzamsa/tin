use std::sync::Arc;

use async_graphql::{Context, FieldResult, Object};
use frunk_core::labelled::Transmogrifier;
use uuid::Uuid;

use super::model;
use crate::{context::ServerContext, scalar::Id};

#[derive(Default)]
pub struct UserQuery;

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
            .find_users(first, after.as_deref(), last, before.as_deref())
            .await?;
        let edges: Vec<model::UserEdge> = user_edges
            .into_iter()
            .map(frunk::labelled::Transmogrifier::transmogrify)
            .collect();

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
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        let result = ctx.user_service.find_user(id).await?;
        Ok(result.transmogrify())
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: model::input::CreateUserInput,
    ) -> FieldResult<model::User> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        let result = ctx.user_service.create_user(input.transmogrify()).await?;
        Ok(result.transmogrify())
    }
    pub async fn update_user(
        &self,
        ctx: &Context<'_>,
        input: model::input::UpdateUserInput,
    ) -> FieldResult<model::User> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        let result = ctx.user_service.update_user(input.transmogrify()).await?;
        Ok(result.transmogrify())
    }
    pub async fn delete_user(&self, ctx: &Context<'_>, id: Id) -> FieldResult<model::User> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        let result = ctx.user_service.delete_user(id).await?;
        Ok(result.transmogrify())
    }
}
