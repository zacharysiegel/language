# Backlog

Known gaps, recorded when found.

## Web app

- Settings are controls in the navigation bar; the web app needs a settings page, as the iOS and Android apps have.

## iOS app

- On iPhone a unit's tab row scrolls sideways, so the Practice tab starts off-screen.

## History

- Commit `0dd4266 (language)` does not compile on its own: its `main.rs` references the `set-artifact-cors` command, which arrives in the next commit, `dcbb42b (language)`. Every later commit builds.
