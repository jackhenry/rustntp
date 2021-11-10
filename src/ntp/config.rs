use std::fs;
use std::io;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct NTPConfig {
    port: i64,
}

impl Default for NTPConfig {
    fn default() -> Self {
        Self { port: 9002 }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn load(&self, config_path: &String) -> Result<NTPConfig, io::ErrorKind> {
        let path = Path::new(config_path);

        if !path.exists() {
            return Err(io::ErrorKind::NotFound);
        }

        let config_str = fs::read_to_string(config_path).unwrap();
        let config_result = toml::from_str(config_str.as_str());

        if config_result.is_err() {
            return Err(io::ErrorKind::InvalidData);
        }
        let config: NTPConfig = config_result.unwrap();
        Ok(config)
    }

    pub fn load_config(&self, config_path: Option<String>) -> NTPConfig {
        if config_path != None {
            let config_path = config_path.unwrap();
            let load_result = self.load(&config_path);
            if let Err(load_error) = load_result {
                tracing::error!("Unable to load config from path: \"{}\"", config_path);
                panic!("{:?}", load_error);
            } else {
                return load_result.unwrap();
            }
        };

        // check for existing configs at /etc/rustntp.toml
        let load_from_etc_result = self.load(&String::from("/etc/rustntp.toml"));
        if let Ok(config) = load_from_etc_result {
            return config;
        };

        return NTPConfig::default();
    }
}
