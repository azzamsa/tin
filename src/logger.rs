use time::format_description::well_known::Rfc3339;
use tracing_subscriber::{
    filter::{self},
    fmt::{layer, time::OffsetTime},
    prelude::*,
    registry,
};

use crate::config::{Config, Env};

pub fn init(config: &Config) -> Result<(), crate::Error> {
    let log_level = if config.env == Env::Production {
        filter::LevelFilter::INFO
    } else {
        filter::LevelFilter::DEBUG
    };

    let env_filter = filter::EnvFilter::new("")
        .add_directive(log_level.into())
        .add_directive("sqlx::query=error".parse()?)
        .add_directive("hyper=warn".parse()?)
        .add_directive("reqwest=warn".parse()?);

    let utc_offset_hour = config.utc_offset_hour;
    let fmt_layer = layer()
        .with_target(true)
        .with_timer(OffsetTime::new(
            time::UtcOffset::from_hms(utc_offset_hour, 0, 0).unwrap_or(time::UtcOffset::UTC),
            Rfc3339,
        ))
        .with_filter(env_filter);

    registry().with(fmt_layer).init();

    Ok(())
}
