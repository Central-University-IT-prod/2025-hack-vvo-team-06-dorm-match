# Stage 1: Build the Rust binary
FROM rust:1.82 AS builder
WORKDIR /usr/src/app
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./common ./common
COPY ./services/auth ./services/auth
COPY ./services/room-management ./services/room-management
COPY ./common/.sqlx ./common/.sqlx
COPY ./services/auth/.sqlx ./services/auth/.sqlx
COPY ./services/room-management/.sqlx ./services/room-management/.sqlx
RUN cargo build --release --bin dormmatch-room-management

# Stage 2: Create the runtime image
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/dormmatch-room-management /app/dormmatch-room-management
COPY ./migrations /app/migrations
CMD ["/app/dormmatch-room-management"]
