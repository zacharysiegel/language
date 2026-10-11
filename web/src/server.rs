use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use axum::Router;
use axum::http::{HeaderValue, header};
use axum::response::Response;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::LeptosRoutes;
use shared::AppError;
use shared::filesystem;

use crate::app::{App, shell, static_shell};

const CACHE_CONTROL_REVALIDATE: &str = "no-cache";
const STATIC_SHELL_DOCUMENT_NAME: &str = "index.html";
const CRATE_DIRECTORY: &str = env!("CARGO_MANIFEST_DIR");
const MANIFEST_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");

pub async fn serve() -> Result<(), AppError> {
    let configuration: ConfFile = get_configuration(None)
        .map_err(|error| AppError::from(format!("could not read the leptos settings; [error={error}]")))?;
    let leptos_options: LeptosOptions = configuration.leptos_options;
    let address: SocketAddr = leptos_options.site_addr;
    let route_listings: Vec<leptos_axum::AxumRouteListing> = leptos_axum::generate_route_list(App);

    let router: Router = Router::new()
        .leptos_routes(&leptos_options, route_listings, {
            let leptos_options: LeptosOptions = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(axum::middleware::map_response(require_revalidation))
        .with_state(leptos_options);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(|error| AppError::from(format!("could not bind; [address={address} error={error}]")))?;

    log!("listening on http://{}", &address);

    axum::serve(listener, router.into_make_service())
        .await
        .map_err(|error| AppError::from(format!("the server stopped; [error={error}]")))
}

/* The bundle and the app's pages keep their names across rebuilds, so a browser applying its own freshness guess
   can pair a cached script with a newer module and never start. */
async fn require_revalidation(mut response: Response) -> Response {
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(CACHE_CONTROL_REVALIDATE));

    response
}

/// Writes the static shell into the site root of a build that has already run.
pub fn write_static_shell() -> Result<(), AppError> {
    let leptos_options: LeptosOptions = read_manifest_leptos_options()?;
    let site_root: PathBuf = resolve_site_root(&leptos_options)?;
    let hash_files: bool = leptos_options.hash_files;
    let document_text: String = render_static_shell(leptos_options);
    let document_path: PathBuf = site_root.join(STATIC_SHELL_DOCUMENT_NAME);

    fs::write(&document_path, &document_text).map_err(|error| {
        AppError::from(format!("could not write the static shell; [path={} error={error}]", document_path.display()))
    })?;

    log!("wrote the static shell; [path={} bytes={} hash_files={hash_files}]", document_path.display(), document_text.len());

    Ok(())
}

pub fn render_static_shell(leptos_options: LeptosOptions) -> String {
    let owner: Owner = Owner::new();

    owner.with(move || static_shell(leptos_options).to_html())
}

/* `get_configuration(None)` reads variables only cargo-leptos sets, so this reads the manifest, which
   leptos_config overlays with any LEPTOS_* variable in the environment. */
fn read_manifest_leptos_options() -> Result<LeptosOptions, AppError> {
    let configuration: ConfFile = get_configuration(Some(MANIFEST_PATH))
        .map_err(|error| AppError::from(format!("could not read the leptos settings; [path={MANIFEST_PATH} error={error}]")))?;

    Ok(configuration.leptos_options)
}

// cargo-leptos resolves the site root against the workspace root.
fn resolve_site_root(leptos_options: &LeptosOptions) -> Result<PathBuf, AppError> {
    let workspace_root: PathBuf = filesystem::find_workspace_root(Path::new(CRATE_DIRECTORY))?;
    let declared_site_root: PathBuf = workspace_root.join(leptos_options.site_root.as_ref());

    fs::canonicalize(&declared_site_root).map_err(|error| {
        AppError::from(format!(
            "the site root does not exist, so no build has run; [path={} error={error}]",
            declared_site_root.display(),
        ))
    })
}

#[cfg(test)]
mod tests {
    use crate::app::STATIC_SHELL_ELEMENT_ID;

    use super::*;

    #[test]
    fn render_static_shell_holds_the_shell_marker_and_the_app_title() {
        let leptos_options: LeptosOptions = LeptosOptions::builder().output_name("language").build();

        let document_text: String = render_static_shell(leptos_options);

        assert!(document_text.starts_with("<!DOCTYPE html>"));
        assert!(document_text.contains(&format!(r#"id="{STATIC_SHELL_ELEMENT_ID}""#)));
        assert!(document_text.contains("<title>Language</title>"));
        assert!(document_text.contains("/pkg/language.css"));
    }
}
