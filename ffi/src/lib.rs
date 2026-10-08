//! The core as the native apps see it: the one reviewable surface their generated bindings come from.

uniffi::setup_scaffolding!();

pub mod course_library;
pub mod course_records;
pub mod ffi_error;
pub mod practice_functions;
pub mod progress_book;

pub use course_library::*;
pub use course_records::*;
pub use ffi_error::*;
pub use practice_functions::*;
pub use progress_book::*;
