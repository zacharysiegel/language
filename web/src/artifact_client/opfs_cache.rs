use js_sys::{ArrayBuffer, AsyncIterator, IteratorNext, Promise, Uint8Array};
use shared::AppError;
use shared::artifact::ArtifactCache;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemGetDirectoryOptions, FileSystemGetFileOptions, FileSystemWritableFileStream};

use crate::artifact_client::js_interop;

const CACHE_DIRECTORY_NAME: &str = "artifact-cache";

/// Repository files kept in the origin private file system, one flat directory of content-addressed names.
pub struct OpfsArtifactCache {
    /* `None` when the browser offers no origin private file system, which turns the cache into a pass-through. */
    directory: Option<FileSystemDirectoryHandle>,
}

impl OpfsArtifactCache {
    pub async fn open() -> OpfsArtifactCache {
        let opened: Result<FileSystemDirectoryHandle, AppError> = open_cache_directory().await;

        match opened {
            Ok(directory) => OpfsArtifactCache { directory: Some(directory) },
            Err(error) => {
                log::warn!("the artifact cache is unavailable; [error={error}]");

                OpfsArtifactCache { directory: None }
            },
        }
    }
}

/* Every cached path ends in its content hash, so its last component alone is unique. */
fn entry_name(relative_path: &str) -> &str {
    relative_path.rsplit('/').next().unwrap_or(relative_path)
}

async fn open_cache_directory() -> Result<FileSystemDirectoryHandle, AppError> {
    let window: web_sys::Window = js_interop::get_window()?;
    let root: FileSystemDirectoryHandle = js_interop::await_and_cast(window.navigator().storage().get_directory()).await?;
    let options: FileSystemGetDirectoryOptions = FileSystemGetDirectoryOptions::new();

    options.set_create(true);

    js_interop::await_and_cast(root.get_directory_handle_with_options(CACHE_DIRECTORY_NAME, &options)).await
}

impl ArtifactCache for OpfsArtifactCache {
    async fn put(&self, relative_path: &str, bytes: &[u8]) -> Result<(), AppError> {
        let Some(directory) = &self.directory
        else {
            return Ok(());
        };

        let options: FileSystemGetFileOptions = FileSystemGetFileOptions::new();

        options.set_create(true);

        let file_handle: FileSystemFileHandle =
            js_interop::await_and_cast(directory.get_file_handle_with_options(entry_name(relative_path), &options)).await?;

        write_file(&file_handle, bytes).await
    }

    async fn get(&self, relative_path: &str) -> Result<Option<Vec<u8>>, AppError> {
        let Some(directory) = &self.directory
        else {
            return Ok(None);
        };

        let lookup: Result<JsValue, JsValue> = JsFuture::from(directory.get_file_handle(entry_name(relative_path))).await;
        let file_handle: FileSystemFileHandle = match lookup {
            Ok(value) => js_interop::dyn_into(value)?,
            Err(error) if js_interop::is_dom_exception_named(&error, "NotFoundError") => return Ok(None),
            Err(error) => return Err(js_interop::error(error)),
        };
        let file: File = js_interop::await_and_cast(file_handle.get_file()).await?;

        read_file_bytes(&file).await.map(Some)
    }

    async fn retain(&self, relative_paths_to_keep: &[String]) -> Result<(), AppError> {
        let Some(directory) = &self.directory
        else {
            return Ok(());
        };

        let kept_names: Vec<&str> = relative_paths_to_keep.iter().map(|relative_path| entry_name(relative_path)).collect();

        for name in list_directory_names(directory).await? {
            if kept_names.contains(&name.as_str()) {
                continue;
            }

            JsFuture::from(directory.remove_entry(&name)).await.map_err(js_interop::error)?;
        }

        Ok(())
    }
}

async fn write_file(file_handle: &FileSystemFileHandle, bytes: &[u8]) -> Result<(), AppError> {
    let writable: FileSystemWritableFileStream = js_interop::await_and_cast(file_handle.create_writable()).await?;
    let data: Uint8Array = js_interop::owned_uint8_array(bytes);
    let write_promise: Promise = writable.write_with_js_u8_array(&data).map_err(js_interop::error)?;

    JsFuture::from(write_promise).await.map_err(js_interop::error)?;
    JsFuture::from(writable.close()).await.map_err(js_interop::error)?;

    Ok(())
}

async fn read_file_bytes(file: &File) -> Result<Vec<u8>, AppError> {
    let array_buffer: ArrayBuffer = js_interop::await_and_cast(file.array_buffer()).await?;

    Ok(Uint8Array::new(&array_buffer).to_vec())
}

/* `keys()` returns a JS async iterator, so each step's `{ value, done }` arrives through a promise. */
async fn list_directory_names(directory: &FileSystemDirectoryHandle) -> Result<Vec<String>, AppError> {
    let iterator: AsyncIterator = directory.keys();
    let mut names: Vec<String> = Vec::new();

    loop {
        let next_promise: Promise = iterator.next().map_err(js_interop::error)?;
        let next_value: JsValue = JsFuture::from(next_promise).await.map_err(js_interop::error)?;
        let next: IteratorNext = next_value.unchecked_into();

        if next.done() {
            return Ok(names);
        }

        let name: String = next
            .value()
            .as_string()
            .ok_or_else(|| AppError::from("a directory key is not a string".to_string()))?;

        names.push(name);
    }
}
