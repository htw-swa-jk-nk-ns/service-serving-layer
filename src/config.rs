use serde::Deserialize;
use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub calculate_adress: String,
    pub raw_data_adress: String,
}

pub fn get_config() -> Settings {
    Settings::new()
}

impl Settings {

    pub fn new() -> Self {
        Self {
            calculate_adress: "service-calculate:8889".to_owned(),
            raw_data_adress: "service-raw-data:8889".to_owned(),
        }
    }
    #[allow(dead_code)]
    pub fn new_old() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("Config"))?;

        s.merge(Environment::new())?;

        s.try_into()
    }
}
