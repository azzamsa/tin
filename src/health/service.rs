use async_graphql::FieldResult;

use super::schema::Health;

pub async fn read() -> FieldResult<Health> {
    let health = Health {
        status: "running".to_string(),
    };
    Ok(health)
}
