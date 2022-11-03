use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{Config, Env};

pub fn init(config: &Config) {
    let log_level = if config.env == Env::Production {
        "info"
    } else {
        "debug"
    };

    tracing_subscriber::registry()
        .with(EnvFilter::new(log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
