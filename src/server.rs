use anyhow::Result;
use futures::StreamExt;
use quinn::{Connecting, Endpoint, RecvStream, SendStream, ServerConfig};
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
        let (sender, reciever) = conn.await?.connection.open_bi().await?;

        let (ret_recv, ret_send) = tokio::join!(Self::recv(reciever), Self::send(sender));
        if let Err(e) = ret_recv {
            error!("catcher failed: {:?}", e);
        }
        if let Err(e) = ret_send {
            error!("pitcher failed: {:?}", e);
        }

        Ok(())
    }

    async fn recv(reciever: RecvStream) -> Result<()> {
        let received = reciever.read_to_end(10).await?;
        info!("catch: {:?}", received);
        Ok(())
    }

    async fn send(mut sender: SendStream) -> Result<()> {
        sender.write_all(b"Hello World").await?;
        sender.finish().await?;
        Ok(())
    }
}
