use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config/Default";

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?
            .try_deserialize::<AppConfig>()
    }
}