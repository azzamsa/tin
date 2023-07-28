use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use serde_json::{from_slice, to_string};
use tin::route::app;
use tower::{util::ServiceExt, Service};

use super::{fake_user, graphql::update::Uuid, teardown};
use super::{
    graphql::{add, update},
    schema::{CreateUserResponse, UpdateUserResponse},
};

#[tokio::test]
async fn update_user() -> Result<()> {
    let mut app = app().await?;
    //
    // Create User
    //

    let query = add::UserMutation::build(fake_user());

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.ready().await?.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let user_response: CreateUserResponse = from_slice(&resp_byte)?;
    assert_eq!(user_response.data.create_user.name, "khawa");

    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //
    let user_id = Uuid(user_id.to_string());

    let args = update::UpdateUserInput {
        id: user_id,
        name: "haitham".to_string(),
        email: "haitam@email.com".to_string(),
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
    let user_response: UpdateUserResponse = from_slice(&resp_byte)?;

    assert_eq!(user_response.data.update_user.name, "haitham");

    teardown().await?;
    Ok(())
}
