use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use base64::Engine;
use secr::store::SecretStore;
use secr::{BASE64, cryptography, load};
use shared::AppError;
use shared::filesystem;

pub const ENV_MASTER_SECRET: &str = "MASTER_SECRET";
pub const ENV_SECR_STORE_PATH: &str = "SECR_STORE_PATH";

static SECRET_STORE: LazyLock<Result<SecretStore, String>> = LazyLock::new(|| load_secret_store().map_err(|error| error.to_string()));

pub fn master_decrypt_utf8(secret_name: &str) -> Result<String, AppError> {
    let master_secret_base64: String = read_environment(ENV_MASTER_SECRET)?;
    let master_secret: Vec<u8> = BASE64
        .decode(master_secret_base64)
        .map_err(|error| AppError::from(format!("{ENV_MASTER_SECRET} is not base64; [error={error}]")))?;
    let secret_store: &SecretStore = SECRET_STORE.as_ref().map_err(|message| AppError::from(message.clone()))?;
    let secret: Vec<u8> = cryptography::decrypt(secret_store, &master_secret, secret_name)
        .map_err(|error| AppError::from(format!("could not decrypt a secret; [name={secret_name} error={error}]")))?;

    String::from_utf8(secret).map_err(|error| AppError::from(format!("a secret is not utf-8; [name={secret_name} error={error}]")))
}

pub fn read_environment(name: &str) -> Result<String, AppError> {
    dotenvy::var(name).map_err(|error| AppError::from(format!("missing environment variable; [name={name} error={error}]")))
}

/// `None` when the variable is unset or empty.
pub fn read_optional_environment(name: &str) -> Option<String> {
    dotenvy::var(name).ok().filter(|value| !value.is_empty())
}

/// A relative `SECR_STORE_PATH` is resolved against the workspace root.
fn load_secret_store() -> Result<SecretStore, AppError> {
    let declared_store_path: String = read_environment(ENV_SECR_STORE_PATH)?;
    let store_path: PathBuf = filesystem::resolve_workspace_relative(Path::new(&declared_store_path))?;

    load::load_secrets_from_file(&store_path)
        .map_err(|error| AppError::from(format!("could not load the secret store; [path={} error={error}]", store_path.display())))
}
