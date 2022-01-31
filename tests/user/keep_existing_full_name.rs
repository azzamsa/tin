use std::sync::Arc;

use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use cynic::MutationBuilder;
use nahla::config::Config;
use nahla::context::ServerContext;
use nahla::routes::graphql_handler;
use nahla::schema::{Mutation, Query};
use nahla::{db, health, meta, user};
use poem::{test::TestClient, Route};
use serde_json::from_str;

use super::graphql::{add, update};
use super::schema::{CreateUserResponse, UpdateUserResponse};

#[tokio::test]
async fn keep_existing_full_name() -> Result<()> {
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

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-keep".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

    let resp = client
        .post("/")
        .data(schema.clone())
        .body_json(&query)
        .send()
        .await;
    resp.assert_status_is_ok();

    let resp_str = resp.into_body().into_string().await?;
    let user_response: CreateUserResponse = from_str(&resp_str)?;

    assert_eq!(user_response.data.create_user.name, "khawa-keep");
    let user_id = user_response.data.create_user.id;

    //
    // Update Only the user name
    //

    let args = update::UpdateUserInput {
        id: update::Uuid(user_id.to_string()),
        name: "khawa-keep-2".to_string(),
        full_name: None,
    };
    let query = update::UserMutation::build(&args);

    let resp = client
        .post("/")
        .data(schema.clone())
        .body_json(&query)
        .send()
        .await;
    //
    // Make sure the full name preserved
    //

    let resp_str = resp.into_body().into_string().await?;
    let user_response: UpdateUserResponse = from_str(&resp_str)?;

    assert_eq!(user_response.data.update_user.name, "khawa-keep-2");
    assert_eq!(
        user_response.data.update_user.full_name,
        Some("Abu Musa Al-Khawarizmi".to_string())
    );

    Ok(())
}
