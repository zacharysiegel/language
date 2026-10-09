# Backlog

Known gaps, recorded when found.

## iOS app

- Running on a physical device needs `LANGUAGE_ORIGIN` set to the serving Mac's local network address and the web server bound beyond loopback (`LEPTOS_SITE_ADDR=0.0.0.0:3000`).
- On iPhone a unit's tab row scrolls sideways, so the Practice tab starts off-screen.

## History

- Commit `0dd4266 (language)` does not compile on its own: its `main.rs` references the `set-artifact-cors` command, which arrives in the next commit, `dcbb42b (language)`. Every later commit builds.
