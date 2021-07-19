ARG RUST_VERSION=1.53.0

FROM rust:$RUST_VERSION as build

ARG SQLX_CLI_VERSION=0.5.5

RUN echo "rust-${RUST_VERSION} sqlx-$SQLX_CLI_VERSION"
RUN USER=root cargo install sqlx-cli --version ${SQLX_CLI_VERSION} --no-default-features --features postgres && \
    mkdir -p /out && cp $(which sqlx) /out/

LABEL version=rust$RUST_VERSION-sqlx$SQLX_CLI_VERSION
