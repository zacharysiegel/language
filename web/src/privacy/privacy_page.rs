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
        <p>
            "Plainspoken teaches languages through courses in its Android and iOS apps and on the web at language.zach.ro. It has no accounts and no advertising, and it collects no information that identifies you."
        </p>

        <section class="policy-section">
            <h2>"Your progress"</h2>
            <p>
                "Your progress (unit status, practice results and answers, your review deck and your settings) is stored on your device and is never sent to us. The Progress page exports it as text, which you can import on another device, and resets it."
            </p>
            <h3>"Android"</h3>
            <p>
                "Progress is a file in the app's private storage. When Android backup or device-to-device transfer is turned on, it is included, under your Google account and Google's terms. Clearing the app's data or uninstalling the app deletes it. Exporting also copies the text to the clipboard."
            </p>
            <h3>"iOS"</h3>
            <p>
                "Progress is a file in the app's private storage. When iCloud Backup or a computer backup is turned on, it is included, under Apple's terms. Deleting the app deletes it. Exporting also copies the text to the clipboard, which Universal Clipboard can share with your other Apple devices."
            </p>
            <h3>"Web"</h3>
            <p>
                "The site keeps what it needs to work in your browser: your progress and settings in local storage, and downloaded course content in the browser's private file storage for the site. It sets no advertising or tracking cookies. Clearing the site's data in your browser deletes it all."
            </p>
        </section>

        <section class="policy-section">
            <h2>"What is downloaded"</h2>
            <p>
                "Course content and recordings are downloaded from language.zach.ro and repository.language.zach.ro, which Cloudflare serves. The requests carry nothing about you or your progress. Like any web server, Cloudflare receives the standard information of each request, such as your IP address and device or browser type, and processes it to deliver the files and protect the service. We do not use it to identify you."
            </p>
            <h3>"Web"</h3>
            <p>
                "To grade written answers, the site downloads an AI model from huggingface.co, which Hugging Face runs and which receives the same standard request information. The model runs in your browser, so your answers never leave your device. You can turn the download off in the site's settings."
            </p>
        </section>

        <section class="policy-section">
            <h2>"Speech"</h2>
            <p>"When a recording is missing, the device's text-to-speech reads the course text aloud. It is given only course text, never anything you write."</p>
            <h3>"Android"</h3>
            <p>"The text-to-speech engine chosen in Android's settings is used. Some engines use online voices, which send the text to their provider."</p>
            <h3>"iOS"</h3>
            <p>"Speech is produced on the device."</p>
            <h3>"Web"</h3>
            <p>"Your browser's speech synthesis is used. Some browsers' voices run on their provider's servers, which receive the text."</p>
        </section>

        <section class="policy-section">
            <h2>"Analytics"</h2>
            <p>"The Android and iOS apps contain no analytics."</p>
            <h3>"Web"</h3>
            <p>
                "The site may use Cloudflare Web Analytics to count visits and page views. It sets no cookies, does not follow you across other sites and does not identify you."
            </p>
        </section>

        <section class="policy-section">
            <h2>"Children"</h2>
            <p>"Plainspoken is not directed at children under 13 and does not knowingly collect information from anyone."</p>
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
