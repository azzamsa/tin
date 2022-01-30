use std::env;

use fern::colors::{Color, ColoredLevelConfig};

pub fn init() {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Cyan);

    let log_level = env::var("GENERAL_LOG_LEVEL").unwrap_or_else(|_| "INFO".into());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let app_log_level = env::var("APP_LOG_LEVEL").unwrap_or_else(|_| "TRACE".into());
    let app_log_level = app_log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Trace);

    let builder = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}][{target}][{level}] {message}",
                date = chrono::Local::now().format(r#"%F %H:%M:%S %:z"#),
                target = record.target(),
                level = colors.color(record.level()),
                message = message
            ));
        }) // It is bit tricky to set log level for each dependencies.
        // So we set the `LOG_LEVEL` to "INFO" so that each dependencies consume the
        // env and use "info" as their log level.
        // But afterward, we are overriding our own module to our custom value
        .level(log_level)
        // - and per-module overrides
        .level_for("nahla", app_log_level)
        .chain(std::io::stderr());

    // globally apply logger
    builder.apply().expect("Applying logger failed");
}
