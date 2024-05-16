use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}
