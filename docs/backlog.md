# Backlog

Known gaps, recorded when found.

## Web app

- Settings are controls in the navigation bar; the web app needs a settings page, as the iOS and Android apps have.
- Open-response grading runs only in browsers that get wllama's default build (Chromium). wllama's compatibility build (Safari) cannot read a GGUF file holding more than 4 GiB of tensor data, so it needs the split model, and its WebGPU queue times out after 30 s on a 512-token prompt batch, so it needs 128-token batches; with both it grades, but reads the prompt in about 50 s.

## Native macOS app

- Plan and build a native macOS app that grades open responses with cross-llama's llama.cpp backend (Metal), free of the browser limits: the 4 GiB WebAssembly memory of wllama's compatibility build, browser storage quotas, and a per-profile model download. It needs native model download management, which cross-llama leaves to the app.

## iOS app

- On iPhone a unit's tab row scrolls sideways, so the Practice tab starts off-screen.

## History

- Commit `0dd4266 (language)` does not compile on its own: its `main.rs` references the `set-artifact-cors` command, which arrives in the next commit, `dcbb42b (language)`. Every later commit builds.
