use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use graph::routes::app;
use serde_json::{from_slice, to_string};
use tower::util::ServiceExt;

use super::{graphql::add, schema::CreateUserResponse};
use crate::user::teardown;

#[tokio::test]
async fn create_user() -> Result<()> {
    let app = app().await?;

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let user_response: CreateUserResponse = from_slice(&resp_byte)?;
    assert_eq!(user_response.data.create_user.name, "khawa");

    teardown().await?;
    Ok(())
}
