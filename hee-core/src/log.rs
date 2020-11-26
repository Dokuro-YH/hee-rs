use tracing_subscriber::fmt::time;

pub use tracing::*;

pub fn init(level: crate::config::LogLevel) {
    tracing_subscriber::fmt()
        .with_max_level(level.to_level_filter())
        .with_timer(time::ChronoLocal::with_format("%FT%T%.3f%:z".to_string()))
        .init();
}
