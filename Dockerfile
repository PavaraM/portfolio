# syntax=docker/dockerfile:1.7

# Portfolio site served by Nginx.
# Zero build step — just copy the static files and serve them.

FROM nginx:alpine

LABEL org.opencontainers.image.title="portfolio" \
      org.opencontainers.image.description="DevOps portfolio site served by Nginx" \
      org.opencontainers.image.source="https://github.com/PavaraM/portfolio"

COPY docker/nginx.conf /etc/nginx/conf.d/default.conf
COPY index.html /usr/share/nginx/html/index.html

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget -qO- http://127.0.0.1/ >/dev/null 2>&1 || exit 1
