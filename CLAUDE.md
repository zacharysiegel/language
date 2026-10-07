# CLAUDE.md

A Brazilian Portuguese course (A1 to B2) for English speakers. A Rust workspace: `shared` (course model, validation, practice and review logic), `tools/course_builder` (validate, synthesize audio, write and publish SQLite bundles) and `web` (Leptos, server render plus client hydration, run with `cargo leptos watch`).

## Conventions

Follow `docs/conventions/` (type naming, log format, conditional compilation) and the owner's global `~/.claude/CLAUDE.md`. Scripts in `scripts/` wrap the tools; use them.

## Visual design

Follow `docs/design.md` (Eafora's identity): tokens in `web/style/_tokens.scss`, no raw colors or sizes elsewhere, no shadows, no rounded corners, no animation.

## Course data

- Author units to the format in `docs/authoring.md`, then run `cargo run --package course_builder -- validate <unit-id>`. The validator is the authority on structure; `shared/tests/course_data.rs` fails `cargo test` if any unit is invalid.
- Every `pt` string is spoken by text-to-speech, so it must be plain speakable text.
- Brazilian Portuguese only. European forms in `pt` fields are rejected by the validator.
- `build/` (clip store, bundles, local repository) and `web/static/embedded/` are generated and not committed.

## Running locally

`./scripts/dev/serve.sh` needs only this machine. `course_builder build` synthesizes every clip with each engine (Kokoro through ONNX Runtime and eSpeak NG, with model files downloaded once into `models/`, and the macOS `say` voices), encodes with `ffmpeg` into `build/clips/<engine>/`, and writes a bundle to `build/artifacts/<version>/`. A clip's key depends only on its persona and text; a change to an engine's pipeline bumps `pipeline_revision`, which discards that engine's clips. `publish local` copies the bundle into `build/repository/`, which the web server serves at `/repository`.

## Architecture

`docs/architecture/overview.md` describes the artifact contract (shards, manifest, publish order, discovery, client loading). The shard schema is `shared/src/course_shard/*.sql`; bump `SCHEMA_VERSION` in `course_shard_schema.rs`, and `MANIFEST_SCHEMA_VERSION` in `shared/src/artifact/manifest.rs`, for a change a reader must detect.
