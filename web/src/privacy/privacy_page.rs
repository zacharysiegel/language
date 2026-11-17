use leptos::prelude::*;

pub const PRIVACY_SEGMENT: &str = "privacy";

const CONTACT_EMAIL: &str = "hello@zach.ro";

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <div class="course-title">
            <h1>"Privacy policy"</h1>
            <span class="page-caption">"Effective 16 November 2026"</span>
        </div>
        <p>"This policy applies to every Plainspoken application, on every platform."</p>

        <section class="policy-section">
            <h2>"Your data"</h2>
            <p>
                "Information you provide or information generated through your use of Plainspoken, such as progress, answers or settings, is stored on your device. We do not store it on our servers. We do not sell or otherwise distribute it for advertising or any other third-party use case."
            </p>
            <p>
                "Your data on your own device is yours to handle as you please. Your data is deleted when the application's data is cleared. Your device has its own terms for how it handles user data stored on it; this policy does not cover the impact of your device or web browser on your data."
            </p>
        </section>

        <section class="policy-section">
            <h2>"Downloads and service providers"</h2>
            <p>
                "Plainspoken downloads application data necessary for its operation, such as audio recordings and computational models, from our servers and from third-party hosting services. These are not requests for user data. As with any Internet request, standard technical information is required in order to fulfill the request; this may include identifying information, such as IP addresses and information about your device."
            </p>
            <p>"Services provided by your device follow the device provider's privacy policy."</p>
        </section>

        <section class="policy-section">
            <h2>"Analytics and cookies"</h2>
            <p>
                "We may use analytics to measure aggregate usage of the application. Analytics cannot be used to retroactively identify you or track you across other applications or platforms. The application stores no cookies or other data on your device beyond what it needs to function."
            </p>
        </section>

        <section class="policy-section">
            <h2>"Children"</h2>
            <p>"Plainspoken is not directed at children under 13 and does not knowingly collect information from children."</p>
        </section>

        <section class="policy-section">
            <h2>"Changes and contact"</h2>
            <p>
                "If this policy changes, the new policy will appear on this page with a new effective date. Direct any questions to "
                <a href=format!("mailto:{CONTACT_EMAIL}")>{CONTACT_EMAIL}</a>
                "."
            </p>
        </section>
    }
}
