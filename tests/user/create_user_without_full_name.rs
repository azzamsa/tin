use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use http_body_util::BodyExt;
use serde_json as json;
use tin::route::app;
use tower::util::ServiceExt;

use super::graphql::{mutations, queries};
use super::teardown;

#[tokio::test]
async fn create_user_without_full_name() -> Result<()> {
    let app = app().await?;

    let args = mutations::CreateUserInput {
        name: "aragorn".to_string(),
        email: "aragorn@mail.com".to_string(),
        full_name: None,
    };
    let query = mutations::CreateUser::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let response: json::Value = json::from_slice(&body)?;
    let response: queries::User = json::from_value(response["data"]["createUser"].clone())?;
    assert_eq!(response.name, "aragorn");
    assert_eq!(response.full_name, None);

    teardown().await?;
    Ok(())
}
