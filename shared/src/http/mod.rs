pub mod http_model;

// reads the local filesystem
#[cfg(not(target_arch = "wasm32"))]
pub mod directory_fetch;
// reqwest opens its own sockets, which wasm32 cannot
#[cfg(not(target_arch = "wasm32"))]
pub mod reqwest_fetch;

pub use http_model::*;

// reads the local filesystem
#[cfg(not(target_arch = "wasm32"))]
pub use directory_fetch::*;
// reqwest opens its own sockets, which wasm32 cannot
#[cfg(not(target_arch = "wasm32"))]
pub use reqwest_fetch::*;
