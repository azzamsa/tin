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
async fn no_first_no_last() -> Result<()> {
    let mut app = app().await?;
    create_users().await?;

    let args = queries::ReadUsersArguments {
        first: None,
        after: None,
        last: None,
        before: None,
    };
    let query = queries::UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: json::Value = json::from_slice(&body)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(
        error_message,
        "You must provide a `first` or `last` value to properly paginate the entity."
    );
    Ok(())
}

#[tokio::test]
async fn both_first_and_last() -> Result<()> {
    let mut app = app().await?;
    create_users().await?;

    let args = queries::ReadUsersArguments {
        first: Some(1),
        after: None,
        last: Some(1),
        before: None,
    };
    let query = queries::UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: json::Value = json::from_slice(&body)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(
        error_message,
        "Passing both `first` and `last` for pagination is not supported."
    );
    Ok(())
}

#[tokio::test]
async fn invalid_cursor() -> Result<()> {
    let mut app = app().await?;
    create_users().await?;

    let args = queries::ReadUsersArguments {
        first: Some(1),
        after: Some("invalid_cursor".to_string()),
        last: None,
        before: None,
    };
    let query = queries::UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await?
        .call(request)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: json::Value = json::from_slice(&body)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "Invalid cursor");
    Ok(())
}

async fn create_users() -> Result<()> {
    let mut app = app().await?;

    let names = ["one", "two", "three", "four", "five", "six"];
    for name in names {
        let args = mutations::CreateUserInput {
            name: name.to_string(),
            email: format!("{name}@mail.com"),
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
    }
    Ok(())
}

#[tokio::test]
async fn find_paginated_user() -> Result<()> {
    let mut app = app().await?;
    create_users().await?;

    let args = queries::ReadUsersArguments {
        first: Some(1),
        after: None,
        last: None,
        before: None,
    };
    let query = queries::UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let response: json::Value = json::from_slice(&body)?;
    let response: queries::UsersQuery = json::from_value(response["data"].clone())?;
    assert_eq!(response.users.total_count, 6);

    //
    // first edges
    //

    assert_eq!(response.users.edges.len(), 1);
    assert_eq!(response.users.edges[0].node.name, "one");

    let one_cursor = &response.users.edges[0].cursor;

    //
    // after
    //

    let args = queries::ReadUsersArguments {
        first: Some(1),
        after: Some(one_cursor.to_string()),
        last: None,
        before: None,
    };
    let query = queries::UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let response: json::Value = json::from_slice(&body)?;
    let response: queries::UsersQuery = json::from_value(response["data"].clone())?;
    assert_eq!(response.users.edges[0].node.name, "two");

    //
    // before
    //

    let two_cursor = &response.users.edges[0].cursor;
    let args = queries::ReadUsersArguments {
        first: Some(1),
        after: None,
        last: None,
        before: Some(two_cursor.to_string()),
    };
    let query = queries::UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(json::to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let response: json::Value = json::from_slice(&body)?;
    let response: queries::UsersQuery = json::from_value(response["data"].clone())?;
    assert_eq!(response.users.edges[0].node.name, "one");

    teardown().await?;
    Ok(())
}
