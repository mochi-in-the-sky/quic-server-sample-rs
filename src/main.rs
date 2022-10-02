use anyhow::Result;
use tracing::*;

use the_catcher_in_the_lie::config::Config;
use the_catcher_in_the_lie::server::Server;
use the_catcher_in_the_lie::setting::Setting;
use the_catcher_in_the_lie::terminator;

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
