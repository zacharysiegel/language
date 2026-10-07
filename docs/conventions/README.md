# Coding conventions

The conventions follow the Eafora project's; these are copies of the ones that apply here.

- [`types.md`](types.md) - type naming (Model + wire pairs, `Kind` enum suffix, `TryFrom<&str>` parsing).
- [`logging.md`](logging.md) - log message format: `<message>; [key=value ...]`.
- [`conditional-compilation.md`](conditional-compilation.md) - gate a block of target or feature-specific items in one `#[cfg]`-ed submodule, with a one-line WHY on each gate.

The course data types in `shared::course` are serialized directly to and from JSON. Shards are read record by record (`shared::sqlite::SqlRecord`) straight into those types, so the `Entity` and `Projection` wire types never appear.
