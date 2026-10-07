use crate::AppError;

/* SQLite writes both values at fixed offsets in a database file's header, so a shard is checked before any
   handle is opened and both targets run the same check. */
const SCHEMA_VERSION_OFFSET: usize = 60;
const APPLICATION_ID_OFFSET: usize = 68;

/// Confirms `bytes` are a shard with the expected `application_id` and `user_version` pragmas, before any query.
pub fn validate_shard_header(bytes: &[u8], expected_application_id: i32, expected_schema_version: i32) -> Result<(), AppError> {
    let application_id: i32 = read_header_i32(bytes, APPLICATION_ID_OFFSET)?;

    if application_id != expected_application_id {
        return Err(AppError::from(format!(
            "sqlite shard: application_id mismatch; [found={application_id:#x} expected={expected_application_id:#x}]",
        )));
    }

    let schema_version: i32 = read_header_i32(bytes, SCHEMA_VERSION_OFFSET)?;

    if schema_version != expected_schema_version {
        return Err(AppError::from(format!(
            "sqlite shard: schema_version {schema_version} {}; [expected={expected_schema_version}]",
            describe_version_direction(schema_version, expected_schema_version),
        )));
    }

    Ok(())
}

/* A stale shard and a stale reader both produce a mismatch, and their remedies are opposite. */
pub fn describe_version_direction(found: i32, expected: i32) -> &'static str {
    if found < expected {
        "predates this build and must be rebuilt"
    } else {
        "comes from a newer build, so this one is out of date"
    }
}

fn read_header_i32(bytes: &[u8], offset: usize) -> Result<i32, AppError> {
    let field: Option<&[u8]> = bytes.get(offset..offset + 4);

    match field {
        Some(field) => Ok(i32::from_be_bytes([field[0], field[1], field[2], field[3]])),
        None => Err(AppError::from(format!("sqlite shard: too short to hold a header; [bytes={}]", bytes.len()))),
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    /* Serialized rather than hand-built, so the offsets under test are the ones SQLite itself writes. */
    fn shard_bytes_with_header(application_id: i32, schema_version: i32) -> Vec<u8> {
        let connection: rusqlite::Connection = rusqlite::Connection::open_in_memory().unwrap();
        connection.execute_batch("create table placeholder (value integer);").unwrap();
        connection.pragma_update(None, "application_id", application_id).unwrap();
        connection.pragma_update(None, "user_version", schema_version).unwrap();

        let data: rusqlite::serialize::Data<'_> = connection.serialize(rusqlite::DatabaseName::Main).unwrap();

        data.to_vec()
    }

    #[test]
    fn validate_shard_header_accepts_matching_ids() {
        let bytes: Vec<u8> = shard_bytes_with_header(0x1234, 3);

        validate_shard_header(&bytes, 0x1234, 3).unwrap();
    }

    #[test]
    fn validate_shard_header_rejects_a_foreign_application_id() {
        let bytes: Vec<u8> = shard_bytes_with_header(0x9999, 3);

        let error: AppError = validate_shard_header(&bytes, 0x1234, 3).unwrap_err();

        assert!(error.to_string().contains("application_id mismatch"));
    }

    #[test]
    fn validate_shard_header_names_the_direction_of_a_version_mismatch() {
        let older: Vec<u8> = shard_bytes_with_header(0x1234, 2);
        let newer: Vec<u8> = shard_bytes_with_header(0x1234, 4);

        assert!(validate_shard_header(&older, 0x1234, 3).unwrap_err().to_string().contains("predates this build"));
        assert!(validate_shard_header(&newer, 0x1234, 3).unwrap_err().to_string().contains("newer build"));
    }

    #[test]
    fn validate_shard_header_rejects_bytes_too_short_for_a_header() {
        assert!(validate_shard_header(b"short", 0x1234, 3).is_err());
    }
}
