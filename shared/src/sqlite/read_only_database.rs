//! A read-only SQLite database opened over a shard's bytes in memory. Shards are immutable, so nothing writes.

#[cfg(not(target_arch = "wasm32"))] // wasm32 reads through the read-only memory VFS instead
pub use native::ReadOnlyDatabase;
#[cfg(target_arch = "wasm32")] // rusqlite does not build for wasm32
pub use wasm::ReadOnlyDatabase;

#[cfg(not(target_arch = "wasm32"))] // wasm32 reads through the read-only memory VFS instead
mod native {
    use std::ptr::NonNull;

    use rusqlite::serialize::OwnedData;
    use rusqlite::types::ValueRef;
    use rusqlite::{Connection, DatabaseName};

    use crate::AppError;
    use crate::sqlite::{SqlRecord, SqlValue};

    pub struct ReadOnlyDatabase {
        connection: Connection,
    }

    impl ReadOnlyDatabase {
        /// rusqlite's `deserialize` takes a SQLite-owned (`sqlite3_malloc`'d) buffer that it frees on close,
        /// so the bytes are copied into one.
        pub fn open(bytes: &[u8]) -> Result<ReadOnlyDatabase, AppError> {
            let mut connection: Connection = Connection::open_in_memory()?;
            let byte_count: usize = bytes.len();

            let owned_data: OwnedData = unsafe {
                let raw: *mut u8 = rusqlite::ffi::sqlite3_malloc(byte_count as std::os::raw::c_int) as *mut u8;
                let raw: NonNull<u8> = NonNull::new(raw)
                    .ok_or_else(|| AppError::from("read_only_database: sqlite3_malloc returned null".to_string()))?;
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), raw.as_ptr(), byte_count);

                OwnedData::from_raw_nonnull(raw, byte_count)
            };

            connection.deserialize(DatabaseName::Main, owned_data, true)?;

            Ok(ReadOnlyDatabase { connection })
        }

        pub fn query(&self, sql: &str) -> Result<Vec<SqlRecord>, AppError> {
            let mut statement: rusqlite::Statement<'_> = self.connection.prepare(sql)?;
            let column_count: usize = statement.column_count();
            let mut rows: rusqlite::Rows<'_> = statement.query([])?;
            let mut records: Vec<SqlRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                let mut values: Vec<SqlValue> = Vec::with_capacity(column_count);

                for column_index in 0..column_count {
                    values.push(to_sql_value(row.get_ref(column_index)?)?);
                }

                records.push(SqlRecord::new(values));
            }

            Ok(records)
        }
    }

    fn to_sql_value(value: ValueRef<'_>) -> Result<SqlValue, AppError> {
        match value {
            ValueRef::Null => Ok(SqlValue::Null),
            ValueRef::Integer(integer) => Ok(SqlValue::Integer(integer)),
            ValueRef::Real(real) => Ok(SqlValue::Real(real)),
            ValueRef::Text(text) => {
                let text: &str = std::str::from_utf8(text)
                    .map_err(|error| AppError::from(format!("read_only_database: non-utf8 text; [error={error}]")))?;

                Ok(SqlValue::Text(text.to_string()))
            },
            ValueRef::Blob(blob) => Ok(SqlValue::Blob(blob.to_vec())),
        }
    }
}

#[cfg(target_arch = "wasm32")] // rusqlite does not build for wasm32
mod wasm {
    use std::ffi::CString;

    use sqlite_wasm_rs::{sqlite3, sqlite3_stmt};

    use crate::AppError;
    use crate::sqlite::{SqlRecord, SqlValue, ffi_conversions, ro_memory_vfs};

    pub struct ReadOnlyDatabase {
        handle: *mut sqlite3,
        vfs_filename: String,
    }

    impl ReadOnlyDatabase {
        /// Opens the bytes through the read-only memory VFS; the shard stays registered until the database drops.
        pub fn open(bytes: &[u8]) -> Result<ReadOnlyDatabase, AppError> {
            let vfs_filename: String = ro_memory_vfs::register_shard(bytes);
            let filename: CString = CString::new(vfs_filename.as_str()).expect("a generated filename has no null byte");
            let mut handle: *mut sqlite3 = std::ptr::null_mut();

            let open_result: std::os::raw::c_int = unsafe {
                sqlite_wasm_rs::sqlite3_open_v2(
                    filename.as_ptr(),
                    &mut handle,
                    sqlite_wasm_rs::SQLITE_OPEN_READONLY,
                    ro_memory_vfs::VFS_NAME.as_ptr(),
                )
            };

            if open_result != sqlite_wasm_rs::SQLITE_OK {
                let message: String = ffi_conversions::error_message(handle);

                unsafe { sqlite_wasm_rs::sqlite3_close(handle) };
                ro_memory_vfs::unregister_shard(&vfs_filename);

                return Err(AppError::from(format!("read_only_database: open failed; [error={message}]")));
            }

            Ok(ReadOnlyDatabase { handle, vfs_filename })
        }

        pub fn query(&self, sql: &str) -> Result<Vec<SqlRecord>, AppError> {
            let query: CString = CString::new(sql)
                .map_err(|_| AppError::from("read_only_database: query contains a null byte".to_string()))?;
            let mut raw_statement: *mut sqlite3_stmt = std::ptr::null_mut();

            let prepare_result: std::os::raw::c_int = unsafe {
                sqlite_wasm_rs::sqlite3_prepare_v2(self.handle, query.as_ptr(), -1, &mut raw_statement, std::ptr::null_mut())
            };

            if prepare_result != sqlite_wasm_rs::SQLITE_OK {
                let message: String = ffi_conversions::error_message(self.handle);

                return Err(AppError::from(format!("read_only_database: prepare failed; [error={message} sql={sql}]")));
            }

            let statement: Statement = Statement { handle: raw_statement };

            self.read_all_records(&statement)
        }

        fn read_all_records(&self, statement: &Statement) -> Result<Vec<SqlRecord>, AppError> {
            let column_count: std::os::raw::c_int = unsafe { sqlite_wasm_rs::sqlite3_column_count(statement.handle) };
            let mut records: Vec<SqlRecord> = Vec::new();

            loop {
                let step_result: std::os::raw::c_int = unsafe { sqlite_wasm_rs::sqlite3_step(statement.handle) };

                if step_result == sqlite_wasm_rs::SQLITE_DONE {
                    return Ok(records);
                }

                if step_result != sqlite_wasm_rs::SQLITE_ROW {
                    let message: String = ffi_conversions::error_message(self.handle);

                    return Err(AppError::from(format!("read_only_database: step failed; [error={message}]")));
                }

                let mut values: Vec<SqlValue> = Vec::with_capacity(column_count as usize);

                for column_index in 0..column_count {
                    values.push(ffi_conversions::column_value(statement.handle, column_index)?);
                }

                records.push(SqlRecord::new(values));
            }
        }
    }

    impl Drop for ReadOnlyDatabase {
        fn drop(&mut self) {
            unsafe { sqlite_wasm_rs::sqlite3_close(self.handle) };
            ro_memory_vfs::unregister_shard(&self.vfs_filename);
        }
    }

    /// Owns a prepared statement and finalizes it on drop, so every early return releases it.
    struct Statement {
        handle: *mut sqlite3_stmt,
    }

    impl Drop for Statement {
        fn drop(&mut self) {
            unsafe { sqlite_wasm_rs::sqlite3_finalize(self.handle) };
        }
    }
}
