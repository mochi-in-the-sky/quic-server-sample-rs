use anyhow::Result;
use futures::StreamExt;
use quinn::{Connecting, Endpoint, NewConnection, ServerConfig};
use tracing::*;

use super::setting::Setting;

pub struct Server {
    setting: Setting,
}

impl Server {
    pub fn new(setting: Setting) -> Self {
        Self { setting }
    }

    pub async fn listen(&self) -> Result<()> {
        let server_config =
            ServerConfig::with_single_cert(self.setting.cert.clone(), self.setting.key.clone())?;

        let (_, mut incoming) = Endpoint::server(server_config, self.setting.addr)?;

        info!("waiting for you");

        while let Some(conn) = incoming.next().await {
            info!("request is comming");
            tokio::spawn(async move {
                if let Err(e) = Self::handler(conn).await {
                    error!("connection is broken: {}", e);
                }
            });
        }

        Ok(())
    }

    async fn handler(conn: Connecting) -> Result<()> {
        info!("handler start");

        let NewConnection {
            connection,
            mut uni_streams,
            ..
        } = conn.await?;
        info!("connected from {}", connection.remote_address());

        if let Some(uni_streams) = uni_streams.next().await {
            let uni_stream = uni_streams?;
            let data = uni_stream.read_to_end(0xFF).await?;
            info!("received \"{}\"", String::from_utf8_lossy(&data));

            let mut send_stream = connection.open_uni().await?;
            send_stream.write(&data).await?;
            send_stream.finish().await?;
            connection.close(0u8.into(), &[]);
        } else {
            error!("cannot open uni stream");
        }

        info!("handler end");

        Ok(())
    }
}
