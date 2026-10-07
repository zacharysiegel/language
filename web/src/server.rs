use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use axum::Router;
use axum::extract::Request;
use axum::http::{HeaderValue, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::LeptosRoutes;
use shared::AppError;
use shared::artifact::discovery::{self, DiscoveryDocument};
use shared::artifact::manifest;
use shared::filesystem;
use tower_http::services::ServeDir;

use crate::app::{App, shell};

pub const ENV_LOCAL_REPOSITORY_ROOT: &str = "PORTUGUESE_COURSE_LOCAL_REPOSITORY_ROOT";
pub const ENV_REPOSITORY_BASE_URL: &str = "PORTUGUESE_COURSE_REPOSITORY_BASE_URL";

const DEFAULT_LOCAL_REPOSITORY_ROOT: &str = "build/repository";
const CACHE_CONTROL_IMMUTABLE: &str = "public, max-age=31536000, immutable";
const CACHE_CONTROL_POINTER: &str = "public, max-age=300";
const CACHE_CONTROL_REVALIDATE: &str = "no-cache";
const CACHE_CONTROL_DISCOVERY: &str = "public, max-age=3600";

pub async fn serve() -> Result<(), AppError> {
    let configuration: ConfFile = get_configuration(None)
        .map_err(|error| AppError::from(format!("could not read the leptos settings; [error={error}]")))?;
    let leptos_options: LeptosOptions = configuration.leptos_options;
    let address: SocketAddr = leptos_options.site_addr;
    let route_listings: Vec<leptos_axum::AxumRouteListing> = leptos_axum::generate_route_list(App);
    let repository_root: PathBuf = local_repository_root()?;
    let discovery_text: String = discovery_document_text()?;

    let app_router: Router = Router::new()
        .leptos_routes(&leptos_options, route_listings, {
            let leptos_options: LeptosOptions = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(axum::middleware::map_response(require_revalidation))
        .with_state(leptos_options);

    let router: Router = app_router
        .route(discovery::DISCOVERY_PATH, get(move || serve_discovery(discovery_text.clone())))
        .nest(discovery::STATIC_REPOSITORY_BASE_URL, repository_router(&repository_root));

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(|error| AppError::from(format!("could not bind; [address={address} error={error}]")))?;

    log!("listening on http://{}; [repository={}]", &address, repository_root.display());

    axum::serve(listener, router.into_make_service())
        .await
        .map_err(|error| AppError::from(format!("the server stopped; [error={error}]")))
}

fn local_repository_root() -> Result<PathBuf, AppError> {
    let declared_root: String = std::env::var(ENV_LOCAL_REPOSITORY_ROOT).unwrap_or_else(|_| DEFAULT_LOCAL_REPOSITORY_ROOT.to_string());

    filesystem::resolve_workspace_relative(Path::new(&declared_root))
}

fn discovery_document_text() -> Result<String, AppError> {
    let repository_base_url: String =
        std::env::var(ENV_REPOSITORY_BASE_URL).unwrap_or_else(|_| discovery::STATIC_REPOSITORY_BASE_URL.to_string());
    let document: DiscoveryDocument = DiscoveryDocument {
        schema_version: discovery::DISCOVERY_SCHEMA_VERSION,
        repository_base_url,
        minimum_client_version: discovery::CLIENT_VERSION.to_string(),
        sunset: None,
    };

    Ok(serde_json::to_string_pretty(&document)?)
}

async fn serve_discovery(discovery_text: String) -> Response {
    (
        [
            (header::CONTENT_TYPE, HeaderValue::from_static("application/json")),
            (header::CACHE_CONTROL, HeaderValue::from_static(CACHE_CONTROL_DISCOVERY)),
        ],
        discovery_text,
    )
        .into_response()
}

fn repository_router(repository_root: &Path) -> Router {
    Router::new()
        .fallback_service(ServeDir::new(repository_root))
        .layer(axum::middleware::from_fn(set_repository_cache_control))
}

/* The same headers `course_builder publish cloudflare-r2` sets on each object, so both repositories behave alike. */
async fn set_repository_cache_control(request: Request, next: Next) -> Response {
    let is_pointer: bool = request
        .uri()
        .path()
        .trim_start_matches('/')
        .starts_with(&format!("{}/", manifest::LATEST_POINTER_DIRECTORY));
    let mut response: Response = next.run(request).await;
    let cache_control: &'static str = if is_pointer { CACHE_CONTROL_POINTER } else { CACHE_CONTROL_IMMUTABLE };

    if response.status().is_success() {
        response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(cache_control));
    }

    response
}

/* The bundle and the app's pages keep their names across rebuilds, so a browser applying its own freshness guess
   can pair a cached script with a newer module and never start. */
async fn require_revalidation(mut response: Response) -> Response {
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(CACHE_CONTROL_REVALIDATE));

    response
}
