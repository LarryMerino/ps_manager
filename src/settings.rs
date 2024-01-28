use std::env;

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Host {
    url: String,
    key: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DebugConf {
    debug_lvl: String,
}

pub trait DebugConfProvider: std::fmt::Debug {
    fn get_debug_lvl(&self) -> String;
}

impl DebugConfProvider for DebugConf {
    fn get_debug_lvl(&self) -> String {
        self.debug_lvl.clone()
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    host: Host,
    debug: DebugConf,
}

pub trait SettingsProvider: std::fmt::Debug{
    fn get_conf_provider(&self) -> &dyn DebugConfProvider;
}

impl SettingsProvider for Settings {
    fn get_conf_provider(&self) -> &dyn DebugConfProvider {
        &self.debug
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "default".into());

        let conf = Config::builder()
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;
        conf.try_deserialize()
    }
}
