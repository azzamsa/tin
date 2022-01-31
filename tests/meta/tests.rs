use std::sync::Arc;

use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use cynic::QueryBuilder;
use nahla::config::Config;
use nahla::context::ServerContext;
use nahla::routes::graphql_handler;
use nahla::schema::{Mutation, Query};
use nahla::{db, health, meta, user};
use poem::{test::TestClient, Route};
use serde_json::from_str;

use super::graphql::queries::MetaQuery;
use super::schema::MetaResponse;

#[tokio::test]
async fn meta() -> Result<()> {
    // Setup app
    let config = Arc::new(Config::load()?);
    let db = db::connect(&config.database).await?;

    let user_service = Arc::new(user::Service::new(db.clone()));
    let meta_service = Arc::new(meta::Service::new());
    let health_service = Arc::new(health::Service::new());

    let server_context = Arc::new(ServerContext {
        user_service,
        meta_service,
        health_service,
    });

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(Arc::clone(&server_context))
        .finish();

    // Test
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let query = MetaQuery::build(());
    let resp = client.post("/").data(schema).body_json(&query).send().await;

    resp.assert_status_is_ok();

    let resp_str = resp.into_body().into_string().await?;
    let meta_response: MetaResponse = from_str(&resp_str)?;

    let cargo_package_version = env!("CARGO_PKG_VERSION").to_string();
    assert_eq!(meta_response.data.meta.version, cargo_package_version);

    Ok(())
}
