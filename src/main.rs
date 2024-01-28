use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::{debug, info};

use crate::settings::{Settings, SettingsProvider};
pub mod settings;

// Check this for some inspiration
// https://github.com/JaynewDee/portform/blob/866727ad3cd6ff499844d54b673472f646c95a40/src/generate.rs#L4

fn main() {
    let settings = Settings::new().unwrap();
    let logger = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(settings.get_conf_provider().get_debug_lvl()),
    )
    .build();

    let multi = MultiProgress::new();
    LogWrapper::new(multi.clone(), logger).try_init().unwrap();

    info!("Logger initialized");
    debug!("{:#?}", settings);

    info!("Finish Job");
}
