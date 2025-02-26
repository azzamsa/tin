use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use http_body_util::BodyExt;
use serde_json as json;
use tin::route::app;
use tower::{Service, util::ServiceExt};

use super::teardown;
use crate::graphql::{mutations, queries};

#[tokio::test]
async fn duplicate_username_create() -> Result<()> {
    let mut app = app().await?;
    //
    // Create User
    //
    let args = mutations::CreateUserInput {
        name: "bilbo".to_string(),
        email: "bilbo@mail.com".to_string(),
        full_name: None,
    };
    let query = mutations::CreateUser::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let _ = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;

    //
    // Create next user with the same name
    //

    let args = mutations::CreateUserInput {
        name: "bilbo".to_string(),
        email: "bilbo@mail.com".to_string(),
        full_name: None,
    };
    let query = mutations::CreateUser::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    let body = response.into_body().collect().await?.to_bytes();
    let body: json::Value = json::from_slice(&body)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "username is already in use");

    teardown().await?;
    Ok(())
}

// Update user's email to to one already in use by another user.
#[tokio::test]
async fn duplicate_username_update() -> Result<()> {
    let mut app = app().await?;
    //
    // Create User
    //

    let args = mutations::CreateUserInput {
        name: "Pippin".to_string(),
        email: "pippin@mail.com".to_string(),
        full_name: None,
    };
    let query = mutations::CreateUser::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    //
    // Create other user
    //

    let args = mutations::CreateUserInput {
        name: "Merry".to_string(),
        email: "merry@mail.com".to_string(),
        full_name: None,
    };
    let query = mutations::CreateUser::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let response: json::Value = json::from_slice(&body)?;
    let response: queries::User = json::from_value(response["data"]["createUser"].clone())?;
    let user_id = response.id;

    //
    // Update second user to the same name as first user
    //
    let args = mutations::UpdateUserInput {
        id: user_id,
        name: "Merry".to_string(),
        email: "pippin@email.com".to_string(),
        full_name: None,
    };
    let query = mutations::UpdateUser::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    let body = response.into_body().collect().await?.to_bytes();
    let body: json::Value = json::from_slice(&body)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "username is already in use");

    teardown().await?;
    Ok(())
}
