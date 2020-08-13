FROM docker.pkg.github.com/cardboxdev/backend/builder:1.45.2 as build

ARG API_NAME
ENV USER="root"
WORKDIR /app

COPY ./diesel.toml ./diesel.toml

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./logic ./logic
COPY ./api-admin ./api-internal ./api-private ./api-public ./

RUN cargo test --release --verbose --package cardbox-api-$API_NAME

RUN cargo build --release --package cardbox-api-$API_NAME

# ----------------------------------------------------------------

FROM docker.pkg.github.com/cardboxdev/backend/start-tools:1.1

ARG API_NAME

WORKDIR /app

RUN touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/cardbox-api-$API_NAME ./server

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY ./docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x server

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/server"]
