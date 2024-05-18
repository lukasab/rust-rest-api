use crate::config::Config;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod error;
mod movies;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug)]
pub struct AppState {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> Result<()> {
    let shared_state = Arc::new(AppState {
        config: Arc::new(config),
        db: db.clone(),
    });
    let shared_state_clone = shared_state.clone(); // Clone the shared_state Arc

    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(shared_state_clone)) // Use the cloned shared_state
            .layer(TraceLayer::new_for_http()),
    );

    let addr = format!(
        "{host}:{port}",
        host = &shared_state.config.host,
        port = &shared_state.config.port
    );
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn api_router() -> Router {
    Router::new()
        .route("/healthcheck", get(health_check_handler))
        .merge(movies::router())
}

pub async fn health_check_handler(
    Extension(_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    tracing::info!("Health check request received");
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
