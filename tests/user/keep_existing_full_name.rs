use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use http_body_util::BodyExt;
use serde_json as json;
use tin::route::app;
use tower::{Service, util::ServiceExt};

use super::graphql::{mutations, queries};
use super::teardown;

#[tokio::test]
async fn keep_existing_full_name() -> Result<()> {
    let mut app = app().await?;
    //
    // Create User With a Full name
    //
    let args = mutations::CreateUserInput {
        name: "gandalf".to_string(),
        email: "gandalf@mail.com".to_string(),
        full_name: Some("Gandalf The Gray".to_string()),
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
    assert_eq!(response.name, "gandalf");

    //
    // Update Only the user name
    //

    let user_id = response.id;
    let args = mutations::UpdateUserInput {
        id: user_id,
        // Mithrandir is the name given to Gandalf by the Elves
        // and it translates to "Gray Pilgrim".
        name: "mithrandir".to_string(),
        email: "gandalf@mail.com".to_string(),
        full_name: None,
    };
    let query = mutations::UpdateUser::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;

    //
    // Make sure the full name is intact
    //

    let body = response.into_body().collect().await?.to_bytes();
    let response: json::Value = json::from_slice(&body)?;
    let response: queries::User = json::from_value(response["data"]["updateUser"].clone())?;
    assert_eq!(response.name, "mithrandir");
    assert_eq!(response.full_name, Some("Gandalf The Gray".to_string()));

    teardown().await?;
    Ok(())
}
