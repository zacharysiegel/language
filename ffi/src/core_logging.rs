const CORE_LOG_TAG: &str = "language_core";

/// Sends the core's log lines to logcat. Later calls do nothing.
#[uniffi::export]
pub fn initialize_core_logging() {
    let config: android_logger::Config = android_logger::Config::default()
        .with_tag(CORE_LOG_TAG)
        .with_max_level(log::LevelFilter::Info);

    android_logger::init_once(config);
}
