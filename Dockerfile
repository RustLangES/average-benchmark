# Etapa base con cargo-chef
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Etapa para planificar dependencias
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY api/ ./api/
COPY benchmark/ ./benchmark/
RUN cargo chef prepare --recipe-path recipe.json

# Etapa de construcción compartida
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release -p api
RUN cargo build --release -p benchmark

# Imagen de API
FROM debian:bookworm-slim AS api
WORKDIR /app
COPY --from=builder /app/target/release/api /app/
EXPOSE 8080
ENTRYPOINT ["/app/api"]

# Imagen de Benchmark
FROM debian:bookworm-slim AS benchmark
WORKDIR /app
COPY --from=builder /app/target/release/benchmark /app/
COPY benchmark/PRIVACY.md /app/
ENTRYPOINT ["/app/benchmark"]
