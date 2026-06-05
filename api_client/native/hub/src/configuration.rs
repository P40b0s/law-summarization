use serde::Deserialize;
use figment::{Figment, providers::{Env, Format, Toml}};


#[derive(Deserialize)]
pub struct Configuration 
{
    #[serde(default = "service_addresse")]
    pub service_addresse: String,
    #[serde(default = "service_port")]
    pub service_port: u16,
}

fn service_addresse() -> String
{
    "127.0.0.1".to_owned()
}
fn service_port() -> u16
{
    8081
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