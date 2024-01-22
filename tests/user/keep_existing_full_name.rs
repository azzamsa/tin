use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request},
};
use cynic::MutationBuilder;
use http_body_util::BodyExt;
use serde_json as json;
use tin::route::app;
use tower::{util::ServiceExt, Service};

use crate::fake_user;

use super::teardown;
use super::{
    graphql::{add, update},
    schema::{CreateUserResponse, UpdateUserResponse},
};

#[tokio::test]
async fn keep_existing_full_name() -> Result<()> {
    let mut app = app().await?;
    //
    // Create User
    //

    let query = add::UserMutation::build(fake_user());

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
    let user_response: CreateUserResponse = json::from_slice(&body)?;
    let user_id = user_response.data.create_user.id;
    //
    // Update Only the user name
    //
    let args = update::UpdateUserInput {
        id: update::Uuid(user_id.to_string()),
        name: "khawa1".to_string(),
        email: "khawa1@email.com".to_string(),
        full_name: None,
    };
    let query = update::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    //
    // Make sure the full name preserved
    //
    let body = response.into_body().collect().await?.to_bytes();
    let user_response: UpdateUserResponse = json::from_slice(&body)?;
    assert_eq!(user_response.data.update_user.name, "khawa1");
    assert_eq!(
        user_response.data.update_user.full_name,
        Some("Abu Musa Al-Khawarizmi".to_string())
    );

    teardown().await?;
    Ok(())
}
