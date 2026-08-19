# syntax=docker/dockerfile:1.7

# ── Stage 1: Build the WASM bundle with Trunk ──
FROM rust:1.87-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates && rm -rf /var/lib/apt/lists/*

# Install prebuilt Trunk (avoids cargo install lightningcss issues)
ARG TRUNK_VERSION=v0.21.14
ARG TARGETARCH
RUN case "$TARGETARCH" in \
      amd64) TRUNK_ARCH="x86_64" ;; \
      arm64) TRUNK_ARCH="aarch64" ;; \
      *) echo "Unsupported arch: $TARGETARCH" && exit 1 ;; \
    esac && \
    curl -fsSL "https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-${TRUNK_ARCH}-unknown-linux-gnu.tar.gz" \
    | tar xz -C /usr/local/bin && \
    trunk --version

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch && rm -rf src

COPY src/ src/
COPY index.html style.css Trunk.toml ./

RUN trunk build --release

# ── Stage 2: Serve with Nginx ──
FROM nginx:alpine

LABEL org.opencontainers.image.title="portfolio" \
      org.opencontainers.image.description="DevOps portfolio site served by Nginx (Leptos/WASM)" \
      org.opencontainers.image.source="https://github.com/PavaraM/portfolio"

COPY docker/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget -qO- http://127.0.0.1/ >/dev/null 2>&1 || exit 1
