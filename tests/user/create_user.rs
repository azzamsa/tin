use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use serde_json::{from_slice, to_string};
use tin::routes::app;
use tower::util::ServiceExt;

use super::{fake_user, teardown};
use super::{graphql::add, schema::CreateUserResponse};

#[tokio::test]
async fn create_user() -> Result<()> {
    let app = app().await?;

    let query = add::UserMutation::build(fake_user());

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
