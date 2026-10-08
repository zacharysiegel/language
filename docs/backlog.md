# Backlog

Known gaps, recorded when found.

## iOS app

- Running on a physical device needs a signing team (`DEVELOPMENT_TEAM`) in `ios/project.yml` and `LANGUAGE_ORIGIN` set to the serving Mac's local network address.
- On iPhone a unit's tab row scrolls sideways, so the Practice tab starts off-screen.

## History

- Commit `0dd4266 (language)` does not compile on its own: its `main.rs` references the `set-artifact-cors` command, which arrives in the next commit, `dcbb42b (language)`. Every later commit builds.
