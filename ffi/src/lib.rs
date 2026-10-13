//! The core as the native apps see it: the one reviewable surface their generated bindings come from.

uniffi::setup_scaffolding!();

// writes to logcat through Android's liblog, which other targets lack
#[cfg(target_os = "android")]
pub mod core_logging;
pub mod course_library;
pub mod course_records;
pub mod ffi_error;
pub mod grouped_table;
pub mod practice_functions;
pub mod progress_book;
pub mod recording_cache;
pub mod unit_item_label;

// writes to logcat through Android's liblog, which other targets lack
#[cfg(target_os = "android")]
pub use core_logging::*;
pub use course_library::*;
pub use course_records::*;
pub use ffi_error::*;
pub use grouped_table::*;
pub use practice_functions::*;
pub use progress_book::*;
pub use recording_cache::*;
pub use unit_item_label::*;
