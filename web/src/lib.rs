/* Release builds monomorphize the whole view tree into one deeply nested concrete type, whose layout
   computation exceeds rustc's default query depth. Dev builds do not hit it: cargo-leptos type-erases them. */
#![recursion_limit = "512"]

pub mod app;
pub mod audio_player;
pub mod course_data;
pub mod glossary;
pub mod home;
pub mod platform;
pub mod practice;
pub mod progress_state;
pub mod progress_view;
pub mod review;
pub mod unit;

// Browser-only runtime glue; the ssr build compiles none of it.
#[cfg(feature = "hydrate")]
pub mod client;

// The dev server; it does not exist in a browser.
#[cfg(feature = "ssr")]
pub mod server;
