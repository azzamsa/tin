use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use nahla::config::Config;
use nahla::context::ServerContext;
use nahla::schema::{Mutation, Query};
use nahla::{db, health, logger, meta, routes, user};
use poem::{get, listener::TcpListener, post, EndpointExt, Route, Server};

#[tokio::main]
async fn main() -> Result<(), nahla::Error> {
    let config = Arc::new(Config::load()?);
    logger::init(&config);

    let db = db::connect(&config.database).await?;
    db::migrate(&db).await?;

    let user_service = Arc::new(user::Service::new(db.clone()));
    let meta_service = Arc::new(meta::Service::new());
    let health_service = Arc::new(health::Service::new());

    let server_context = Arc::new(ServerContext {
        user_service,
        meta_service,
        health_service,
    });

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(Arc::clone(&server_context))
        .finish();

    let app = Route::new()
        .at("/", get(routes::graphql_playground))
        .at("/graphql", post(routes::graphql_handler))
        .data(schema);

    let address = format!("{}:{}", &config.base_url, &config.http.port);

    log::info!("App started at `{}`", address);
    Server::new(TcpListener::bind(address))
        .run(app)
        .await
        .unwrap();

    Ok(())
}
