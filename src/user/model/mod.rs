pub mod input;
use std::sync::Arc;

use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sqlx::Row;

use crate::{
    context::ServerContext,
    relay::Base64Cursor,
    user::{
        entities,
        scalar::{Id, Time},
    },
};

#[derive(Debug, SimpleObject)]
pub struct User {
    /// The ID of the User.
    pub id: Id,
    pub created_at: Time,

    /// The name for the User.
    pub name: String,
    /// The full name for the User.
    pub full_name: Option<String>,
}

impl From<entities::User> for User {
    fn from(user: entities::User) -> Self {
        Self {
            id: user.id,
            created_at: user.created_at,

            name: user.name,
            full_name: user.full_name,
        }
    }
}

#[derive(Debug, SimpleObject)]
pub struct UserEdge {
    // The item at the end of the edge.
    pub node: User,
    // A cursor for use in pagination.
    pub cursor: String,
}

impl From<entities::UserEdge> for UserEdge {
    fn from(user: entities::UserEdge) -> Self {
        Self {
            node: user.node.into(),
            cursor: user.cursor,
        }
    }
}

impl From<entities::User> for UserEdge {
    fn from(user: entities::User) -> Self {
        let cursor = Base64Cursor::new(user.id).encode();
        let user_model = user.into();
        Self {
            node: user_model,
            cursor,
        }
    }
}

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct UserConnection {
    // A list of edges.
    pub edges: Vec<UserEdge>,
    //
    // helper
    //
    #[graphql(skip)]
    pub after: Option<String>,
    #[graphql(skip)]
    pub before: Option<String>,
    #[graphql(skip)]
    pub first: Option<i32>,
    #[graphql(skip)]
    pub last: Option<i32>,
}

#[derive(Debug, SimpleObject)]
pub struct PageInfo {
    // When paginating forwards, the cursor to continue.
    pub end_cursor: Option<String>,
    // When paginating forwards, are there more items?
    pub has_next_page: bool,
    // When paginating backwards, the cursor to continue.
    pub start_cursor: Option<String>,
    // When paginating backwards, are there more items?
    pub has_previous_page: bool,
}

impl From<entities::PageInfo> for PageInfo {
    fn from(page_info: entities::PageInfo) -> Self {
        Self {
            has_next_page: page_info.has_next_page,
            has_previous_page: page_info.has_previous_page,
            start_cursor: page_info.start_cursor,
            end_cursor: page_info.end_cursor,
        }
    }
}

#[ComplexObject]
impl UserConnection {
    // Information to aid in pagination.
    async fn page_info(&self, ctx: &Context<'_>) -> Result<PageInfo> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;
        let page_info = server_ctx
            .user_service
            .find_page_info(
                self.first,
                self.after.clone(),
                self.last,
                self.before.clone(),
            )
            .await?;
        Ok(page_info.into())
    }
    // Identifies the total count of items in the connection.
    async fn total_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;
        let db = &server_ctx.user_service.db;

        let total_count_query = "select count(*) as exact_count from  user_";
        match sqlx::query(total_count_query).fetch_one(db).await {
            Err(err) => {
                tracing::error!("{}", &err);
                Err(err.into())
            }
            Ok(row) => Ok(row.get(0)),
        }
    }
}
