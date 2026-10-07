use js_sys::{ArrayBuffer, Promise, Uint8Array};
use shared::AppError;
use shared::http::{HttpCacheMode, HttpFetch, HttpRequest, HttpResponse};
use web_sys::{RequestCache, RequestInit, Window};

use crate::artifact_client::js_interop;

pub struct BrowserFetch;

impl HttpFetch for BrowserFetch {
    async fn get(&self, request: &HttpRequest) -> Result<HttpResponse, AppError> {
        let window: Window = js_interop::get_window()?;
        let init: RequestInit = RequestInit::new();

        init.set_method("GET");

        match request.cache_mode {
            HttpCacheMode::Reload => init.set_cache(RequestCache::Reload),
            HttpCacheMode::Default => {},
        }

        let fetch_promise: Promise = window.fetch_with_str_and_init(&request.url, &init);
        let response: web_sys::Response = js_interop::await_and_cast(fetch_promise).await?;
        let status: u16 = response.status();
        let buffer_promise: Promise = response.array_buffer().map_err(js_interop::error)?;
        let array_buffer: ArrayBuffer = js_interop::await_and_cast(buffer_promise).await?;

        Ok(HttpResponse {
            status,
            bytes: Uint8Array::new(&array_buffer).to_vec(),
        })
    }
}
