pub mod input;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::user::{
    entities,
    scalar::{Id, Time},
    service,
};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub id: Id,
    pub created_at: Time,

    pub name: String,
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

#[derive(Debug, Clone, SimpleObject)]
pub struct UserEdge {
    pub node: User,
    pub cursor: Id,
}

impl From<entities::User> for UserEdge {
    fn from(user: entities::User) -> Self {
        let user_id = user.id;
        let user_model = user.into();

        Self {
            node: user_model,
            cursor: user_id,
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct UserConnection {
    pub edges: Vec<UserEdge>,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct PageInfo {
    // When paginating forwards, the cursor to continue.
    pub end_cursor: Option<Id>,
    // When paginating forwards, are there more items?
    pub has_next_page: bool,
    // When paginating backwards, the cursor to continue.
    pub start_cursor: Option<Id>,
    // When paginating backwards, are there more items?
    pub has_previous_page: bool,
}

impl From<service::PageInfo> for PageInfo {
    fn from(page_info: service::PageInfo) -> Self {
        Self {
            has_next_page: page_info.has_next_page,
            has_previous_page: page_info.has_previous_page,
            start_cursor: page_info.start_cursor,
            end_cursor: page_info.end_cursor,
        }
    }
}
