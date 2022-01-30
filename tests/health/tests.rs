use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use cynic::QueryBuilder;
use nahla::db;
use nahla::routes::graphql_handler;
use nahla::{Mutation, Query};
use poem::{test::TestClient, Route};

use super::graphql::queries::HealthQuery;
use super::schema::HealthResponse;

#[tokio::test]
async fn health() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    let query = HealthQuery::build(());
    let resp = client.post("/").data(schema).body_json(&query).send().await;

    let resp_str = resp.into_body().into_string().await?;
    let health_response: HealthResponse = serde_json::from_str(&resp_str)?;
    assert_eq!(health_response.data.health.status, "running");

    Ok(())
}
