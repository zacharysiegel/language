# Português do Brasil

A Brazilian Portuguese course for English speakers, from A1 to B2: 44 units that teach the principles first (sounds and spelling, stress and accents, the structure of the language, then grammar), with dialogues, readings, exercises and vocabulary, with spoken audio for every Portuguese sentence and a spaced-repetition review deck. Progress is kept in the browser.

## Run it

```sh
./scripts/dev/serve.sh
```

On the first run that builds the course and publishes it to the local repository (`build/repository/`), then serves http://127.0.0.1:3000 with `cargo leptos watch`. The audio is spoken by the Kokoro neural voices, run locally through ONNX Runtime with eSpeak NG for the phonemes, and by the macOS Brazilian Portuguese voices; the two engines synthesize at the same time. The first run downloads pinned model files into `models/` (checked against checksums) and synthesizes every clip with each engine, which takes tens of minutes; later runs synthesize only the clips that changed. It needs `cmake` (eSpeak NG is built from source), `ffmpeg`, and Apple silicon.

The Voice menu chooses between the two recording sets: Auto (the macOS voice for clips of one or two words, where the neural voice is unreliable, and the neural voice for longer ones), Neural, or macOS. `./scripts/build/build-course.sh --engine macos-say` synthesizes only the macOS set, which needs no download.

## How the data flows

- `course_builder build` validates `data/`, synthesizes missing clips into `build/clips/<engine>/`, and writes a bundle to `build/artifacts/<version>/`: a `manifest.json`, one brotli-compressed SQLite shard for the course outline and one per unit (`data/*.sqlite.br`, named by their SHA-256), and the recordings (`audio/<sha256>.m4a`). It also copies the manifest and shards to `web/static/embedded/`, the bundle the app falls back to when no repository answers.
- `course_builder publish local | cloudflare-r2 | dry` uploads the bundle: recordings the repository lacks, then the shards, then `<version>/manifest.json`, and `latest/manifest.json` last.
- The app reads `/discovery` for the repository's address, fetches `latest/manifest.json`, opens the course shard, and opens a unit's shard when the unit is visited. Every download is checked against its hash and cached in the browser, so the app opens offline once visited.

## Publishing to Cloudflare R2

Copy `template.env` to `.env` and fill in the R2 settings. Put the publish token's secret access key in `secrets.yaml`, encrypted with your master key:

```sh
secr encrypt --key "$MASTER_SECRET" --name cloudflare.r2.publish.secret_access_key <secret-access-key>
```

Then run `./scripts/publish/publish-course.sh cloudflare-r2`. To serve the app against R2, set `PORTUGUESE_COURSE_REPOSITORY_BASE_URL` to the bucket's public URL; the bucket needs a CORS rule that allows `GET` from the app's origin.

## Layout

- `data/curriculum.json` lists the units; `data/units/<id>.json` holds one unit each. The format is described in `docs/authoring.md`.
- `shared/` has the course model and its validation, the shard schema and reader, the artifact contract (manifest, discovery, verified fetch and cache), answer checking, the review schedule and the progress model.
- `tools/course_builder/` validates the data, synthesizes the audio, writes bundles, and publishes them.
- `web/` is the Leptos app and the server that hosts it, `/discovery`, and the local repository.

## Commands

```sh
cargo run --package course_builder -- validate --all
./scripts/build/build-course.sh
./scripts/publish/publish-course.sh local
cargo test
```
