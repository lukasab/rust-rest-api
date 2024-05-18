use axum::{http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::api::AppState;
use axum::Extension;

pub fn router() -> Router {
    Router::new().route("/movies", get(get_movies).post(create_movie))
    //.merge(users::router()) // Add this line to merge the users router
}

#[derive(Serialize)]
struct Movie {
    id: i32,
    name: String,
}

pub async fn get_movies(
    state: Extension<Arc<AppState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    tracing::info!("Requesting movies...");
    let db = &state.db;
    let movies = sqlx::query_as!(Movie, "SELECT * FROM movies")
        .fetch_all(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "error": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": movies}).to_string(),
    ))
}

#[derive(Deserialize)]
pub struct CreateMovie {
    pub id: i32,
    pub name: String,
}

pub async fn create_movie(
    state: Extension<Arc<AppState>>,
    Json(movie): Json<CreateMovie>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    tracing::info!("Creating movie...");
    let db = &state.db;
    let movie = sqlx::query_as!(
        Movie,
        "INSERT INTO movies (id, name) VALUES ($1, $2) RETURNING *",
        movie.id,
        movie.name
    )
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_error) => {
            if db_error.kind() == sqlx::error::ErrorKind::UniqueViolation {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    json!({"success": false, "error": "Movie already exists"}).to_string(),
                );
            } else {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"success": false, "error": "DB error"}).to_string(),
                );
            }
        }
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "error": e.to_string()}).to_string(),
            );
        }
    })?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": movie}).to_string(),
    ))
}
