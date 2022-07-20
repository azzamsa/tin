use std::sync::Arc;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
};

use crate::{
    config,
    config::Config,
    context::ServerContext,
    db, health, meta, routes,
    schema::{AppSchema, Mutation, Query},
    user, Error,
};

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
pub async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn app() -> Result<Router, Error> {
    let config = Arc::new(Config::load()?);

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

    let mut app = Router::new()
        .route("/graphql", post(routes::graphql_handler))
        .route("/health", get(health::resolver::health));
    if config.env != config::Env::Production {
        app = app.route("/", get(routes::graphql_playground));
    }
    let app = app.layer(Extension(schema));

    Ok(app)
}
