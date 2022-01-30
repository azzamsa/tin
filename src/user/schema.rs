use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, SimpleObject)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct UpdateUserInput {
    /// The ID of the User to modify.
    pub id: i32,
    /// The name for the User.
    pub name: String,
    /// The full name for the User.
    pub full_name: Option<String>,
}
