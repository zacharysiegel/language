# Architecture: versioned SQLite artifacts in an object store

The course follows Eafora's architecture (`/Users/singularity/eafora/docs/architecture/overview.md` and `client.md`): a producer builds immutable, content-addressed SQLite shards and publishes them to an object store; every client is a reader of those files through the shared Rust core, and never calls an application server for course data.

## Data flow

```
data/units/*.json (authoring source, in git)
        |  course_builder build: validate, synthesize missing clips
        v
build/clips/<engine>/<clip_key>.m4a          clip store, one directory per speech engine
        |  course_builder build: write the bundle
        v
build/artifacts/<version>/
        manifest.json
        data/course-<sha256>.sqlite.br        levels, units, objectives, vocabulary, their clips
        data/unit-<id>-<sha256>.sqlite.br     one unit's lesson and its clips
        audio/<sha256>.m4a                    every recording the shards name (hard links into the clip store)
        |  course_builder publish local | cloudflare-r2 | dry
        v
repository:  audio/<sha256>.m4a
             <version>/manifest.json, <version>/data/...
             latest/manifest.1.json, latest/manifest.json (written last)
        v
clients: /discovery -> latest manifest -> course shard -> unit shards on demand -> recordings when played
```

## Artifact contract

- A version is a directory `<version>/` (a UTC timestamp, `YYYYMMDDTHHMMSSZ`) holding `manifest.json` and the shards. Shard names end in the SHA-256 of their bytes.
- Recordings are named by the SHA-256 of their bytes and sit outside every version, so a publish uploads only the ones the repository lacks. They are not stored in SQLite.
- `manifest.json` leads with `manifest_schema_version`, then `version`, `artifact_created`, the course shard, the unit shards by unit id, and each voice engine's pipeline revision. Paths are relative to the version directory. `shared::artifact::Manifest` is the canonical type for producer and clients.
- A shard is SQLite with `application_id` `PTCR` and `user_version` set to the shard schema version, compressed with brotli because neither R2 nor a static file server compresses a generic binary type. The manifest hash covers the compressed bytes.
- Every shard has a `clip` table (clip key, persona, spoken text) and a `clip_audio` table (clip key, engine, recording hash, size). The clip key is a hash of the persona and the text, so an unchanged sentence keeps its recordings across builds; the recording hash is what a client fetches.
- Headers: shards, recordings and versioned manifests are `public, max-age=31536000, immutable`; `latest/` pointers are `public, max-age=300`, and clients fetch them bypassing the HTTP cache.
- Publish order: recordings, shards, `<version>/manifest.json`, `latest/manifest.<schema>.json`, then `latest/manifest.json`. A client that reads the latest pointer finds every file it names.
- A client that cannot read the schema of `latest/manifest.json` falls back to `latest/manifest.<its schema>.json`.
- Discovery: the app's origin serves `/discovery` with `schema_version`, `repository_base_url`, `minimum_client_version` and `sunset`. The client requests the static repository's manifest (`/repository`) in parallel and reuses it when discovery agrees.

## Producer

- Authoring source stays JSON in git; there is no canonical database. The data is hand-authored and small, so a database would add nothing. This is the deliberate deviation from Eafora.
- `course_builder build` validates, synthesizes each engine's missing clips with the engines running at the same time, writes the bundle, refreshes `web/static/embedded/`, and keeps the two newest bundles.
- `course_builder publish` has three targets, as in Eafora: `local` copies into a directory (default `build/repository/`) and keeps the two newest versions and the recordings they name; `cloudflare-r2` uploads through the S3 API with credentials from `.env` and the secret access key from `secrets.yaml` (decrypted with `secr`); `dry` logs each upload.

## Consumer

- `shared` holds the client side of the contract: manifest and discovery types, verified fetch, the cache trait, brotli decoding, the shard reader, and a read-only in-memory SQLite (rusqlite on native targets, `sqlite-wasm-rs` with a read-only memory VFS on `wasm32`).
- Web: on start the app shows the version it cached last, if any, then the latest version when it differs. Shards are cached in the origin private file system by name, and the cache is trimmed to the latest manifest. With no cache and no repository, the embedded bundle opens. A unit's shard is fetched when the unit is visited. The Voice menu picks a recording per clip from `clip_audio`, falling back to the other engine, then to the browser's speech synthesis.
- iOS and Android can read the same artifacts through the same core; they are not built.
- Learner progress stays on the device.

## Hosting

- Local (the Mac mini): `cargo leptos serve --release` (or `serve.sh` for development) serves the app, `/discovery` and `build/repository/` at `/repository`. Nothing else is needed.
- R2: publish with `cloudflare-r2`, and set `PORTUGUESE_COURSE_REPOSITORY_BASE_URL` on the app's server to the bucket's public URL. The bucket needs a CORS rule allowing `GET` from the app's origin.

## Deviations from the original proposal

- One course shard (outline and vocabulary) replaced the proposed separate index and glossary shards; both pages need the same records.
- Shards are brotli-compressed, as Eafora's are.
- Recordings sit outside the version directories, keyed by hash, rather than under `audio/<engine>/` per version, so an unchanged recording is uploaded once.
- Versions are UTC timestamps rather than `YYYY-MM-DD+<label>`.
- `build` synthesizes; there is no separate `synthesize` command.
- The local repository is `build/repository/`, served by the app's server, rather than `web/static/repository/`.
- The embedded bundle holds every shard and no recordings, rather than three foundation units.
- The repository trait gained `existing_keys`, which Eafora's lacks, to skip recordings already published.
