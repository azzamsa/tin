use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::{MutationBuilder, QueryBuilder};
use serde_json::{from_slice, to_string, Value};
use tin::routes::app;
use tower::{util::ServiceExt, Service};

use super::teardown;
use super::{
    graphql::{
        add, delete, queries,
        queries::{ReadUserArguments, UserQuery},
    },
    schema::CreateUserResponse,
};

#[tokio::test]
async fn delete_user() -> Result<()> {
    let mut app = app().await?;

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.ready().await?.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let user_response: CreateUserResponse = from_slice(&resp_byte)?;
    assert_eq!(user_response.data.create_user.name, "khawa");

    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //

    let user_id_str = delete::Uuid(user_id.to_string());
    let args = delete::DeleteUserArguments { id: user_id_str };
    let query = delete::UserMutation::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let _ = app.ready().await?.call(request).await?;
    //
    // Make sure user deleted
    //
    let args = ReadUserArguments {
        id: queries::Uuid(user_id.to_string()),
    };
    let query = UserQuery::build(args);

    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.ready().await?.call(request).await?;
    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = from_slice(&resp_byte)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    teardown().await?;
    Ok(())
}
