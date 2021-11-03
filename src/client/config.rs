use std::{default::Default, env, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    servers: Option<Vec<Server>>,
    pools: Option<Vec<Pool>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    address: String,
    port: Option<u16>,
    ntske_server: Option<Vec<Server>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    address: String,
    port: Option<u16>,
}
const DEFAULT_NTP_PORT: u16 = 123;
const DEFAULT_NTP_SERVER_HOST: &str = "127.0.0.1";

impl Default for ClientConfig {
    fn default() -> Self {
        let default_server = Server {
            address: DEFAULT_NTP_SERVER_HOST.to_string(),
            port: Some(DEFAULT_NTP_PORT),
            ntske_server: None
        };

        Self {
            servers: Some(vec![default_server]),
            pools: None
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {

    /// Load config from default location(s)
    // TODO: figure out best practices
    pub fn load() {
    }

    /// Load config from specific path
    pub fn load_from(path: &String) -> Result<ClientConfig, rustntp::Error> {
        return Ok(ClientConfig::default());
    }

    pub fn load_from_or_default(path: &String) -> Result<ClientConfig, rustntp::Error> {
        let config_path = Path::new(path);
        let loaded_config = if !config_path.exists() { Ok(ClientConfig::default()) } else { Self::load_from(path) };
        
        loaded_config
    }

    // Used to write/update a config to default config location
    pub fn write_config(config: ClientConfig) {

    }

}
