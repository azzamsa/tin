mod check_username_exists;
mod create_user;
mod delete_user;
mod find_user;
mod find_users;
mod update_user;

use uuid::Uuid;

use crate::{
    db::DB,
    user::{model::input, repository::Repository},
};

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
}

impl Service {
    pub fn new(db: DB) -> Self {
        let repo = Repository::new();
        Self { db, repo }
    }
}

#[derive(Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug)]
pub struct CreateUserInput {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug)]
pub struct UpdateUserInput {
    /// The ID of the User to modify.
    pub id: Uuid,
    /// The name for the User.
    pub name: String,
    /// The full name for the User.
    pub full_name: Option<String>,
}

impl From<input::CreateUserInput> for CreateUserInput {
    fn from(user: input::CreateUserInput) -> Self {
        Self {
            name: user.name,
            full_name: user.full_name,
        }
    }
}

impl From<input::UpdateUserInput> for UpdateUserInput {
    fn from(user: input::UpdateUserInput) -> Self {
        Self {
            id: user.id,
            name: user.name,
            full_name: user.full_name,
        }
    }
}
