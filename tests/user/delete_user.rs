use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::{MutationBuilder, QueryBuilder};
use http_body_util::BodyExt;
use serde_json as json;
use tin::route::app;
use tower::{Service, util::ServiceExt};

use super::graphql::{mutations, queries};
use super::teardown;

#[tokio::test]
async fn delete_user() -> Result<()> {
    let mut app = app().await?;

    //
    // Create User
    //

    let args = mutations::CreateUserInput {
        // Blue Wizard's fate is ambiguous
        name: "Blue Wizard".to_string(),
        email: "blue@mail.com".to_string(),
        full_name: None,
    };
    let query = mutations::CreateUser::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let response: json::Value = json::from_slice(&body)?;
    let response: queries::User = json::from_value(response["data"]["createUser"].clone())?;
    assert_eq!(response.name, "Blue Wizard");

    //
    // Delete User
    //
    let user_id: queries::Uuid = response.id;
    let args = mutations::DeleteUserArguments {
        id: user_id.clone(),
    };
    let query = mutations::DeleteUser::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let _ = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;

    //
    // Make sure user deleted
    //

    let args = queries::ReadUserArguments { id: user_id };
    let query = queries::UserQuery::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    let body = response.into_body().collect().await?.to_bytes();
    let body: json::Value = json::from_slice(&body)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    teardown().await?;
    Ok(())
}
