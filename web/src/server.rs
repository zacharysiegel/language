use std::net::SocketAddr;

use axum::Router;
use axum::http::{HeaderValue, header};
use axum::response::Response;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::LeptosRoutes;
use shared::AppError;

use crate::app::{App, shell};

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

/* The bundle and the data keep their names across rebuilds, so a browser applying its own freshness guess can pair a
   cached script with a newer module or data file and never start. */
async fn require_revalidation(mut response: Response) -> Response {
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    response
}
