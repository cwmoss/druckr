use super::AppState;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use std::{env, path::PathBuf};
use typst_bake::{IntoDict, IntoValue};

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

    async fn letter(&self, from: String, to: String, msg: String) -> String {
        // format!("ein brief von {} nach {}", from, to);
        print_letter(LetterInput { from, to, msg })
    }
}

pub fn api_routes() -> Router<Arc<AppState>> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)))
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[derive(IntoValue, IntoDict)]
struct LetterInput {
    from: String,
    to: String,
    msg: String,
}

fn print_letter(input: LetterInput) -> String {
    let pdf = typst_bake::document!("letter.typ")
        .with_inputs(input)
        .to_pdf()
        .unwrap();
    let fname = format!("{}.pdf", gen_rand(8));
    save_pdf(&pdf, &fname);
    // app_state.printed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    fname
}

fn save_pdf(data: &[u8], filename: &str) {
    let out_dir = PathBuf::from(env::current_dir().unwrap().to_string_lossy().to_string());
    // std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::write(out_dir.join(filename), data).unwrap();
}

fn gen_rand(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    rand::rng().fill_bytes(&mut buf);
    let b64 = general_purpose::STANDARD.encode(&buf);
    b64.trim_end_matches('=')
        .replace('+', "-")
        .replace('/', "_")
}
