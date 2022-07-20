use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::MutationBuilder;
use graph::routes::app;
use serde_json::{from_slice, to_string};
use tower::{util::ServiceExt, Service};

use super::{
    graphql::{add, update},
    schema::{CreateUserResponse, UpdateUserResponse},
};
use crate::user::{graphql::update::Uuid, teardown};

#[tokio::test]
async fn update_user() -> Result<()> {
    let mut router = app().await?;
    let app = router.ready().await?;
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

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let user_response: CreateUserResponse = from_slice(&resp_byte)?;
    assert_eq!(user_response.data.create_user.name, "khawa");

    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //
    let user_id = Uuid(user_id.to_string());

    let args = update::UpdateUserInput {
        id: user_id,
        name: "haitham".to_string(),
        full_name: None,
    };
    let query = update::UserMutation::build(&args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.call(request).await?;
    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let user_response: UpdateUserResponse = from_slice(&resp_byte)?;

    assert_eq!(user_response.data.update_user.name, "haitham");

    teardown().await?;
    Ok(())
}
