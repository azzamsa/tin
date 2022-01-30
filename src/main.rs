use std::env;

use async_graphql::{EmptySubscription, Schema};
use nahla::db;
use nahla::logger;
use nahla::routes;
use nahla::{Mutation, Query};
use poem::{get, listener::TcpListener, post, EndpointExt, Route, Server};

#[tokio::main]
async fn main() {
    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    let app = Route::new()
        .at("/", get(routes::graphql_playground))
        .at("/graphql", post(routes::graphql_handler))
        .data(schema);

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let address = format!("{}:{}", host, port);

    logger::init();

    log::info!("App started at `{}`", address);
    Server::new(TcpListener::bind(address))
        .run(app)
        .await
        .unwrap();
}
