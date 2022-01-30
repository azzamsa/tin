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

use anyhow::{Context, Result};
use async_graphql::{EmptySubscription, Schema};
use cynic::{MutationBuilder, QueryBuilder};
use nahla::db;
use nahla::routes::graphql_handler;
use nahla::{Mutation, Query};
use poem::{test::TestClient, Route};
use serde_json::{from_str, Value};

use super::graphql::queries::{ReadUserArguments, UserQuery, UsersQuery};
use super::graphql::{add, delete, update};
use super::schema::{CreateUserResponse, UpdateUserResponse};

#[tokio::test]
async fn read_users() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    let query = UsersQuery::build(());
    let resp = client.post("/").data(schema).body_json(&query).send().await;

    resp.assert_status_is_ok();

    Ok(())
}

#[tokio::test]
async fn read_user() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    let args = ReadUserArguments { id: 100 };
    let query = UserQuery::build(args);
    let resp = client.post("/").data(schema).body_json(&query).send().await;

    resp.assert_status_is_ok();

    let resp_str = resp.into_body().into_string().await?;
    let body: Value = from_str(&resp_str).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    Ok(())
}

#[tokio::test]
async fn create_user() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    let args = add::CreateUserInput {
        name: "khawa-create".to_string(),
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
    let user_response: CreateUserResponse = serde_json::from_str(&resp_str)?;

    assert_eq!(user_response.data.create_user.name, "khawa-create");

    Ok(())
}

#[tokio::test]
async fn create_user_without_full_name() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    let args = add::CreateUserInput {
        name: "khawa-create-no-full-name".to_string(),
        full_name: None,
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
    let user_response: CreateUserResponse = serde_json::from_str(&resp_str)?;

    assert_eq!(
        user_response.data.create_user.name,
        "khawa-create-no-full-name"
    );
    assert_eq!(user_response.data.create_user.full_name, None);

    Ok(())
}

#[tokio::test]
async fn duplicate_username() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-duplicate".to_string(),
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

    //
    // Create next user with the same name
    //

    let args = add::CreateUserInput {
        name: "khawa-duplicate".to_string(),
        full_name: None,
    };
    let query = add::UserMutation::build(&args);

    let resp = client
        .post("/")
        .data(schema.clone())
        .body_json(&query)
        .send()
        .await;
    let resp_str = resp.into_body().into_string().await?;

    let body: Value = from_str(&resp_str).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "a user with same `name` already exists");

    Ok(())
}

#[tokio::test]
async fn update_user() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-update".to_string(),
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
    let user_response: CreateUserResponse = serde_json::from_str(&resp_str)?;
    assert_eq!(user_response.data.create_user.name, "khawa-update");
    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //

    let args = update::UpdateUserInput {
        id: user_id,
        name: "haitham".to_string(),
        full_name: None,
    };
    let query = update::UserMutation::build(&args);

    let resp = client
        .post("/")
        .data(schema.clone())
        .body_json(&query)
        .send()
        .await;
    let resp_str = resp.into_body().into_string().await?;
    let user_response: UpdateUserResponse = serde_json::from_str(&resp_str)?;

    assert_eq!(user_response.data.update_user.name, "haitham");

    Ok(())
}

#[tokio::test]
async fn delete_user() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

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
    let user_response: CreateUserResponse = serde_json::from_str(&resp_str)?;

    assert_eq!(user_response.data.create_user.name, "khawa-delete");
    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //

    let args = delete::DeleteUserArguments { id: user_id };
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

    let args = ReadUserArguments { id: user_id };
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

#[tokio::test]
async fn keep_existing_full_name() -> Result<()> {
    let app = Route::new().at("/", graphql_handler);
    let client = TestClient::new(app);

    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

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
    let user_response: CreateUserResponse = serde_json::from_str(&resp_str)?;

    assert_eq!(user_response.data.create_user.name, "khawa-keep");
    let user_id = user_response.data.create_user.id;

    //
    // Update Only the user name
    //

    let args = update::UpdateUserInput {
        id: user_id,
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
    let user_response: UpdateUserResponse = serde_json::from_str(&resp_str)?;

    assert_eq!(user_response.data.update_user.name, "khawa-keep-2");
    assert_eq!(
        user_response.data.update_user.full_name,
        Some("Abu Musa Al-Khawarizmi".to_string())
    );

    Ok(())
}
