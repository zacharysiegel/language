use shared::AppErrorStatic;
use shared::artifact::repository_key;
use shared::grading;
use wasm_bindgen_futures::JsFuture;
use web_sys::{FileSystemDirectoryHandle, FileSystemGetDirectoryOptions};

use crate::artifact_client::{self, js_interop};
use crate::grading::web_lock;

/* wllama 3.8.1 keeps its cache in this directory of the origin private file system, each file named
   `<hash of its URL>_<file name>` beside a `__metadata__` file of the same name. */
const WLLAMA_CACHE_DIRECTORY_NAME: &str = "cache";

// wllama 3.8.1's error when a split model's part has no metadata, which every later download of the model then fails with.
pub const INCOMPLETE_MODEL_ERROR_TEXT: &str = "Model file not found";

// cross-llama 0.2.0's Web Lock over a model's cached files is this prefix followed by the model's absolute URL.
const CROSS_LLAMA_MODEL_LOCK_NAME_PREFIX: &str = "cross-llama model ";

/// Deletes every file of the grading model from wllama's cache, complete or not, holding cross-llama's lock on `model_url`.
pub async fn remove_grading_model_files(model_url: &str) -> Result<usize, AppErrorStatic> {
    let lock_name: String = get_cross_llama_model_lock_name(model_url)?;

    web_lock::run_while_holding(&lock_name, remove_grading_model_files_unlocked()).await?
}

async fn remove_grading_model_files_unlocked() -> Result<usize, AppErrorStatic> {
    let cache_directory: FileSystemDirectoryHandle = open_wllama_cache_directory().await?;
    let part_name_suffixes: Vec<String> = (1..=grading::GRADING_MODEL_PART_COUNT)
        .map(|part_number| {
            let part_file_name: String =
                repository_key::model_part_file_name(grading::GRADING_MODEL_NAME, part_number, grading::GRADING_MODEL_PART_COUNT);

            format!("_{part_file_name}")
        })
        .collect();
    let mut removed_count: usize = 0;

    for entry_name in artifact_client::list_directory_names(&cache_directory).await? {
        let is_model_file: bool = part_name_suffixes.iter().any(|suffix| entry_name.ends_with(suffix.as_str()));

        if !is_model_file {
            continue;
        }

        JsFuture::from(cache_directory.remove_entry(&entry_name)).await.map_err(js_interop::error)?;
        removed_count += 1;
    }

    Ok(removed_count)
}

fn get_cross_llama_model_lock_name(model_url: &str) -> Result<String, AppErrorStatic> {
    let page_url: String = js_interop::get_window()?.location().href().map_err(js_interop::error)?;
    let resolved_model_url: web_sys::Url = web_sys::Url::new_with_base(model_url, &page_url).map_err(js_interop::error)?;

    Ok(format!("{CROSS_LLAMA_MODEL_LOCK_NAME_PREFIX}{}", resolved_model_url.href()))
}

async fn open_wllama_cache_directory() -> Result<FileSystemDirectoryHandle, AppErrorStatic> {
    let window: web_sys::Window = js_interop::get_window()?;
    let root: FileSystemDirectoryHandle = js_interop::await_and_cast(window.navigator().storage().get_directory()).await?;
    let options: FileSystemGetDirectoryOptions = FileSystemGetDirectoryOptions::new();

    options.set_create(true);

    js_interop::await_and_cast(root.get_directory_handle_with_options(WLLAMA_CACHE_DIRECTORY_NAME, &options)).await
}
