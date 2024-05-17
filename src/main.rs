mod config;

use dotenvy::dotenv;
use envy;
use rust_rest_api::api;
use rust_rest_api::config::Config;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load environment variables from .env file
    dotenv().ok();
    let cfg = envy::from_env::<Config>().unwrap();

    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let rust_log_filter =
        tracing::level_filters::LevelFilter::from(rust_log.parse::<tracing::Level>().unwrap());

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(rust_log_filter)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_level(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("Starting application up");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await?;
    tracing::debug!("Connected to database");

    api::serve(cfg, pool).await?;
    Ok(())
}
