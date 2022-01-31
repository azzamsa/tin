use env_logger::Builder;
use log::LevelFilter;

use crate::config::{Config, Env};

pub fn init(config: &Config) {
    let log_level = if config.env == Env::Production {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };

    Builder::new()
        .filter_level(log_level)
        .filter_module("sqlx::query", LevelFilter::Error)
        .init();
}
