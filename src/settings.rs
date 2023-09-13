use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    url: String,
    token: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Add as many configurations as you need within the 'config' folder
        // It's recommended to use environment variables or other methods to add sensitive data
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        log::info!("run_mode: {}", run_mode);

        Config::builder()
            .add_source(File::with_name(&format!("src/config/{}", run_mode)))
            .build()
            .unwrap()
            .try_deserialize()
    }
}
