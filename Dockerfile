FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY rust-toolchain.toml rust-toolchain.toml
COPY src src
COPY locales locales

RUN cargo build --release

FROM debian:bookworm-slim

LABEL org.opencontainers.image.title="rAthenaFR Discord Bot"
LABEL org.opencontainers.image.description="Bot Discord à commandes slash en lecture seule pour bases rAthenaFR."
LABEL org.opencontainers.image.licenses="GPL-3.0-only"
LABEL org.opencontainers.image.version="0.2.4"

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --home /nonexistent --shell /usr/sbin/nologin rathenafr

WORKDIR /app
COPY --from=builder /app/target/release/rathenafr-discord-bot /usr/local/bin/rathenafr-discord-bot

USER 10001:10001

# Le binaire rafraîchit /tmp/rathenafr-health tant que la base répond.
# Le conteneur est marqué "unhealthy" si le fichier devient trop ancien.
HEALTHCHECK --interval=30s --timeout=5s --start-period=60s --retries=3 \
    CMD ["/bin/sh", "-c", "test -f /tmp/rathenafr-health && [ $(( $(date +%s) - $(stat -c %Y /tmp/rathenafr-health) )) -lt 90 ]"]

ENTRYPOINT ["/usr/local/bin/rathenafr-discord-bot"]
