use serde::{Deserialize, Serialize};
use std::{default::Default, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    pub servers: Option<Vec<Server>>,
    pub pools: Option<Vec<Pool>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub address: String,
    pub port: Option<u16>,
    pub ntske_server: Option<NTSKeyExchangeServer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NTSKeyExchangeServer {
    pub address: Option<String>,
    pub port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    pub address: String,
    pub port: Option<u16>,
}

const DEFAULT_NTP_PORT: u16 = 123;
const DEFAULT_NTP_SERVER_HOST: &str = "localhost";

impl Default for ClientConfig {
    fn default() -> Self {
        let default_server = Server {
            address: DEFAULT_NTP_SERVER_HOST.to_string(),
            port: Some(DEFAULT_NTP_PORT),
            ntske_server: None,
        };

        Self {
            servers: Some(vec![default_server]),
            pools: None,
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    /// Load config from default location(s)
    // TODO: figure out best practices
    pub fn load() {}

    /// Load config from specific path
    pub fn load_from(path: &String) -> Result<ClientConfig, rustntp::Error> {
        return Ok(ClientConfig::default());
    }

    pub fn load_from_or_default(path: &String) -> Result<ClientConfig, rustntp::Error> {
        let config_path = Path::new(path);
        let loaded_config = if !config_path.exists() {
            Ok(ClientConfig::default())
        } else {
            Self::load_from(path)
        };

        loaded_config
    }

    // Used to write/update a config to default config location
    pub fn write_config(config: ClientConfig) {}
}
