FROM docker.pkg.github.com/cardbox/backend/builder:rust1.53.0-sqlx0.5.5 as build

ENV USER="root"
ENV SQLX_OFFLINE=true
WORKDIR /app

COPY

COPY ./Cargo.lock ./Cargo.toml ./sqlx-data.json ./
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./settings ./settings
COPY ./api-admin ./api-admin
COPY ./api-public ./api-public
COPY ./api-private ./api-private
COPY ./api-internal ./api-internal
COPY ./core ./core
COPY ./app ./app

ARG API_NAME

RUN cargo test --release --verbose --package cardbox-api-$API_NAME

RUN cargo build --release --package cardbox-api-$API_NAME

# ----------------------------------------------------------------

FROM docker.pkg.github.com/cardbox/backend/start-tools:1.3

ARG API_NAME

WORKDIR /app

RUN touch .env

COPY --from=build /out/sqlx /bin/
COPY --from=build /app/target/release/cardbox-api-$API_NAME ./server

COPY --from=build /app/migrations ./migrations
COPY ./config ./config
COPY ./docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x server

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/server"]
