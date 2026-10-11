/* The bin lays out the future returned by the lib's render path, which monomorphizes the whole view tree
   and overflows rustc's default query depth. Only a release build shows it: `cargo check` skips codegen,
   and cargo-leptos type-erases dev builds. */
#![recursion_limit = "512"]

#[cfg(feature = "ssr")] // the server binary has no role in the wasm32 build
mod server_binary {
    use std::env;
    use std::process;

    use shared::AppError;

    const PRERENDER_ARGUMENT: &str = "prerender";

    enum ExitStatus {
        Failed = 1,
        Usage = 64, // the conventional status for a misuse of a command, which this repository's scripts use too
    }

    pub async fn run() {
        let first_argument: Option<String> = env::args().nth(1);

        let result: Result<(), AppError> = match first_argument.as_deref() {
            None => web::server::serve().await,
            Some(PRERENDER_ARGUMENT) => web::server::write_static_shell(),
            Some(unrecognized) => {
                eprintln!("unrecognized argument; [argument={unrecognized} expected={PRERENDER_ARGUMENT}]");
                process::exit(ExitStatus::Usage as i32);
            },
        };

        if let Err(error) = result {
            eprintln!("{error}");
            process::exit(ExitStatus::Failed as i32);
        }
    }
}

#[cfg(feature = "ssr")] // the server binary has no role in the wasm32 build
#[tokio::main]
async fn main() {
    server_binary::run().await;
}

#[cfg(not(feature = "ssr"))] // the wasm32 build is a library only
fn main() {}
