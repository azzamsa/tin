use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Request, Response};
use poem::{
    handler,
    web::{Data, Html, Json},
    IntoResponse,
};

use crate::schema::AppSchema;

#[handler]
pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[handler]
pub async fn graphql_handler(schema: Data<&AppSchema>, req: Json<Request>) -> Json<Response> {
    Json(schema.execute(req.0).await)
}
