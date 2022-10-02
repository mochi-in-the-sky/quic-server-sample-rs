use anyhow::Result;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::{error::Error, fs::File, io::BufReader};
use tracing::*;

use super::config::Config;

pub struct Setting {
    pub addr: SocketAddr,
    pub cert: Vec<rustls::Certificate>,
    pub key: rustls::PrivateKey,
}

impl Setting {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        info!("start boot sequence");

        let addr = SocketAddr::from(([0, 0, 0, 0], config.bind_port));

        let (mut cert, mut key) = Self::generate_self_signed_cert()?;
        if config.cert_path.is_some() && config.key_path.is_some() {
            info!(
                "certificate existed: cert_path: {:?}: key_path: {:?}",
                config.cert_path, config.key_path
            );
            (cert, key) =
                Self::generate_cert_from_file(config.cert_path.unwrap(), config.key_path.unwrap())?;
        }

        Ok(Self { addr, cert, key })
    }

    fn generate_self_signed_cert(
    ) -> Result<(Vec<rustls::Certificate>, rustls::PrivateKey), Box<dyn Error>> {
        let cert = rcgen::generate_simple_self_signed(vec!["0, 0, 0, 0".to_string()])?;
        let key = rustls::PrivateKey(cert.serialize_private_key_der());
        Ok((vec![rustls::Certificate(cert.serialize_der()?)], key))
    }

    fn generate_cert_from_file(
        cert_path: PathBuf,
        key_path: PathBuf,
    ) -> Result<(Vec<rustls::Certificate>, rustls::PrivateKey), Box<dyn Error>> {
        info!("regular route confirmed");

        let mut cert_chain_reader = BufReader::new(File::open(cert_path)?);
        let certs = rustls_pemfile::certs(&mut cert_chain_reader)?
            .into_iter()
            .map(rustls::Certificate)
            .collect();

        let mut key_reader = BufReader::new(File::open(key_path)?);
        let mut keys = rustls_pemfile::rsa_private_keys(&mut key_reader)?;

        assert_eq!(keys.len(), 1);
        let key = rustls::PrivateKey(keys.remove(0));

        Ok((certs, key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        std::env::set_var("THROWSTERHOUSE_FIVE_BIND_PORT", "7777");
        let config1 = Config::new().unwrap();
        let setting1 = Setting::new(config1.clone()).unwrap();
        assert_eq!(Ok(setting1.addr), "0.0.0.0:7777".parse());

        std::env::set_var("THROWSTERHOUSE_FIVE_CERT_PATH", "tests/cert/oreore.cert");
        std::env::set_var("THROWSTERHOUSE_FIVE_KEY_PATH", "tests/cert/oreore.key");
        let config2 = Config::new().unwrap();
        let setting2 = Setting::new(config2.clone()).unwrap();

        let (cert, key) =
            Setting::generate_cert_from_file(config2.cert_path.unwrap(), config2.key_path.unwrap())
                .unwrap();
        assert_eq!(cert, setting2.cert);
        assert_eq!(key, setting2.key);

        assert_ne!(setting1.cert, setting2.cert);
        assert_ne!(setting1.key, setting2.key);
    }
}
