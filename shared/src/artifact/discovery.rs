use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::artifact::schema_version;

pub const DISCOVERY_SCHEMA_VERSION: u32 = 1;

/* The app's own origin serves the discovery document at this path, so a client commits to it and nothing else. */
pub const DISCOVERY_PATH: &str = "/discovery";

/* Used when discovery cannot be read, and to decide whether the first manifest request already went to the right place. */
pub const STATIC_REPOSITORY_BASE_URL: &str = "/repository";

pub const EMBEDDED_BUNDLE_BASE_URL: &str = "/embedded";

/* Compared against `minimum_client_version`; bumped when a client build can no longer read what is published. */
pub const CLIENT_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryDocument {
    pub schema_version: u32,
    pub repository_base_url: String,
    pub minimum_client_version: String,
    pub sunset: Option<String>,
}

pub fn parse_discovery_document(bytes: &[u8]) -> Result<DiscoveryDocument, AppError> {
    schema_version::require_schema_version(bytes, "schema_version", DISCOVERY_SCHEMA_VERSION)?;

    let document: DiscoveryDocument = serde_json::from_slice(bytes)?;

    Ok(document)
}

/// Which repository base a client should trust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthoritativeBase {
    Static,
    Discovered(String),
}

pub fn authoritative_repository_base(static_base: &str, discovery: &Result<DiscoveryDocument, AppError>) -> AuthoritativeBase {
    match discovery {
        Err(_) => AuthoritativeBase::Static,
        Ok(document) if document.repository_base_url == static_base => AuthoritativeBase::Static,
        Ok(document) => AuthoritativeBase::Discovered(document.repository_base_url.clone()),
    }
}

/// Whether `client_version` is at least `minimum_version`, comparing dot-separated integers; an unparseable
/// minimum is treated as satisfied so a malformed document cannot lock every client out.
pub fn client_version_is_supported(client_version: &str, minimum_version: &str) -> bool {
    let client_parts: Option<Vec<u64>> = parse_version_parts(client_version);
    let minimum_parts: Option<Vec<u64>> = parse_version_parts(minimum_version);

    match (client_parts, minimum_parts) {
        (Some(client_parts), Some(minimum_parts)) => client_parts >= minimum_parts,
        _ => true,
    }
}

fn parse_version_parts(version: &str) -> Option<Vec<u64>> {
    version.split('.').map(|part| part.parse::<u64>().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_DISCOVERY_JSON: &str = r#"{
  "schema_version": 1,
  "repository_base_url": "https://repository.example",
  "minimum_client_version": "0.1.0",
  "sunset": null
}"#;

    fn document_with_base(repository_base_url: &str) -> DiscoveryDocument {
        DiscoveryDocument {
            schema_version: 1,
            repository_base_url: repository_base_url.to_string(),
            minimum_client_version: "0.1.0".to_string(),
            sunset: None,
        }
    }

    #[test]
    fn parse_discovery_document_reads_the_fields() {
        let document: DiscoveryDocument = parse_discovery_document(VALID_DISCOVERY_JSON.as_bytes()).unwrap();

        assert_eq!(document.repository_base_url, "https://repository.example");
        assert_eq!(document.sunset, None);
    }

    #[test]
    fn parse_discovery_document_rejects_unknown_schema_version() {
        let json: String = VALID_DISCOVERY_JSON.replace("\"schema_version\": 1", "\"schema_version\": 2");

        let error: AppError = parse_discovery_document(json.as_bytes()).unwrap_err();

        assert!(error.to_string().contains("schema_version 2 comes from a newer build"));
    }

    #[test]
    fn parse_discovery_document_handles_a_missing_sunset_and_unknown_fields() {
        let json: &str = r#"{
  "schema_version": 1,
  "repository_base_url": "https://repository.example",
  "minimum_client_version": "0.1.0",
  "field_added_later": "ignored"
}"#;

        let document: DiscoveryDocument = parse_discovery_document(json.as_bytes()).unwrap();

        assert_eq!(document.sunset, None);
    }

    #[test]
    fn authoritative_repository_base_uses_static_when_discovery_fails() {
        let discovery: Result<DiscoveryDocument, AppError> = Err(AppError::from("discovery failed".to_string()));

        assert_eq!(authoritative_repository_base("/repository", &discovery), AuthoritativeBase::Static);
    }

    #[test]
    fn authoritative_repository_base_uses_static_when_discovery_matches() {
        let discovery: Result<DiscoveryDocument, AppError> = Ok(document_with_base("/repository"));

        assert_eq!(authoritative_repository_base("/repository", &discovery), AuthoritativeBase::Static);
    }

    #[test]
    fn authoritative_repository_base_uses_discovered_when_base_differs() {
        let discovery: Result<DiscoveryDocument, AppError> = Ok(document_with_base("https://repository.example"));

        assert_eq!(
            authoritative_repository_base("/repository", &discovery),
            AuthoritativeBase::Discovered("https://repository.example".to_string()),
        );
    }

    #[test]
    fn client_version_is_supported_compares_numerically() {
        assert!(client_version_is_supported("0.10.0", "0.9.0"));
        assert!(client_version_is_supported("0.1.0", "0.1.0"));
        assert!(!client_version_is_supported("0.1.0", "0.2.0"));
        assert!(client_version_is_supported("0.1.0", "not a version"));
    }
}
