// After some experiment, we avoid using the setup and tear-down method
// of cleaning up the database after and before each test.
//
// Currently, Rust has no good support for something like Python `Conftest.py`.
// We have to do some acrobats to do setup-and-teardown.
//
// Most of the time, the tests are failing, because the teardown is not executed if
// the test function is panic.
//
// The best workaround we have currently is just to use different fixture for each test
// function.

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

use super::graphql::queries::UsersQuery;

#[tokio::test]
async fn find_users() -> Result<()> {
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

    let query = UsersQuery::build(());
    let resp = client.post("/").data(schema).body_json(&query).send().await;

    resp.assert_status_is_ok();

    Ok(())
}
