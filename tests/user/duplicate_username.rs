use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use serde_json::{from_slice, to_string, Value};
use tin::routes::app;
use tower::{util::ServiceExt, Service};

use super::{graphql::add, schema::CreateUserResponse};
use super::{graphql::update, teardown};

#[tokio::test]
async fn duplicate_username_create() -> Result<()> {
    let mut app = app().await?;
    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let _ = app.ready().await?.call(request).await?;

    //
    // Create next user with the same name
    //

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: None,
    };
    let query = add::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.ready().await?.call(request).await?;
    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = from_slice(&resp_byte)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "username is already in use");

    teardown().await?;
    Ok(())
}

#[tokio::test]
async fn duplicate_username_update() -> Result<()> {
    let mut router = app().await?;
    let app = router.ready().await?;
    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.ready().await?.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    //
    // Create second user
    //

    let args = add::CreateUserInput {
        name: "khawa1".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.ready().await?.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let user_response: CreateUserResponse = from_slice(&resp_byte)?;
    let user_id = user_response.data.create_user.id;

    //
    // Update second user to the same name as first user
    //

    let user_id = update::Uuid(user_id.to_string());
    let args = update::UpdateUserInput {
        id: user_id,
        name: "khawa".to_string(),
        full_name: None,
    };
    let query = update::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.ready().await?.call(request).await?;
    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = from_slice(&resp_byte)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "username is already in use");

    teardown().await?;
    Ok(())
}
