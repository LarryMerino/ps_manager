use std::env;

use config::{Config, ConfigError, File};
use ps_manager::data_fetcher::api_client::PsApiCredentialsProvider;
use serde_derive::Deserialize;

/// Represent
#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Credentials {
    host: String,
    key: String,
}

pub trait CredentialsProvider {
    fn get_api_credential_provider(&self) -> impl PsApiCredentialsProvider;
}

impl CredentialsProvider for Credentials {
    fn get_api_credential_provider(&self) -> impl PsApiCredentialsProvider {
        Credentials {
            host: self.host.clone(),
            key: self.key.clone(),
        }
    }
}

impl PsApiCredentialsProvider for Credentials {
    fn get_host(&self) -> String {
        self.host.clone()
    }

    fn get_raw_key(&self) -> String {
        self.key.clone()
    }
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
    credentials: Credentials,
    debug: DebugConf,
}

pub trait SettingsProvider: std::fmt::Debug {
    fn debug_conf_provider(&self) -> &impl DebugConfProvider;
    fn api_credential_provider(&self) -> &impl PsApiCredentialsProvider;
}

impl SettingsProvider for Settings {
    fn debug_conf_provider(&self) -> &impl DebugConfProvider {
        &self.debug
    }

    fn api_credential_provider(&self) -> &impl PsApiCredentialsProvider {
        &self.credentials
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
