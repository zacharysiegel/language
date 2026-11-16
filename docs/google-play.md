# Google Play

The Android app is published as Plainspoken under the personal developer account "zach.ro", with the permanent package name `ro.zach.language`. Builds are signed with the upload key in `~/.android/language-release.jks` (alias `language`); Play App Signing holds the key that signs installed apps. `cd android && ./gradlew bundleRelease` writes `app/build/outputs/bundle/release/app-release.aab`, with the signing properties in `~/.gradle/gradle.properties`.

## Production access for a personal account

Personal accounts created after November 13, 2023 cannot publish to production or offer pre-registration until both of these are done ([Google's requirements](https://support.google.com/googleplay/android-developer/answer/14151465)):

- A closed test with at least 12 testers opted in, each continuously for the 14 days before the application. A tester who opts out and back in restarts their 14 days. Tell testers to stay opted in.
- An application from the Dashboard ("Apply for production"), answering:
  - The closed test: how easy testers were to recruit, whether they used every feature, how their use differed from expected production use, and a summary of their feedback and how it was collected.
  - The app: target audience, the value it offers, and the expected installs in the first year. These answers are not public.
  - Readiness: what changed after the closed test, and how readiness was decided.

Google usually reviews the application within seven days and emails the account owner. With fewer than 12 opted-in testers or too little engagement, the closed test has to continue.

Internal testing has no requirements and reaches testers within seconds. Open testing becomes available only with production access. A closed test can start only after the app's setup in Play Console is complete.

## Declarations that depend on the app

Play Console's App content section records these answers. Update them in the same change that makes them false.

- App access: no part of the app is restricted. Adding sign-in or any other restriction requires instructions and test credentials for Google's reviewers.
- Data safety: no user data is collected or shared. Adding accounts, analytics, crash reporting or any upload of progress changes this.
- Ads: none.
- Target audience: 13 and over.
