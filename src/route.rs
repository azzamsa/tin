use std::{fs, sync::Arc};

use async_graphql::{
    EmptySubscription, Schema,
    http::{GraphQLPlaygroundConfig, playground_source},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    response::{self, IntoResponse},
    routing::{get, post},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    Error, config,
    config::Config,
    context::ServerContext,
    db,
    domain::{health, meta, user},
    driver::mailer::Mailer,
    route,
    schema::{AppSchema, Mutation, Query},
};

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
pub async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn app() -> Result<Router, Error> {
    let config = Arc::new(Config::load()?);

    let db = db::connect(&config.database).await?;
    db::migrate(&db).await?;

    let health_service = Arc::new(health::Service::new());
    let meta_service = Arc::new(meta::Service::new(Arc::clone(&config)));
    let mailer_service = Mailer::new();
    let user_service = Arc::new(user::Service::new(db, mailer_service));

    let server_context = Arc::new(ServerContext {
        user_service,
        meta_service,
        health_service,
    });

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(Arc::clone(&server_context))
        .finish();

    // Export schema to file
    if let Some(location) = &config.schema_location {
        fs::write(location, schema.sdl()).map_err(|_| {
            Error::InvalidArgument(format!(
                "GraphQL schema location doesn't exists `{}`",
                &location
            ))
        })?;
        tracing::info!("Wrote GraphQL schema to {}", location);
    }

    #[derive(OpenApi)]
    #[openapi(
        paths(
            health::resolver::health,
        ),
        components(schemas(health::model::Health, health::model::HealthResponse)),
        tags(
            (name = "Rust GraphQL", description = "Rust GraphQL Boilerplate 🏗️")
        )
    )]
    struct ApiDoc;

    let mut app = Router::new()
        .route("/graphql", post(route::graphql_handler))
        .route("/health", get(health::resolver::health));
    if config.env != config::Env::Production {
        app = app
            .route("/playground", get(route::graphql_playground))
            .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));
    }
    let app = app.with_state(schema);

    Ok(app)
}
