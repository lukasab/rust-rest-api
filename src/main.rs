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
    // add health check route
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await?;

    api::serve(cfg, pool).await?;
    Ok(())
}
