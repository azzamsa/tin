use std::sync::Arc;

use async_graphql::{
    connection::{Connection, Edge},
    types::connection::{query, EmptyFields},
    Context, Error, FieldResult, Object,
};
use uuid::Uuid;

use super::model::{input, User};
use crate::{context::ServerContext, user::scalar::Id};

/// Relay connection result
pub type ConnectionResult<T> =
    async_graphql::Result<Connection<usize, T, EmptyFields, EmptyFields>>;

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserQuery {
    pub async fn users(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> ConnectionResult<User> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.user_service.find_users().await;
        match result {
            Ok(users) => {
                query(
                    after,
                    before,
                    first,
                    last,
                    |after, before, first, last| async move {
                        let iter_len = users.len();
                        let mut start = after.map(|after| after + 1).unwrap_or(0);
                        let mut end = before.unwrap_or(100);

                        if let Some(first) = first {
                            end = (start + first).min(end);
                        }

                        if let Some(last) = last {
                            start = if last > end - start { end } else { end - last };
                        }

                        let mut connection = Connection::new(start > 0, end < iter_len);
                        let iter = users.into_iter().map(|user| user.into());

                        connection.edges.extend(
                            (start..end)
                                .into_iter()
                                .zip(iter.skip(start).take(end - start))
                                .map(|(cursor, node)| Edge::new(cursor, node)),
                        );

                        Ok::<_, Error>(connection)
                    },
                )
                .await
            }
            Err(err) => Err(Error::new(err.to_string())),
        }
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
