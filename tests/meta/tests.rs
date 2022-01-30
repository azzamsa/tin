use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use cynic::QueryBuilder;
use nahla::db;
use nahla::routes::graphql_handler;
use nahla::{Mutation, Query};
use poem::{test::TestClient, Route};

use super::graphql::queries::MetaQuery;
use super::schema::MetaResponse;

#[tokio::test]
async fn meta() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    let query = MetaQuery::build(());
    let resp = client.post("/").data(schema).body_json(&query).send().await;

    resp.assert_status_is_ok();

    let resp_str = resp.into_body().into_string().await?;
    let meta_response: MetaResponse = serde_json::from_str(&resp_str)?;

    let cargo_package_version = env!("CARGO_PKG_VERSION").to_string();
    assert_eq!(meta_response.data.meta.version, cargo_package_version);

    Ok(())
}
