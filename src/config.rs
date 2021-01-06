use serde::Deserialize;
use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub serving_layer_port: Option<String>,
    pub serving_layer_ip: Option<String>,
    pub calculate_adress: String,
}

pub fn get_config() -> Settings {
    Settings::new().unwrap()
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("Config"))?;

        s.merge(Environment::new())?;

        s.try_into()
    }
}
