ARG RUST_VERSION=1.53.0

FROM rust:$RUST_VERSION as build

ARG DIESEL_CLI_VERSION=1.4.1

RUN echo "rust-${RUST_VERSION} diesel-$DIESEL_CLI_VERSION"
RUN USER=root cargo install diesel_cli --version ${DIESEL_CLI_VERSION} --no-default-features --features postgres && \
    mkdir -p /out && cp $(which diesel) /out/

LABEL version=$RUST_VERSION-$DIESEL_CLI_VERSION
