use super::AppState;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;

use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
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
