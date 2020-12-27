use serde::{Deserialize};


#[derive(Deserialize, Clone)]
pub struct Config {
    pub serving_layer_port: Option<String>,
    pub serving_layer_ip: Option<String>,
    pub calculate_adress: String,
}
pub fn get_config() -> Config {
    let config_str = include_str!("../Config.toml");
    let config: Config = toml::from_str(&config_str).unwrap();
    config
}
