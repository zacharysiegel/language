// reads the local filesystem
#[cfg(not(target_arch = "wasm32"))]
pub mod workspace_root;

// reads the local filesystem
#[cfg(not(target_arch = "wasm32"))]
pub use workspace_root::*;
