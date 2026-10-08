use std::backtrace::Backtrace;

use shared::{AppError, AppErrorStatic};

/// A failure in the core. `message` names what failed and why; `backtrace` is the Rust call stack where the error
/// arose, for logs rather than for the learner.
#[derive(Debug, uniffi::Error)]
pub enum CoreError {
    Failure { message: String, backtrace: String },
}

impl CoreError {
    /// A failure that arose here, with this call stack.
    pub fn arising(message: String) -> CoreError {
        CoreError::Failure {
            message,
            backtrace: Backtrace::force_capture().to_string(),
        }
    }
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::Failure { message, .. } => formatter.write_str(message),
        }
    }
}

impl std::error::Error for CoreError {}

/* `AppErrorStatic` keeps the source error's chain as text in its message. */
impl From<AppError> for CoreError {
    fn from(error: AppError) -> CoreError {
        CoreError::from(AppErrorStatic::from(error))
    }
}

impl From<AppErrorStatic> for CoreError {
    fn from(error: AppErrorStatic) -> CoreError {
        let underlying: &minimer::AppErrorStatic = &error.0;

        CoreError::Failure {
            message: underlying.message.clone(),
            backtrace: underlying.backtrace.to_string(),
        }
    }
}

impl From<std::io::Error> for CoreError {
    fn from(error: std::io::Error) -> CoreError {
        CoreError::arising(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn failing_operation() -> Result<(), AppError> {
        Err(AppError::from_error("reading a shard failed", Box::new(std::io::Error::other("disk unplugged"))))
    }

    #[test]
    fn from_app_error_keeps_the_cause_and_the_backtrace_where_it_arose() {
        let error: CoreError = failing_operation().map_err(CoreError::from).unwrap_err();
        let CoreError::Failure { message, backtrace } = error;

        assert!(message.ends_with("reading a shard failed -> disk unplugged"), "{message}");
        assert!(backtrace.contains("failing_operation"), "{backtrace}");
    }
}
