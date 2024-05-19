use axum::{http::StatusCode, routing::get, Json, Router};
use chrono::{DateTime, Utc};
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
    title: String,
    release_year: i32,
    genre: String,
    poster_url: Option<String>,
    created_at: DateTime<Utc>,
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
    pub title: String,
    pub release_year: i32,
    pub genre: String,
    pub poster_url: Option<String>,
}

pub async fn create_movie(
    state: Extension<Arc<AppState>>,
    Json(movie): Json<CreateMovie>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    tracing::info!("Creating movie...");
    let db = &state.db;
    let movie = sqlx::query_as!(
        Movie,
        "INSERT INTO movies (title, release_year, genre, poster_url) VALUES ($1, $2, $3, $4) RETURNING *",
        movie.title,
        movie.release_year,
        movie.genre,
        movie.poster_url
    )
    .fetch_one(db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "error": e.to_string()}).to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": movie}).to_string(),
    ))
}
