use std::sync::Arc;

use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

use crate::api::AppState;

pub fn router() -> Router {
    Router::new().route("/movies", get(get_movies).post(create_movie))
}

#[derive(Serialize, ToSchema)]
pub struct Movie {
    id: i32,
    #[schema(example = "The Matrix")]
    title: String,
    #[schema(example = 2021)]
    release_year: i32,
    #[schema(example = "Action")]
    genre: String,
    #[schema(value_type = Url, example = "https://example.com/poster.jpg")]
    poster_url: Option<String>,
    #[schema(example = "2021-08-01T12:00:00Z")]
    created_at: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/movies",
    tag = "Movies",
    responses(
        (status = StatusCode::OK, description = "Return all movies", body = [Movie]),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Option<String>)
    )
)]
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
                json!(e.to_string()).to_string(),
            )
        })?;

    Ok((StatusCode::OK, json!(movies).to_string()))
}

#[derive(Deserialize, ToSchema)]
pub struct CreateMovie {
    #[schema(example = "The Matrix")]
    pub title: String,
    #[schema(example = 2021)]
    pub release_year: i32,
    #[schema(example = "Action")]
    pub genre: String,
    #[schema(value_type = Url, example = "https://example.com/poster.jpg")]
    pub poster_url: Option<String>,
}

#[utoipa::path(
    post,
    path = "/movies",
    tag = "Movies",
    request_body = CreateMovie,
    responses(
        (status = StatusCode::CREATED, description = "Sucessufully created new movie", body = Movie),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Option<String>)
    )
)]
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
            json!(e.to_string()).to_string(),
        )
    })?;

    Ok((StatusCode::CREATED, json!(movie).to_string()))
}
