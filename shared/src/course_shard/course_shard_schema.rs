/* ASCII "PTCR"; written to SQLite's `application_id` pragma so hex viewers and `file(1)` can identify the shards. */
pub const APPLICATION_ID: i32 = 0x5054_4352;

/* Written to SQLite's `user_version` pragma; bumped when a shard's tables change in a way a reader must detect. */
pub const SCHEMA_VERSION: i32 = 2;

const CLIP_TABLES_DDL: &str = include_str!("clip_tables.sql");
const COURSE_TABLES_DDL: &str = include_str!("course_tables.sql");
const UNIT_TABLES_DDL: &str = include_str!("unit_tables.sql");

pub fn course_shard_ddl() -> [&'static str; 2] {
    [CLIP_TABLES_DDL, COURSE_TABLES_DDL]
}

pub fn unit_shard_ddl() -> [&'static str; 2] {
    [CLIP_TABLES_DDL, UNIT_TABLES_DDL]
}
