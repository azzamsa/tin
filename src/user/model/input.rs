use async_graphql::InputObject;
use uuid::Uuid;

use crate::user::scalar::Id;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub id: Uuid,
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(InputObject)]
pub struct DeleteUserInput {
    pub user_id: Id,
}
