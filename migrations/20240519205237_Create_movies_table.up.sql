-- Add up migration script here
CREATE TABLE movies (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    release_year INT NOT NULL,
    genre VARCHAR(255) NOT NULL,
    poster_url TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);