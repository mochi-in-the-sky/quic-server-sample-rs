use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::PathBuf;
use tracing::*;

const ENV_PREFIX: &str = "THE_CATCHER_IN_THE_LIE_";
const DEFAULT_BIND_PORT: u16 = 7777;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_bind_port")]
    pub bind_port: u16,
    pub cert_path: Option<PathBuf>,
    pub key_path: Option<PathBuf>,
}

fn default_bind_port() -> u16 {
    DEFAULT_BIND_PORT
}

impl Config {
    pub fn new() -> Result<Self> {
        debug!("parse magic spells");
        let config = envy::prefixed(ENV_PREFIX).from_env::<Config>()?;
        if let Some(cert_path) = &config.cert_path {
            if !cert_path.exists() {
                return Err(anyhow!("cert_path not found"));
            }
        }
        if let Some(key_path) = &config.key_path {
            if !key_path.exists() {
                return Err(anyhow!("key_path not found"));
            }
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        std::env::set_var("THE_CATCHER_IN_THE_LIE_BIND_PORT", "7777");
        let config = Config::new().unwrap();
        assert_eq!(config.bind_port, 7777);

        std::env::set_var("THE_CATCHER_IN_THE_LIE_BIND_PORT", "160000000");
        let config = Config::new();
        assert!(config.is_err());

        std::env::remove_var("THE_CATCHER_IN_THE_LIE_BIND_PORT");
        let config = Config::new().unwrap();
        assert_eq!(config.bind_port, 7777);

        std::env::set_var("THE_CATCHER_IN_THE_LIE_CERT_PATH", "/tmp/oreore.cert");
        let config = Config::new();
        assert!(config.is_err());
        std::env::remove_var("THE_CATCHER_IN_THE_LIE_CERT_PATH");

        std::env::set_var("THE_CATCHER_IN_THE_LIE_KEY_PATH", "/tmp/oreore.key");
        let config = Config::new();
        assert!(config.is_err());
        std::env::remove_var("THE_CATCHER_IN_THE_LIE_KEY_PATH");

        std::env::set_var("THE_CATCHER_IN_THE_LIE_CERT_PATH", "tests/cert/oreore.cert");
        let config = Config::new();
        assert!(config.is_ok());
        std::env::remove_var("THE_CATCHER_IN_THE_LIE_CERT_PATH");

        std::env::set_var("THE_CATCHER_IN_THE_LIE_KEY_PATH", "tests/cert/oreore.key");
        let config = Config::new();
        assert!(config.is_ok());
        std::env::remove_var("THE_CATCHER_IN_THE_LIE_KEY_PATH");
    }
}
