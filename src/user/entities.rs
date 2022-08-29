use chrono;
use sqlx;

use crate::relay::Base64Cursor;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug)]
pub struct PageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
    pub start_cursor: Option<String>,
    pub has_previous_page: bool,
}

#[derive(Debug)]
pub struct UserEdge {
    pub node: User,
    pub cursor: String,
}

impl From<User> for UserEdge {
    fn from(user: User) -> Self {
        let cursor = Base64Cursor::new(user.id).encode();
        Self { node: user, cursor }
    }
}
