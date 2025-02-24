use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::QueryBuilder;
use http_body_util::BodyExt;
use serde_json as json;
use tin::route::app;
use tower::util::ServiceExt;

use super::graphql::queries;

#[tokio::test]
async fn find_user() -> Result<()> {
    let app = app().await?;

    let args = queries::ReadUserArguments {
        id: queries::Uuid("00000000-0000-0000-0000-000000000000".to_string()),
    };
    let query = queries::UserQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: json::Value = json::from_slice(&body)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    Ok(())
}
