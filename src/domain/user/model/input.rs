use async_graphql::InputObject;

use crate::scalar::Id;

#[derive(InputObject)]
pub struct CreateUserInput {
    /// The name for the User.
    pub name: String,
    /// The full name for the User.
    pub full_name: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    /// The ID of the User to modify.
    pub id: Id,
    /// The name for the User.
    pub name: String,
    /// The full name for the User.
    pub full_name: Option<String>,
}

#[derive(InputObject)]
pub struct DeleteUserInput {
    pub user_id: Id,
}
