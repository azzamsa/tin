use std::env;

use fern::colors::{Color, ColoredLevelConfig};

pub fn init() {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Cyan);

    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".into());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let builder = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}][{target}][{level}] {message}",
                date = chrono::Local::now().format(r#"%F %H:%M:%S %:z"#),
                target = record.target(),
                level = colors.color(record.level()),
                message = message
            ));
        })
        .level(log_level)
        .chain(std::io::stderr());

    // globally apply logger
    builder.apply().expect("Applying logger failed");
}
