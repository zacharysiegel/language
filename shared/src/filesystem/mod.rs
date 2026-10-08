// reads and writes the local filesystem
#[cfg(not(target_arch = "wasm32"))]
pub mod atomic_write;
// reads the local filesystem
#[cfg(not(target_arch = "wasm32"))]
pub mod workspace_root;

// reads and writes the local filesystem
#[cfg(not(target_arch = "wasm32"))]
pub use atomic_write::*;
// reads the local filesystem
#[cfg(not(target_arch = "wasm32"))]
pub use workspace_root::*;
