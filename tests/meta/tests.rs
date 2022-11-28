use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::QueryBuilder;
use serde_json::{from_slice, to_string};
use tin::routes::app;
use tower::util::ServiceExt;

use super::{graphql::queries::MetaQuery, schema::MetaResponse};

#[tokio::test]
async fn meta() -> Result<()> {
    let app = app().await?;

    let query = MetaQuery::build(());
    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let meta_response: MetaResponse = from_slice(&resp_byte)?;

    let cargo_package_version = env!("CARGO_PKG_VERSION").to_string();
    assert_eq!(meta_response.data.meta.version, cargo_package_version);

    Ok(())
}
