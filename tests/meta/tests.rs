use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::QueryBuilder;
use http_body_util::BodyExt; // for `collect`
use serde_json as json;
use tin::route::app;
use tower::util::ServiceExt;

use super::graphql::queries::MetaQuery;

#[tokio::test]
async fn meta() -> Result<()> {
    let app = app().await?;

    let query = MetaQuery::build(());
    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let response: json::Value = json::from_slice(&body)?;
    let response: MetaQuery = json::from_value(response["data"].clone())?;

    let cargo_package_version = env!("CARGO_PKG_VERSION").to_string();
    assert_eq!(response.meta.version, cargo_package_version);

    Ok(())
}
