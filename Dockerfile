ARG rust_ver=1.56
FROM rust:${rust_ver}-slim-bullseye as build

ENV SQLX_OFFLINE=true
ARG rustc_mode=release
ARG rustc_opts=--release
ARG API_NAME

RUN apt update && apt upgrade -y && apt install ca-certificates libsodium-dev pkg-config -y && update-ca-certificates && apt clean

RUN rustup component add rustfmt

# Create user and group files, which will be used in a running container to
# run the process as an unprivileged user.
RUN mkdir -p /out/etc/ \
 && echo 'nobody:x:65534:65534:nobody:/:' > /out/etc/passwd \
 && echo 'nobody:x:65534:' > /out/etc/group

COPY Cargo.lock Cargo.toml /app/
COPY db/Cargo.toml /app/db/
COPY settings/Cargo.toml /app/settings/
COPY api-admin/Cargo.toml /app/api-admin/
COPY api-public/Cargo.toml /app/api-public/
COPY api-internal/Cargo.toml /app/api-internal/
COPY api-private/Cargo.toml /app/api-private/
COPY core/Cargo.toml /app/core/
COPY app/Cargo.toml /app/app/

WORKDIR /app

RUN mkdir -p db/src/ && touch db/src/lib.rs \
&& mkdir -p settings/src/ && touch settings/src/lib.rs \
&& mkdir -p api-admin/src/ && touch api-admin/src/lib.rs \
&& mkdir -p api-public/src/ && touch api-public/src/lib.rs \
&& mkdir -p api-internal/src/ && touch api-internal/src/lib.rs \
&& mkdir -p api-private/src/ && touch api-private/src/lib.rs \
&& mkdir -p core/src/ && touch core/src/lib.rs \
&& mkdir -p app/src/ && touch app/src/lib.rs \

ENV SODIUM_USE_PKG_CONFIG=1
# Build dependencies only.
RUN cargo build --lib ${rustc_opts}
# Remove fingreprints of pre-built empty project sub-crates
# to rebuild them correctly later.
RUN rm -rf /app/target/${rustc_mode}/.fingerprint/cardbox*

COPY db/ /app/db/
COPY settings/ /app/settings/
COPY api-admin/ /app/api-admin/
COPY api-public/ /app/api-public/
COPY api-internal/ /app/api-internal/
COPY api-private/ /app/api-private/
COPY core/ /app/core/
COPY app/ /app/app/
COPY sqlx-data.json /app/

RUN cargo build --package cardbox-api-$API_NAME ${rustc_opts}

RUN cp /app/target/${rustc_mode}/cardbox-api-$API_NAME /out/cardbox-api-$API_NAME
COPY config /out/config
COPY .env.sample /out/.env

# ----------------------------------------------------------------

FROM debian:bullseye-slim AS runtime

ARG API_NAME

COPY --from=build /out/ /

RUN apt update && apt upgrade -y && apt install ca-certificates libsodium23 -y && update-ca-certificates && apt clean

USER nobody:nobody

ENV API_NAME=${API_NAME}

ENTRYPOINT ["sh", "-c", "/cardbox-api-${API_NAME}"]