use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::QueryBuilder;
use graph::routes::app;
use serde_json::{from_slice, to_string};
use tower::util::ServiceExt;

use super::{graphql::queries::HealthQuery, schema::HealthResponse};

#[tokio::test]
async fn health() -> Result<()> {
    let app = app().await?;

    let query = HealthQuery::build(());
    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let health_response: HealthResponse = from_slice(&resp_byte)?;
    assert_eq!(health_response.data.health.status, "running");

    Ok(())
}
