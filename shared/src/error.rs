use std::error::Error;

minimer::define_app_error!(pub AppError);

// `Send + Sync`, unlike `AppError`, because it keeps no source error.
minimer::define_app_error_static!(pub AppErrorStatic);

minimer::impl_from_error!(AppError, serde_json::Error);

impl From<AppError> for AppErrorStatic {
    fn from(error: AppError) -> AppErrorStatic {
        AppErrorStatic(minimer::AppErrorStatic::from(error.0))
    }
}

impl From<AppErrorStatic> for AppError {
    fn from(error: AppErrorStatic) -> AppError {
        AppError(minimer::AppError::from(error.0))
    }
}

pub fn render_error_chain(error: &dyn Error) -> String {
    let mut rendered: String = error.to_string();
    let mut next: Option<&dyn Error> = error.source();

    while let Some(source) = next {
        rendered.push_str(" -> ");
        rendered.push_str(&source.to_string());
        next = source.source();
    }

    rendered
}

#[cfg(not(target_arch = "wasm32"))] // rusqlite does not build for wasm32
minimer::impl_from_error!(AppError, rusqlite::Error);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_error_static_from_app_error_keeps_the_source_error_as_text() {
        let error: AppError = AppError::from_error("reading a shard failed", Box::new(std::io::Error::other("disk unplugged")));

        let static_error: AppErrorStatic = AppErrorStatic::from(error);

        assert!(static_error.message.ends_with("reading a shard failed -> disk unplugged"), "{}", static_error.message);
    }
}
