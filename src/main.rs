use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum::Server;
use tin::{config::Config, logger, routes::app};

#[tokio::main]
async fn main() -> Result<(), tin::Error> {
    let config = Arc::new(Config::load()?);
    logger::init(&config);

    let app = app().await?;

    let host: IpAddr = config.base_url.parse()?;
    let port = config.http.port;
    let address = &SocketAddr::new(host, port);

    log::info!("App started at `{}`", address);
    Server::bind(address).serve(app.into_make_service()).await?;

    Ok(())
}
