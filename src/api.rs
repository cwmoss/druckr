use super::AppState;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;

use axum::{
    Router,
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, FromRequest, Multipart, Path, Request, State},
    http::{HeaderValue, StatusCode, header},
    middleware,
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    serve::Listener,
};

use std::sync::Arc;

struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

pub fn api_routes() -> Router<Arc<AppState>> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)))
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
