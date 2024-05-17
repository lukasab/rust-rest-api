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

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_level(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("Starting application up");

    let cfg = envy::from_env::<Config>().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await?;
    tracing::info!("Connected to database");

    api::serve(cfg, pool).await?;
    Ok(())
}
