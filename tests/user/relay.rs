use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cynic::{MutationBuilder, QueryBuilder};
use serde_json::{from_slice, to_string, Value};
use tin::routes::app;
use tower::{util::ServiceExt, Service};

use super::{
    graphql::{
        add,
        queries::{ReadUsersArguments, UsersQuery},
    },
    schema::UsersResponse,
};
use crate::user::teardown;

#[tokio::test]
async fn no_first_no_last() -> Result<()> {
    let mut router = app().await?;
    let app = router.ready().await?;
    create_users().await?;

    let args = ReadUsersArguments {
        first: None,
        after: None,
        last: None,
        before: None,
    };
    let query = UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = from_slice(&resp_byte)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(
        error_message,
        "You must provide a `first` or `last` value to properly paginate the entity."
    );
    Ok(())
}

#[tokio::test]
async fn both_first_and_last() -> Result<()> {
    let mut router = app().await?;
    let app = router.ready().await?;
    create_users().await?;

    let args = ReadUsersArguments {
        first: Some(1),
        after: None,
        last: Some(1),
        before: None,
    };
    let query = UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = from_slice(&resp_byte)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(
        error_message,
        "Passing both `first` and `last` for pagination is not supported."
    );
    Ok(())
}

#[tokio::test]
async fn invalid_cursor() -> Result<()> {
    let mut router = app().await?;
    let app = router.ready().await?;
    create_users().await?;

    let args = ReadUsersArguments {
        first: Some(1),
        after: Some("invalid_cursor".to_string()),
        last: None,
        before: None,
    };
    let query = UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = from_slice(&resp_byte)?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "Invalid cursor");
    Ok(())
}

async fn create_users() -> Result<()> {
    let mut router = app().await?;
    let app = router.ready().await?;

    let names = ["one", "two", "three", "four", "five", "six"];
    for name in names {
        let args = add::CreateUserInput {
            name: name.to_string(),
            full_name: None,
        };
        let query = add::UserMutation::build(args);

        let request = Request::builder()
            .method(http::Method::POST)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .uri("/graphql")
            .body(Body::from(to_string(&query)?))?;

        let response = app.call(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
    }
    Ok(())
}

#[tokio::test]
async fn find_paginated_user() -> Result<()> {
    let mut router = app().await?;
    let app = router.ready().await?;
    create_users().await?;

    let args = ReadUsersArguments {
        first: Some(1),
        after: None,
        last: None,
        before: None,
    };
    let query = UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let users_response: UsersResponse = from_slice(&resp_byte)?;
    assert_eq!(users_response.data.users.total_count, 6);
    //
    // first edges
    //
    assert_eq!(users_response.data.users.edges.len(), 1);
    assert_eq!(users_response.data.users.edges[0].node.name, "one");

    let one_cursor = users_response.data.users.edges[0].cursor.clone();
    //
    // after
    //
    let args = ReadUsersArguments {
        first: Some(1),
        after: Some(one_cursor),
        last: None,
        before: None,
    };
    let query = UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let users_response: UsersResponse = from_slice(&resp_byte)?;
    assert_eq!(users_response.data.users.edges[0].node.name, "two");

    let two_cursor = users_response.data.users.edges[0].cursor.clone();
    //
    // before
    //
    let args = ReadUsersArguments {
        first: Some(1),
        after: None,
        last: None,
        before: Some(two_cursor),
    };
    let query = UsersQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.call(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let users_response: UsersResponse = from_slice(&resp_byte)?;
    assert_eq!(users_response.data.users.edges[0].node.name, "one");

    teardown().await?;
    Ok(())
}
