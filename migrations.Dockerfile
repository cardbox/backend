ARG rust_ver=1.54
FROM rust:${rust_ver}-slim

COPY migrations /app/migrations
COPY ./docker-entrypoint.sh /app/entrypoint.sh

WORKDIR /app
RUN cargo install sqlx-cli --no-default-features --features postgres
RUN chmod +x entrypoint.sh

RUN seq 1 8 | xargs -I{} mkdir -p /usr/share/man/man{} && \
    apt update && \
    apt -y install libpq-dev postgresql-client ca-certificates && \
    update-ca-certificates && \
    apt clean

ENTRYPOINT ["/app/entrypoint.sh"]
LABEL version=0.1