use shared::AppError;

#[derive(Debug, uniffi::Error)]
pub enum CoreError {
    Failure { message: String },
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::Failure { message } => formatter.write_str(message),
        }
    }
}

impl std::error::Error for CoreError {}

impl From<AppError> for CoreError {
    fn from(error: AppError) -> CoreError {
        CoreError::Failure {
            message: error.message.clone(),
        }
    }
}

impl From<std::io::Error> for CoreError {
    fn from(error: std::io::Error) -> CoreError {
        CoreError::Failure { message: error.to_string() }
    }
}
