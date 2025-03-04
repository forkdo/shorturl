FROM rust:bookworm AS builder

ARG ENV_PATH=url_shortener.db

WORKDIR /build

COPY . .

RUN apt update && \
    apt install -y sqlite3

RUN cp .env url-shortener && \
    cd url-shortener && \
    sqlite3 "$ENV_PATH" < migrations/schema.sql && \
    cargo install --path .

FROM gcr.io/distroless/cc-debian12
LABEL maintainer="Jetsung Chan<i@jetsung.com"

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/url-shortener /usr/local/bin/shorturl
COPY --from=builder /build/.env /app/
COPY --from=builder /build/url-shortener/*.db /app/
COPY --from=builder /build/url-shortener/migrations /app/migrations

EXPOSE 3000

ENTRYPOINT ["shorturl"]