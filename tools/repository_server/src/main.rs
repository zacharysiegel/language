use std::path::{Path, PathBuf};
use std::process::ExitCode;

use axum::Router;
use axum::extract::Request;
use axum::http::{HeaderValue, Method, header};
use axum::middleware::Next;
use axum::response::Response;
use shared::AppError;
use shared::artifact::{local_repository, repository_key};
use shared::filesystem;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let result: Result<(), AppError> = serve().await;

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        },
    }
}

async fn serve() -> Result<(), AppError> {
    let address: &str = local_repository::LOCAL_REPOSITORY_SOCKET_ADDRESS;
    let repository_root: PathBuf = local_repository_root()?;
    let router: Router = repository_router(&repository_root);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|error| AppError::from(format!("could not bind; [address={address} error={error}]")))?;

    log::info!("serving the local repository; [url={} root={}]", local_repository::LOCAL_REPOSITORY_BASE_URL, repository_root.display());

    axum::serve(listener, router.into_make_service())
        .await
        .map_err(|error| AppError::from(format!("the server stopped; [error={error}]")))
}

fn local_repository_root() -> Result<PathBuf, AppError> {
    let declared_root: String = dotenvy::var(local_repository::ENV_LOCAL_REPOSITORY_ROOT)
        .unwrap_or_else(|_| local_repository::DEFAULT_LOCAL_REPOSITORY_DIRECTORY.to_string());

    filesystem::resolve_workspace_relative(Path::new(&declared_root))
}

fn repository_router(repository_root: &Path) -> Router {
    let cross_origin_reads: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::HEAD,
        ]);

    Router::new()
        .fallback_service(ServeDir::new(repository_root))
        .layer(axum::middleware::from_fn(set_repository_cache_control))
        .layer(cross_origin_reads)
}

/* The same headers `course_builder publish cloudflare-r2` sets on each object, so both repositories behave alike. */
async fn set_repository_cache_control(request: Request, next: Next) -> Response {
    let cache_control: &'static str = repository_cache_control(request.uri().path());
    let mut response: Response = next.run(request).await;

    if response.status().is_success() {
        response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(cache_control));
    }

    response
}

fn repository_cache_control(path: &str) -> &'static str {
    repository_key::cache_control_for_key(path.trim_start_matches('/'))
}

#[cfg(test)]
mod tests {
    use super::*;

    const COURSE_ID: &str = "pt_br.en.foundations";

    #[test]
    fn repository_cache_control_revalidates_the_index_and_a_course_pointer_and_keeps_a_publication_file() {
        let course_pointer_path: String = format!("/{}", repository_key::course_latest_manifest_key(COURSE_ID));
        let publication_file_path: String = format!("/{}", repository_key::publication_file_key(COURSE_ID, "2026-10-08+duluth", "data/course-ab.sqlite.br"));

        assert_eq!(repository_cache_control(&format!("/{}", repository_key::COURSE_INDEX_KEY)), repository_key::CACHE_CONTROL_POINTER);
        assert_eq!(repository_cache_control(&course_pointer_path), repository_key::CACHE_CONTROL_POINTER);
        assert_eq!(repository_cache_control(&publication_file_path), repository_key::CACHE_CONTROL_IMMUTABLE);
    }
}
