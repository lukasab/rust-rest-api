# Use the official Rust image as the base image
FROM rust:1.78.0 AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock /

# Copy the source code
COPY src /src

# Build the dependencies only
RUN cargo build --release

ENV HOST=0.0.0.0
ENV PORT=3000
ENV DEBUG=FALSE
ENV SECRET_KEY=your_secret_key
ENV DATABASE_URL="postgres://postgres:password@db:5432/mydatabase"
ENV RUST_LOG=DEBUG

EXPOSE 3000

CMD ["cargo", "run", "--release"]

