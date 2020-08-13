FROM docker.pkg.github.com/cardboxdev/backend/builder:1.45.2 as build

ARG CRATE_NAME
ENV USER="root"
WORKDIR /app

COPY ./diesel.toml ./diesel.toml

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./logic ./logic
COPY ./${CRATE_NAME} ./${CRATE_NAME}

RUN cargo test --release --verbose --package cardbox-$CRATE_NAME

RUN cargo build --release --package cardbox-$CRATE_NAME

# ----------------------------------------------------------------

FROM docker.pkg.github.com/cardboxdev/backend/start-tools:1.1

ARG CRATE_NAME

WORKDIR /app

RUN touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/cardbox-$CRATE_NAME ./server

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY ./docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x server

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/server"]
