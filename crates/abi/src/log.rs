use std::error::Error;

use tracing_subscriber::{
    filter::{FromEnvError, ParseError},
    fmt,
    prelude::*,
    EnvFilter,
};

use crate::config::LogConfig;

pub fn logger_init(config: &LogConfig) {
    let default_filter = { format!("{},{}", config.level.level(), config.filter) };
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|from_env_error| {
            _ = from_env_error
                .source()
                .and_then(|source| source.downcast_ref::<ParseError>())
                .map(|parse_err| {
                    eprintln!("log failed to parse filter from env: {}", parse_err);
                });

            Ok::<EnvFilter, FromEnvError>(EnvFilter::builder().parse_lossy(&default_filter))
        })
        .unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter_layer)
        .init()
}
