use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request},
};
use cynic::MutationBuilder;
use graph::routes::app;
use serde_json::{from_slice, to_string};
use tower::util::ServiceExt;

use super::{
    graphql::{add, update},
    schema::{CreateUserResponse, UpdateUserResponse},
};
use crate::user::teardown;

#[tokio::test]
async fn keep_existing_full_name() -> Result<()> {
    let app = app().await?;
    //
    // Create User
    //

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

    let response = app.clone().oneshot(request).await?;
    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let user_response: CreateUserResponse = from_slice(&resp_byte)?;
    let user_id = user_response.data.create_user.id;
    //
    // Update Only the user name
    //
    let args = update::UpdateUserInput {
        id: update::Uuid(user_id.to_string()),
        name: "khawa1".to_string(),
        full_name: None,
    };
    let query = update::UserMutation::build(&args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.clone().oneshot(request).await?;
    //
    // Make sure the full name preserved
    //
    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let user_response: UpdateUserResponse = from_slice(&resp_byte)?;
    assert_eq!(user_response.data.update_user.name, "khawa1");
    assert_eq!(
        user_response.data.update_user.full_name,
        Some("Abu Musa Al-Khawarizmi".to_string())
    );

    teardown().await?;
    Ok(())
}
