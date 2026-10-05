# Português do Brasil

A Brazilian Portuguese course for English speakers, from A1 to B2: 42 units of vocabulary, dialogues, grammar, readings and exercises, with spoken audio for every Portuguese sentence and a spaced-repetition review deck. Progress is kept in the browser.

## Run it

```sh
./scripts/dev/serve.sh
```

That compiles the course data and synthesizes the audio on the first run (about 5,500 clips with the macOS Brazilian Portuguese voices and `ffmpeg`, roughly 20 minutes), then serves http://127.0.0.1:3000 with `cargo leptos watch`.

## Layout

- `data/curriculum.json` lists the units; `data/units/<id>.json` holds one unit each. The format is described in `docs/authoring.md`.
- `shared/` has the course model and its validation, answer checking, review schedule and progress model.
- `tools/course_builder/` validates the data, compiles it and synthesizes the audio.
- `web/` is the Leptos app.

## Commands

```sh
cargo run --package course_builder -- validate --all
./scripts/build/build-course.sh
cargo test
```
