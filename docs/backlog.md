# Backlog

Known gaps, recorded when found.

## iOS app

- Running on a physical device needs a signing team (`DEVELOPMENT_TEAM`) in `ios/project.yml` and `PORTUGUESE_COURSE_ORIGIN` set to the serving Mac's local network address.
- The app checks for a newer course version only at launch, on pull-to-refresh on the course page, and from Settings.
- On iPhone a unit's tab row scrolls sideways, so the Practice tab starts off-screen.
- The UI tests take about 6 minutes and answer one unit's practice and one reading's questions, rather than every unit as the web regression script does.

## History

- Commit `0dd4266 (portuguese-course)` does not compile on its own: its `main.rs` references the `set-artifact-cors` command, which arrives in the next commit, `dcbb42b (portuguese-course)`. Every later commit builds.
