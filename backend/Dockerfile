# ─── Build-Stage ────────────────────────────────────────────────
FROM rust:1.86-slim AS builder

WORKDIR /app

# Cargo.toml + Lock zuerst kopieren für besseres Layer-Caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ─── Runtime-Stage ──────────────────────────────────────────────
FROM debian:bookworm-slim

# ca-certificates für HTTPS (reqwest zu adsb.lol)
# libssl3 weil reqwest per default native-tls nutzt
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# WORKDIR = wo die SQLite-Datei landet → hier mounten wir das Volume
WORKDIR /data

COPY --from=builder /app/target/release/squawk7700 /usr/local/bin/squawk7700

CMD ["squawk7700"]
