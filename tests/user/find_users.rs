use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::QueryBuilder;
use graph::routes::app;
use serde_json::to_string;
use tower::util::ServiceExt;

use super::graphql::queries::UsersQuery;

#[tokio::test]
async fn find_users() -> Result<()> {
    let app = app().await?;

    let query = UsersQuery::build(());
    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
