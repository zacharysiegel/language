//! Convert raw `sqlite-wasm-rs` FFI outputs into owned Rust values. wasm32-only; native uses rusqlite.

use std::ffi::CStr;

use sqlite_wasm_rs::{sqlite3, sqlite3_stmt};

use crate::AppError;
use crate::sqlite::SqlValue;

/// The column's value in the type SQLite stored it as.
pub(crate) fn column_value(statement: *mut sqlite3_stmt, column_index: std::os::raw::c_int) -> Result<SqlValue, AppError> {
    let column_type: std::os::raw::c_int = unsafe { sqlite_wasm_rs::sqlite3_column_type(statement, column_index) };

    match column_type {
        sqlite_wasm_rs::SQLITE_INTEGER => {
            let integer: i64 = unsafe { sqlite_wasm_rs::sqlite3_column_int64(statement, column_index) };

            Ok(SqlValue::Integer(integer))
        },
        sqlite_wasm_rs::SQLITE_FLOAT => {
            let real: f64 = unsafe { sqlite_wasm_rs::sqlite3_column_double(statement, column_index) };

            Ok(SqlValue::Real(real))
        },
        sqlite_wasm_rs::SQLITE_TEXT => column_text(statement, column_index).map(SqlValue::Text),
        sqlite_wasm_rs::SQLITE_BLOB => Ok(SqlValue::Blob(column_blob(statement, column_index))),
        _ => Ok(SqlValue::Null),
    }
}

fn column_text(statement: *mut sqlite3_stmt, column_index: std::os::raw::c_int) -> Result<String, AppError> {
    let text: *const std::os::raw::c_uchar = unsafe { sqlite_wasm_rs::sqlite3_column_text(statement, column_index) };

    if text.is_null() {
        return Err(AppError::from(format!("ffi_conversions: null text column; [column={column_index}]")));
    }

    let text: &CStr = unsafe { CStr::from_ptr(text as *const std::os::raw::c_char) };

    text.to_str()
        .map(|value| value.to_string())
        .map_err(|error| AppError::from(format!("ffi_conversions: non-utf8 text column; [column={column_index} error={error}]")))
}

fn column_blob(statement: *mut sqlite3_stmt, column_index: std::os::raw::c_int) -> Vec<u8> {
    let byte_count: std::os::raw::c_int = unsafe { sqlite_wasm_rs::sqlite3_column_bytes(statement, column_index) };
    let blob: *const std::os::raw::c_void = unsafe { sqlite_wasm_rs::sqlite3_column_blob(statement, column_index) };

    if blob.is_null() || byte_count <= 0 {
        return Vec::new();
    }

    unsafe { std::slice::from_raw_parts(blob as *const u8, byte_count as usize) }.to_vec()
}

/// The connection's last error message, or `"unknown sqlite error"` if none is set.
pub(crate) fn error_message(db: *mut sqlite3) -> String {
    let raw: *const std::os::raw::c_char = unsafe { sqlite_wasm_rs::sqlite3_errmsg(db) };

    if raw.is_null() {
        return "unknown sqlite error".to_string();
    }

    unsafe { CStr::from_ptr(raw) }.to_string_lossy().into_owned()
}
