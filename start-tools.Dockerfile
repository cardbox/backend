FROM debian:buster-slim

RUN seq 1 8 | xargs -I{} mkdir -p /usr/share/man/man{} && \
    apt update && \
    apt -y install libpq-dev postgresql-client ca-certificates && \
    update-ca-certificates && \
    apt clean

LABEL version=1.3
