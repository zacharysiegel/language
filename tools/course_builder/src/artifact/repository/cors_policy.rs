use std::fs;
use std::path::Path;

use serde::Deserialize;
use shared::AppError;

/// The cross-origin rule a repository applies so the app's origin may read its files.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorsPolicy {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub max_age_seconds: i32,
}

pub fn read_cors_policy(path: &Path) -> Result<CorsPolicy, AppError> {
    let text: String = fs::read_to_string(path)
        .map_err(|error| AppError::from(format!("could not read the CORS policy; [path={} error={error}]", path.display())))?;
    let policy: CorsPolicy = serde_json::from_str(&text)?;

    if policy.allowed_origins.is_empty() || policy.allowed_methods.is_empty() {
        return Err(AppError::from(format!("the CORS policy names no origin or no method; [path={}]", path.display())));
    }

    Ok(policy)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn read_cors_policy_reads_the_committed_policy() {
        let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("r2-cors.json");

        let policy: CorsPolicy = read_cors_policy(&path).unwrap();

        assert!(policy.allowed_methods.contains(&"GET".to_string()));
    }
}
