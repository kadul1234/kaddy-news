# ── Stage 1: Build ────────────────────────────────────────────────────────
FROM rust:1.85-slim AS builder

WORKDIR /app

# Cache dependencies before copying source
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Build the real binary
COPY src ./src
COPY static ./static
RUN touch src/main.rs && cargo build --release

# ── Stage 2: Runtime ──────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/news_app .

# The app writes news_articles.txt here at runtime
RUN touch news_articles.txt

EXPOSE 3000
CMD ["./news_app"]
