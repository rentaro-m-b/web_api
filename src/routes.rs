use std::sync::Arc;

use axum::{routing::post, Extension, Router};
use crate::{presentation::{graphql::schema::{graphql_execute, new_schema, Query}, handler::hello::HelloHandlerImpl}, usecase::hello::HelloUsecaseImpl};

pub async fn new_routers() -> Router {
    let hello_usecase = Arc::new(HelloUsecaseImpl{});
    let handler = HelloHandlerImpl{ hello_usecase };
    let query = Query { hello_handler: Arc::new(handler) };
    let schema = Arc::new(new_schema(query).await);
    Router::new()
        .route("/", post(graphql_execute))
        .layer(Extension(schema.clone()))
}
