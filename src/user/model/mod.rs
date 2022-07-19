pub mod input;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::user::{
    entities,
    scalar::{Id, Time},
};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub id: Option<Id>,
    pub created_at: Option<Time>,

    pub name: String,
    pub full_name: Option<String>,
}

impl From<entities::User> for User {
    fn from(user: entities::User) -> Self {
        Self {
            id: Some(user.id),
            created_at: Some(user.created_at),

            name: user.name,
            full_name: user.full_name,
        }
    }
}
