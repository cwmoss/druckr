pub mod api;
pub mod config;

use std::time::Instant;
use std::{collections::HashMap, path::PathBuf};

use crate::config::Config;
use axum::{
    Router,
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, FromRequest, Multipart, Path, Request, State},
    http::{HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    serve::Listener,
};
use rust_embed::Embed;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::{env, fs};

#[derive(Clone, Debug)]
pub struct AppState {
    pub conf: Config,
    // pub datasets: HashMap<String, Dataset>,
    // pub system: Dataset,
    pub start_time: Instant,
    pub printed: Arc<AtomicU64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = Config::new();
    let open_browser = true;
    let bind_host = "127.0.0.1";
    let port = 11011;
    // let http_prefix = format!("{}/", args.prefix.trim_end_matches('/'));
    let http_prefix = "/";
    let hostport = format!("{}:{}", bind_host, port).to_string();
    conf.setup();
    let app_state = AppState {
        // system: conf.system_dataset().await,
        conf,
        // datasets: HashMap::new(),
        printed: Arc::new(AtomicU64::new(0)),
        start_time: Instant::now(),
    };
    let state = Arc::new(app_state);

    // let data_router = dataset_handler::get_routes();
    let api_routes = api::api_routes(state.clone());

    let router = Router::new()
        // .with_state(state)
        // .nest("/data", data_router)
        // .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/download/{pdf}", get(download_pdf))
        .route("/favicon.ico", get(favicon))
        .route("/{*file}", get(static_handler))
        .route("/", get(index))
        .nest("/graphql", api_routes)
        .with_state(state)
        .fallback_service(get(not_found));

    let router = match String::from(http_prefix.clone()).as_str() {
        "/" | "" => router,
        http_prefix => Router::new().nest(&http_prefix, router),
    };

    // start the server
    let listener = tokio::net::TcpListener::bind(hostport.clone()).await;
    let listener = match listener {
        Ok(l) => l,
        Err(msg) => {
            println!("unable to bind. trying different port ({})", msg);
            let hostport = format!("{}:0", bind_host);
            tokio::net::TcpListener::bind(hostport.clone())
                .await
                .unwrap()
        }
    };

    println!(
        "Listening on http://{:?}{}",
        listener.local_addr().ok().unwrap(),
        http_prefix
    );

    if open_browser {
        // let future = after_start(hostport.clone());
        // set_timeout_async!(future, 600);
    }
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found 😥</p>")
}

async fn download_pdf(
    State(app_state): State<Arc<AppState>>,
    Path(pdf): Path<String>,
) -> impl axum::response::IntoResponse {
    let path = PathBuf::from(env::current_dir().unwrap().to_string_lossy().to_string()).join(pdf);

    if let Some(data) = fs::read(path).ok() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/pdf"),
        );
        /*headers.insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", pdf)).unwrap(),
        );*/
        return (headers, data).into_response();
    }
    return "PDF not found".into_response();
}

#[derive(Embed)]
#[folder = "public/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

async fn index() -> impl IntoResponse {
    StaticFile("index.html")
}

async fn favicon() -> impl IntoResponse {
    StaticFile("accent.svg")
}

async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    StaticFile(path)
}
