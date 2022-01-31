use std::sync::Arc;

use anyhow::{Context, Result};
use async_graphql::{EmptySubscription, Schema};
use cynic::{MutationBuilder, QueryBuilder};
use nahla::config::Config;
use nahla::context::ServerContext;
use nahla::routes::graphql_handler;
use nahla::schema::{Mutation, Query};
use nahla::{db, health, meta, user};
use poem::{test::TestClient, Route};
use serde_json::{from_str, Value};

use super::graphql::queries;
use super::graphql::queries::{ReadUserArguments, UserQuery};
use super::graphql::{add, delete};
use super::schema::CreateUserResponse;

#[tokio::test]
async fn delete_user() -> Result<()> {
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
        name: "khawa-delete".to_string(),
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

    assert_eq!(user_response.data.create_user.name, "khawa-delete");
    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //

    let user_id_str = delete::Uuid(user_id.to_string());
    let args = delete::DeleteUserArguments { id: user_id_str };
    let query = delete::UserMutation::build(&args);
    let _resp = client
        .post("/")
        .data(schema.clone())
        .body_json(&query)
        .send()
        .await;

    //
    // Make sure user deleted
    //
    let args = ReadUserArguments {
        id: queries::Uuid(user_id.to_string()),
    };
    let query = UserQuery::build(args);

    let resp = client
        .post("/")
        .data(schema.clone())
        .body_json(&query)
        .send()
        .await;
    resp.assert_status_is_ok();

    let resp_str = resp.into_body().into_string().await?;

    let body: Value = from_str(&resp_str).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    Ok(())
}
