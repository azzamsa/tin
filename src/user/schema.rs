use async_graphql::{InputObject, SimpleObject};

use crate::db::schema::user_ as user;

#[derive(Debug, SimpleObject, diesel::Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug, InputObject, diesel::Insertable)]
#[table_name = "user"]
pub struct CreateUserInput {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug, InputObject, diesel::AsChangeset)]
#[table_name = "user"]
pub struct UpdateUserInput {
    /// The ID of the User to modify.
    pub id: i32,
    /// The name for the User.
    pub name: String,
    /// The full name for the User.
    pub full_name: Option<String>,
}
