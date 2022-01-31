use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::scalar::Id;

#[derive(Serialize, Deserialize, InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct UpdateUserInput {
    pub id: Uuid,
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct DeleteUserInput {
    pub user_id: Id,
}
