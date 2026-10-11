use shared::AppError;
use shared::artifact::{self, DiscoveryDocument};

pub const STATIC_DISCOVERY_JSON: &str = include_str!("../../static/discovery");

pub fn static_discovery_document() -> Result<DiscoveryDocument, AppError> {
    artifact::parse_discovery_document(STATIC_DISCOVERY_JSON.as_bytes())
}

/// Where a client looks when discovery cannot be read, and where the embedded bundle's recordings are served.
pub fn static_repository_base_url() -> Result<String, AppError> {
    // A site build compiles in the repository its deployed discovery document names.
    match option_env!("LANGUAGE_REPOSITORY_BASE_URL") {
        Some(repository_base_url) => Ok(repository_base_url.to_string()),
        None => Ok(static_discovery_document()?.repository_base_url),
    }
}

#[cfg(test)]
mod tests {
    use shared::artifact::{discovery, local_repository};

    use super::*;

    #[test]
    fn static_discovery_document_names_the_local_repository() {
        let document: DiscoveryDocument = static_discovery_document().unwrap();

        assert_eq!(document.repository_base_url, local_repository::LOCAL_REPOSITORY_BASE_URL);
        assert_eq!(document.minimum_client_version, discovery::CLIENT_VERSION);
        assert_eq!(document.sunset, None);
    }
}
