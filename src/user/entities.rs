use chrono;
use sqlx;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub name: String,
    pub full_name: Option<String>,
}
