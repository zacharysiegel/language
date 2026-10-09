pub const ENV_LOCAL_REPOSITORY_ROOT: &str = "LANGUAGE_LOCAL_REPOSITORY_ROOT";

/* Relative to the workspace root. */
pub const DEFAULT_LOCAL_REPOSITORY_DIRECTORY: &str = "build/repository";

pub const LOCAL_REPOSITORY_SOCKET_ADDRESS: &str = "127.0.0.1:3002";

pub const LOCAL_REPOSITORY_BASE_URL: &str = "http://127.0.0.1:3002";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_repository_base_url_names_the_socket_address() {
        assert_eq!(LOCAL_REPOSITORY_BASE_URL, format!("http://{LOCAL_REPOSITORY_SOCKET_ADDRESS}"));
    }
}
