/* Release builds monomorphize the whole view tree into one deeply nested concrete type, whose layout
   computation exceeds rustc's default query depth. Dev builds do not hit it: cargo-leptos type-erases them. */
#![recursion_limit = "512"]

pub mod app;
pub mod artifact_client;
pub mod audio_player;
pub mod course_data;
pub mod courses;
pub mod glossary;
pub mod home;
pub mod platform;
pub mod practice;
pub mod progress_state;
pub mod progress_view;
pub mod review;
pub mod unit;

#[cfg(feature = "hydrate")] // uses browser APIs the server build lacks
pub mod client;

#[cfg(feature = "ssr")] // serves files and sockets a browser build cannot
pub mod server;
