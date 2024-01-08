use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum::serve;
use tin::{config::Config, logger, route::app};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), tin::Error> {
    let config = Arc::new(Config::load()?);
    logger::init(&config)?;

    let app = app().await?;

    let host: IpAddr = config.base_url.parse()?;
    let port = config.http.port;
    let address = &SocketAddr::new(host, port);

    tracing::info!("App started at `{}`", address);
    serve(
        TcpListener::bind(address).await.unwrap(),
        app.into_make_service(),
    )
    .await?;

    Ok(())
}
