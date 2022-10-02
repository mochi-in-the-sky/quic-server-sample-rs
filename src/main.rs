use anyhow::Result;
use tracing::*;

use throwsterhouse_five::config::Config;
use throwsterhouse_five::server::Server;
use throwsterhouse_five::setting::Setting;
use throwsterhouse_five::terminator;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    info!("lift off");

    let terminator = terminator::new();
    let config = Config::new()?;
    let setting = Setting::new(config).unwrap();

    let server = Server::new(setting);
    let server = server.listen();

    tokio::select! {
        _ = terminator => info!("terminator is comming"),
        _ = server => info!("mission completed")
    }

    info!("I'll be back");
    Ok(())
}
