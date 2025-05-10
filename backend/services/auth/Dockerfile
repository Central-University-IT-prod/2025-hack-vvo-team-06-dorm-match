# Stage 1: Build the Rust binary
FROM rust:1.82 AS builder
WORKDIR /usr/src/app
# Install sqlx-cli
RUN cargo install sqlx-cli
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./common ./common
COPY ./services/auth ./services/auth
COPY ./services/room-management ./services/room-management
COPY ./migrations ./migrations
# Run sqlx prepare (requires PostgreSQL at build time)
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
RUN cargo sqlx prepare --workspace
RUN cargo build --release --bin dormmatch-auth

# Stage 2: Create the runtime image
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/dormmatch-auth /app/dormmatch-auth
COPY --from=builder /usr/src/app/migrations /app/migrations
# Install sqlx-cli for migrations
RUN apt-get update && apt-get install -y curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $HOME/.cargo/env && \
    cargo install sqlx-cli --no-default-features --features postgres && \
    apt-get remove -y curl && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*
# Copy entrypoint script
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh
ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/dormmatch-auth"]
