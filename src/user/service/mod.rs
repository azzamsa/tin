mod check_username_exists;
mod create_user;
mod delete_user;
mod find_user;
mod find_users;
mod update_user;

use uuid::Uuid;

use crate::{db::DB, user::repository::Repository};

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    pub db: DB,
}

impl Service {
    pub fn new(db: DB) -> Self {
        let repo = Repository::new();
        Self { db, repo }
    }
}

#[derive(Debug)]
pub struct CreateUserInput {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug)]
pub struct UpdateUserInput {
    pub id: Uuid,
    pub name: String,
    pub full_name: Option<String>,
}
