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
        <p>"This policy applies to Plainspoken's apps and website."</p>

        <section class="policy-section">
            <h2>"Your data"</h2>
            <p>
                "What you create in Plainspoken, such as your progress, answers and settings, is stored on your device or in your browser. We do not receive it, and we never sell it or use it for advertising."
            </p>
            <p>
                "You can export your data and import it elsewhere. Clearing the app's or site's data, or uninstalling the app, deletes it. When your device's or browser's own backup or sync is turned on, it may include your data, under that provider's terms."
            </p>
        </section>

        <section class="policy-section">
            <h2>"Downloads and service providers"</h2>
            <p>
                "Plainspoken downloads course material and the files its features need, such as recordings and models, from our servers and from third-party hosting and delivery providers. These requests do not include your data. As with any internet request, the providers receive standard technical information, such as your IP address and device or browser type, which they use to deliver the files and protect their services."
            </p>
            <p>"Services built into your device or browser, such as text-to-speech voices and backups, follow their provider's terms."</p>
        </section>

        <section class="policy-section">
            <h2>"Analytics and cookies"</h2>
            <p>
                "We may use analytics that count visits and usage in aggregate. They do not identify you or follow you across other sites. Plainspoken uses no advertising or tracking cookies."
            </p>
        </section>

        <section class="policy-section">
            <h2>"Children"</h2>
            <p>"Plainspoken is not directed at children under 13 and does not knowingly collect information from children."</p>
        </section>

        <section class="policy-section">
            <h2>"Changes and contact"</h2>
            <p>
                "If this policy changes, the new policy will appear on this page with a new effective date. Questions: "
                <a href=format!("mailto:{CONTACT_EMAIL}")>{CONTACT_EMAIL}</a>
                "."
            </p>
        </section>
    }
}
