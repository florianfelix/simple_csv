#![allow(unused)]
use stacked_errors::StackableErr;
use std::path::PathBuf;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

pub struct EzLog {}

impl EzLog {
    pub fn init() -> stacked_errors::Result<()> {
        let env_filter = EnvFilter::builder()
            .with_default_directive(tracing::Level::INFO.into())
            .try_from_env()
            .unwrap_or(EnvFilter::new("info"));

        let log_file = std::fs::File::create(PathBuf::from("./atui.log")).stack()?;
        let file_subscriber = tracing_subscriber::fmt::layer()
            // .json()
            .with_file(true)
            .with_line_number(true)
            .with_writer(log_file)
            .with_target(true)
            .with_ansi(false)
            .without_time()
            .with_filter(env_filter);

        // let env_filter = EnvFilter::builder()
        //     .with_default_directive(tracing::Level::INFO.into())
        //     .parse_lossy("info,surrealdb=warn");
        // // .try_from_env()
        // // .unwrap_or(EnvFilter::new("info"));

        // let std_subscriber = tracing_subscriber::fmt::layer()
        //     // .json()
        //     .with_writer(std::io::stdout)
        //     .with_file(true)
        //     .without_time()
        //     .with_filter(env_filter);

        let registry = tracing_subscriber::registry()
            .with(file_subscriber)
            // .with(std_subscriber)
            .with(ErrorLayer::default());

        registry.try_init().stack_err("Failed to init Registry")?;

        Ok(())
    }
}
