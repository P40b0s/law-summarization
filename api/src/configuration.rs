use serde::Deserialize;
use summarization_core::CoreConfiguration;
use figment::{Figment, providers::{Env, Format, Toml}};




#[derive(Deserialize)]
pub struct Configuration {
    #[serde(default = "origins")]
    pub origins: Vec<String>,
    #[serde(default = "port")]
    pub server_port: u16,
    #[serde(default = "core_configuration")]
    pub core_configuration: CoreConfiguration,
}

fn origins() -> Vec<String>
{
    vec![
                "http://localhost:8080".to_owned(),
                "http://localhost:8081".to_owned(),
                "http://localhost:80".to_owned(),
            ]
}
fn port() -> u16
{
    8081
}
fn core_configuration() -> CoreConfiguration
{
    CoreConfiguration::default()
}

impl Configuration {
    pub fn new() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Toml::file("/config/config.toml"))
            .merge(Env::prefixed("APP_"))
            .extract()
    }
}