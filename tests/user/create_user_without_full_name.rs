use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use http_body_util::BodyExt;
use serde_json::{from_slice, to_string};
use tin::route::app;
use tower::util::ServiceExt;

use super::teardown;
use super::{graphql::add, schema::CreateUserResponse};

#[tokio::test]
async fn create_user_without_full_name() -> Result<()> {
    let app = app().await?;

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        email: "khawa@email.com".to_string(),
        full_name: None,
    };
    let query = add::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let user_response: CreateUserResponse = from_slice(&body)?;
    assert_eq!(user_response.data.create_user.name, "khawa");
    assert_eq!(user_response.data.create_user.full_name, None);

    teardown().await?;
    Ok(())
}
